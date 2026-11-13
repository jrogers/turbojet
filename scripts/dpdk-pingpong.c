// A UDP ping-pong over DPDK, for issue #46: the network's round trip without the kernel, to set
// against sockperf's over it. One message at a time, both ends polling their one port.
// `scripts/aws-bench.sh` with RUNS=dpdk builds and runs it:
//
//   dpdk-pingpong EAL-ARGS -- server
//   dpdk-pingpong EAL-ARGS -- client PEER-MAC OWN-IP PEER-IP SECONDS SIZE
//
// The server answers each UDP datagram to port 9000 with the same datagram, addresses swapped,
// until killed. The client sends SIZE-byte datagrams for 1 s of warm-up and then SECONDS
// measured, each when the last is answered, and prints the round trips' percentiles.

#include <arpa/inet.h>
#include <inttypes.h>
#include <netinet/in.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <rte_cycles.h>
#include <rte_eal.h>
#include <rte_ethdev.h>
#include <rte_ether.h>
#include <rte_ip.h>
#include <rte_mbuf.h>
#include <rte_udp.h>

#define PORT 9000
#define BURST 32
#define RING 1024

static uint16_t port_id;
static struct rte_ether_addr own_mac;

static void die(const char *what) {
    fprintf(stderr, "dpdk-pingpong: %s\n", what);
    exit(1);
}

static struct rte_mempool *start_port(void) {
    if (rte_eth_dev_count_avail() != 1) die("needs exactly one port");
    RTE_ETH_FOREACH_DEV(port_id) break;
    struct rte_mempool *pool = rte_pktmbuf_pool_create("pool", 8191, 256, 0, RTE_MBUF_DEFAULT_BUF_SIZE,
                                                       rte_eth_dev_socket_id(port_id));
    if (pool == NULL) die("no mbuf pool");
    struct rte_eth_conf conf;
    memset(&conf, 0, sizeof conf);
    if (rte_eth_dev_configure(port_id, 1, 1, &conf) < 0) die("configure failed");
    int socket = rte_eth_dev_socket_id(port_id);
    if (rte_eth_rx_queue_setup(port_id, 0, RING, socket, NULL, pool) < 0) die("rx queue failed");
    if (rte_eth_tx_queue_setup(port_id, 0, RING, socket, NULL) < 0) die("tx queue failed");
    if (rte_eth_dev_start(port_id) < 0) die("start failed");
    rte_eth_macaddr_get(port_id, &own_mac);
    struct rte_eth_link link;
    for (int i = 0; i < 100; i++) {
        rte_eth_link_get_nowait(port_id, &link);
        if (link.link_status == RTE_ETH_LINK_UP) break;
        rte_delay_ms(100);
    }
    char mac[RTE_ETHER_ADDR_FMT_SIZE];
    rte_ether_format_addr(mac, sizeof mac, &own_mac);
    fprintf(stderr, "port %u: %s, link %s\n", port_id, mac, link.link_status ? "up" : "down");
    return pool;
}

// The datagram's UDP header if it's IPv4 UDP to PORT, else NULL.
static struct rte_udp_hdr *ours(struct rte_mbuf *m) {
    struct rte_ether_hdr *eth = rte_pktmbuf_mtod(m, struct rte_ether_hdr *);
    if (eth->ether_type != rte_cpu_to_be_16(RTE_ETHER_TYPE_IPV4)) return NULL;
    struct rte_ipv4_hdr *ip = (struct rte_ipv4_hdr *)(eth + 1);
    if (ip->next_proto_id != IPPROTO_UDP) return NULL;
    struct rte_udp_hdr *udp = (struct rte_udp_hdr *)((char *)ip + rte_ipv4_hdr_len(ip));
    return udp->dst_port == rte_cpu_to_be_16(PORT) ? udp : NULL;
}

static void server(void) {
    struct rte_mbuf *bufs[BURST];
    for (;;) {
        uint16_t n = rte_eth_rx_burst(port_id, 0, bufs, BURST), out = 0;
        for (uint16_t i = 0; i < n; i++) {
            if (ours(bufs[i]) == NULL) {
                rte_pktmbuf_free(bufs[i]);
                continue;
            }
            struct rte_ether_hdr *eth = rte_pktmbuf_mtod(bufs[i], struct rte_ether_hdr *);
            struct rte_ipv4_hdr *ip = (struct rte_ipv4_hdr *)(eth + 1);
            // Swapping the addresses leaves the IP checksum as it was.
            rte_ether_addr_copy(&eth->src_addr, &eth->dst_addr);
            rte_ether_addr_copy(&own_mac, &eth->src_addr);
            uint32_t src = ip->src_addr;
            ip->src_addr = ip->dst_addr;
            ip->dst_addr = src;
            bufs[out++] = bufs[i];
        }
        uint16_t sent = 0;
        while (sent < out) sent += rte_eth_tx_burst(port_id, 0, bufs + sent, out - sent);
    }
}

static int compare(const void *a, const void *b) {
    uint64_t x = *(const uint64_t *)a, y = *(const uint64_t *)b;
    return (x > y) - (x < y);
}

static void client(struct rte_mempool *pool, char **argv) {
    struct rte_ether_addr peer;
    if (rte_ether_unformat_addr(argv[0], &peer) < 0) die("bad PEER-MAC");
    struct in_addr own_ip, peer_ip;
    if (inet_pton(AF_INET, argv[1], &own_ip) != 1 || inet_pton(AF_INET, argv[2], &peer_ip) != 1) die("bad IP");
    uint64_t hz = rte_get_timer_hz(), seconds = strtoull(argv[3], NULL, 10);
    uint16_t size = (uint16_t)atoi(argv[4]);
    if (size < sizeof(uint64_t) || size > 1400) die("SIZE is 8 to 1400");

    size_t cap = 1 << 20, count = 0;
    uint64_t *samples = malloc(cap * sizeof *samples), lost = 0, seq = 0;
    if (samples == NULL) die("no memory");
    struct rte_mbuf *bufs[BURST];
    uint64_t begin = rte_get_timer_cycles(), measure = begin + hz, end = measure + seconds * hz;
    for (uint64_t now = begin; now < end; seq++) {
        struct rte_mbuf *m = rte_pktmbuf_alloc(pool);
        if (m == NULL) die("out of mbufs");
        uint16_t udp_len = sizeof(struct rte_udp_hdr) + size, ip_len = sizeof(struct rte_ipv4_hdr) + udp_len;
        struct rte_ether_hdr *eth = (struct rte_ether_hdr *)rte_pktmbuf_append(m, sizeof *eth + ip_len);
        rte_ether_addr_copy(&peer, &eth->dst_addr);
        rte_ether_addr_copy(&own_mac, &eth->src_addr);
        eth->ether_type = rte_cpu_to_be_16(RTE_ETHER_TYPE_IPV4);
        struct rte_ipv4_hdr *ip = (struct rte_ipv4_hdr *)(eth + 1);
        memset(ip, 0, sizeof *ip);
        ip->version_ihl = RTE_IPV4_VHL_DEF;
        ip->total_length = rte_cpu_to_be_16(ip_len);
        ip->time_to_live = 64;
        ip->next_proto_id = IPPROTO_UDP;
        ip->src_addr = own_ip.s_addr;
        ip->dst_addr = peer_ip.s_addr;
        ip->hdr_checksum = rte_ipv4_cksum(ip);
        struct rte_udp_hdr *udp = (struct rte_udp_hdr *)(ip + 1);
        udp->src_port = udp->dst_port = rte_cpu_to_be_16(PORT);
        udp->dgram_len = rte_cpu_to_be_16(udp_len);
        udp->dgram_cksum = 0; // none, which IPv4 allows
        memset(udp + 1, 0, size);
        memcpy(udp + 1, &seq, sizeof seq);

        uint64_t sent_at = rte_get_timer_cycles();
        while (rte_eth_tx_burst(port_id, 0, &m, 1) == 0) {}
        int answered = 0;
        while (!answered) {
            now = rte_get_timer_cycles();
            if (now - sent_at > hz / 100) break; // 10 ms: lost
            uint16_t n = rte_eth_rx_burst(port_id, 0, bufs, BURST);
            for (uint16_t i = 0; i < n; i++) {
                struct rte_udp_hdr *reply = ours(bufs[i]);
                uint64_t got;
                if (reply != NULL) {
                    memcpy(&got, reply + 1, sizeof got);
                    answered |= got == seq;
                }
                rte_pktmbuf_free(bufs[i]);
            }
        }
        now = rte_get_timer_cycles();
        if (!answered) {
            lost++;
        } else if (sent_at >= measure) {
            if (count == cap) {
                cap *= 2;
                samples = realloc(samples, cap * sizeof *samples);
                if (samples == NULL) die("no memory");
            }
            samples[count++] = now - sent_at;
        }
    }
    if (count == 0) die("no answers");
    qsort(samples, count, sizeof *samples, compare);
    printf("%zu round trips of %u-byte UDP datagrams in %" PRIu64 " s, %" PRIu64 " lost; timer %" PRIu64 " Hz\n",
           count, size, seconds, lost, hz);
    const char *names[] = {"min", "p50", "p90", "p99", "p99.9", "p99.99", "max"};
    const double quantiles[] = {0, 0.5, 0.9, 0.99, 0.999, 0.9999, 1};
    for (int i = 0; i < 7; i++) {
        // Nearest rank, as `load` reports.
        size_t rank = (size_t)(quantiles[i] * (double)count + 0.999999);
        if (rank < 1) rank = 1;
        if (rank > count) rank = count;
        printf("  %-7s%.1f µs\n", names[i], (double)samples[rank - 1] * 1e6 / (double)hz);
    }
}

int main(int argc, char **argv) {
    int eal = rte_eal_init(argc, argv);
    if (eal < 0) die("EAL init failed");
    argc -= eal;
    argv += eal;
    struct rte_mempool *pool = start_port();
    if (argc == 2 && strcmp(argv[1], "server") == 0) {
        server();
    } else if (argc == 7 && strcmp(argv[1], "client") == 0) {
        client(pool, argv + 2);
    } else {
        die("usage: dpdk-pingpong EAL-ARGS -- server | client PEER-MAC OWN-IP PEER-IP SECONDS SIZE");
    }
    rte_eth_dev_stop(port_id);
    rte_eal_cleanup();
    return 0;
}
