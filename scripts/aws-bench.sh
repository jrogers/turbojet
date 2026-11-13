#!/usr/bin/env bash
# The benchmark on AWS Linux: the `ack` acceptor on one machine and the `load` initiator on
# another, in one cluster placement group, so orders and their acknowledgements cross a real
# network; the criterion benchmarks, measured or profiled, on the second. Without the network run,
# only the second is launched.
#   scripts/aws-bench.sh         # launch, build, run, fetch the results, terminate
#   scripts/aws-bench.sh clean   # delete what an earlier run left, should one have died
# Needs the aws CLI logged in (aws login, or aws configure sso), jq, ssh, git and python3. Builds HEAD as committed,
# or each of REVS.
#
# Runs, each one's output into target/aws-bench/<run>/:
#   network   `load` against `ack` at each of LOADS, a fresh `ack` for each
#   local     the criterion benchmarks (`cargo bench -p turbojet --all-features`) on the initiator's
#             machine, their temporary files on its local NVMe, once for each of MALLOC; criterion's
#             reports into criterion-<malloc>/, its output into local-<malloc>.txt
#   profile   each of those bench targets under samply, for PROFILE_SECS a benchmark (criterion's
#             --profile-time, which skips its analysis), into profile-<target>.json.gz beside its
#             .syms.json: `samply load` one. Built with frame pointers and line tables, as every
#             run is when this one is asked for, so their numbers aren't for the guide's tables.
# results.md has what it ran on, then every run's output.
#
# What it costs: two instances while it runs, ~$0.40/hour each for c6id.2xlarge on demand (check
# AWS's pricing), and a public IPv4 address each. Everything it makes is tagged
# turbojet-bench=<run> and deleted on any exit; should this machine stop first, each instance
# shuts itself down after MAX_MINUTES, which terminates it, and `clean` deletes the rest.
#
#   AWS_PROFILE  the aws CLI's profile, as for any aws command (AWS_PROFILE=development ...)
#   REGION       us-east-1           TYPE         c6id.2xlarge (needs local NVMe: a `d` type; x86 or Graviton)
#   AZ           the zone (us-east-1b); by default each that offers TYPE, until one has room
#   MAX_MINUTES  180
#   RUNS         network local (the default), profile, or any of them
#   LOADS        `load`'s sessions:rate pairs, space-separated (1:1000 1:10000 1:100000 8:200000)
#   DURATION     10, seconds `load` measures each, after its 2 s of warm-up
#   BENCHES      the local run's bench targets (codec session roundtrip sessions fixp fast); all by default
#   BENCH_ARGS   criterion's arguments for the local and profile runs, e.g. '--quick' or 'roundtrip'
#   REVS         commits the local run measures in turn on the one machine, HEAD by default: an A/B
#                (REVS='main pad main pad'), a column each in results.md, numbered in order, since a
#                commit may be listed more than once to interleave rounds. The network and profile
#                runs use the first.
#   PROFILE_SECS 5, seconds the profile run gives each benchmark
#   MALLOC       glibc, jemalloc (LD_PRELOAD, Ubuntu's libjemalloc2), jemalloc-bg (the same, purging
#                freed pages on a thread of its own: MALLOC_CONF=background_thread:true), or several,
#                space-separated: the local and network runs run once for each on the same machines,
#                the local run's medians a column each; the profile run uses the first
#   RUSTFLAGS    the build's. On Graviton by default -C target-feature=+lse,+crc (atomics and CRC
#                inline, not called); on x86 none. RUSTFLAGS= builds without.
#   NIC_COALESCE_OFF  1: both machines' NICs without interrupt moderation (ethtool -C, adaptive-rx
#                off, rx-usecs 0, tx-usecs 0; ENA's defaults are adaptive, 20 and 64 µs), the
#                settings before and after in nic.txt. In turbofan's runs on c9gd this took a
#                round trip from 130 µs to 71 µs.
#   DRIVERS      the network run's drivers, tokio (the default), spin (`ack --spin` and `load --spin`:
#                each session on a thread of its own, polling its socket), or both: a run for each
#   BUSY_POLL    µs the kernel busy-polls the NIC for a socket with nothing to read
#                (net.core.busy_poll for epoll, as tokio waits; net.core.busy_read for a read, as
#                the spinning driver does), on both machines; several, space-separated, a run for
#                each. 0 (the default) is the kernel's: no busy polling.
#   PIN          1: on both machines the interrupts on CPU 0 (irqbalance stopped); the threads that
#                spin, `load`'s sender and each spin-N, on a CPU of their own from the last down,
#                in the upper half (wrapping round within it should there be more); every other
#                thread on the lower half from CPU 1. Where each went is in the results.
#   ISOLCPUS     1: the upper half of the CPUs kept from the scheduler and from timer ticks
#                (isolcpus, nohz_full, rcu_nocbs on the kernel command line, which takes a
#                reboot), so with PIN=1 the spinning threads have their cores to themselves
set -euo pipefail
cd "$(dirname "$0")/.."

export AWS_REGION=${REGION:-us-east-1}
TYPE=${TYPE:-c6id.2xlarge}
MAX_MINUTES=${MAX_MINUTES:-180}
RUNS=${RUNS:-network local}
LOADS=${LOADS:-1:1000 1:10000 1:100000 8:200000}
DURATION=${DURATION:-10}
BENCHES=${BENCHES:-}
BENCH_ARGS=${BENCH_ARGS:-}
PROFILE_SECS=${PROFILE_SECS:-5}
MALLOC=${MALLOC:-glibc}
SHAS=()
for rev in ${REVS:-HEAD}; do SHAS+=("$(git rev-parse --short "$rev^{commit}")"); done
for malloc in $MALLOC; do
    [[ $malloc == glibc || $malloc == jemalloc || $malloc == jemalloc-bg ]] ||
        { echo "MALLOC is glibc, jemalloc or jemalloc-bg, not $malloc" >&2; exit 2; }
done
# The environment that puts an allocator under a command, expanded on the machine running it.
preload() {
    case $1 in
        jemalloc) echo 'LD_PRELOAD=/usr/lib/$(uname -m)-linux-gnu/libjemalloc.so.2' ;;
        jemalloc-bg) echo 'LD_PRELOAD=/usr/lib/$(uname -m)-linux-gnu/libjemalloc.so.2 MALLOC_CONF=background_thread:true' ;;
    esac
}
FIRST_MALLOC=${MALLOC%% *}
SAMPLY_VERSION=0.13.1
NIC_COALESCE_OFF=${NIC_COALESCE_OFF:-0}
[[ $NIC_COALESCE_OFF == 0 || $NIC_COALESCE_OFF == 1 ]] || { echo "NIC_COALESCE_OFF is 0 or 1" >&2; exit 2; }
DRIVERS=${DRIVERS:-tokio}
for driver in $DRIVERS; do
    [[ $driver == tokio || $driver == spin ]] || { echo "DRIVERS are tokio or spin, not $driver" >&2; exit 2; }
done
BUSY_POLL=${BUSY_POLL:-0}
for us in $BUSY_POLL; do [[ $us =~ ^[0-9]+$ ]] || { echo "BUSY_POLL is in µs, not $us" >&2; exit 2; }; done
PIN=${PIN:-0} ISOLCPUS=${ISOLCPUS:-0}
[[ $PIN == 0 || $PIN == 1 ]] || { echo "PIN is 0 or 1" >&2; exit 2; }
[[ $ISOLCPUS == 0 || $ISOLCPUS == 1 ]] || { echo "ISOLCPUS is 0 or 1" >&2; exit 2; }
TAG=turbojet-bench
PORT=9876

log() { printf '%s %s\n' "$(date +%H:%M:%S)" "$*" >&2; }

# Everything tagged by any run: instances first, which the rest can't be deleted under.
clean() {
    local filter="Name=tag-key,Values=$TAG"
    local ids
    ids=$(aws ec2 describe-instances --filters "$filter" \
        "Name=instance-state-name,Values=pending,running,stopping,stopped" \
        --query 'Reservations[].Instances[].InstanceId' --output text)
    if [[ -n $ids ]]; then
        log "terminating $ids"
        # shellcheck disable=SC2086 # one ID a word
        aws ec2 terminate-instances --instance-ids $ids >/dev/null
        # shellcheck disable=SC2086
        aws ec2 wait instance-terminated --instance-ids $ids
    fi
    for sg in $(aws ec2 describe-security-groups --filters "$filter" --query 'SecurityGroups[].GroupId' --output text); do
        log "deleting security group $sg"
        aws ec2 delete-security-group --group-id "$sg" >/dev/null
    done
    for pg in $(aws ec2 describe-placement-groups --filters "$filter" --query 'PlacementGroups[].GroupName' --output text); do
        log "deleting placement group $pg"
        aws ec2 delete-placement-group --group-name "$pg"
    done
    for key in $(aws ec2 describe-key-pairs --filters "$filter" --query 'KeyPairs[].KeyName' --output text); do
        log "deleting key pair $key"
        aws ec2 delete-key-pair --key-name "$key"
    done
}

if [[ ${1:-} == clean ]]; then
    clean
    exit 0
elif [[ $# -gt 0 ]]; then
    sed -n '2,6p' "$0" >&2
    exit 2
fi

for run in $RUNS; do
    [[ $run == network || $run == local || $run == profile ]] || { echo "no run $run" >&2; exit 2; }
done
asked() { [[ " $RUNS " == *" $1 "* ]]; }
COUNT=1
asked network && COUNT=2
for load in $LOADS; do
    [[ $load =~ ^[0-9]+:[0-9]+$ ]] || { echo "LOADS are sessions:rate pairs, not $load" >&2; exit 2; }
done
for tool in aws jq ssh git python3; do
    command -v "$tool" >/dev/null || { echo "needs $tool" >&2; exit 1; }
done
aws sts get-caller-identity >/dev/null || { echo "the aws CLI isn't logged in: aws login (or aws configure sso)" >&2; exit 1; }
if [[ -n $(git status --porcelain --untracked-files=no) ]]; then
    log "warning: uncommitted changes aren't benchmarked; it builds HEAD, $(git rev-parse --short HEAD)"
fi

RUN=$TAG-$(date +%Y%m%d-%H%M%S)
OUT=target/aws-bench/$RUN
mkdir -p "$OUT"
KEY=$OUT/key

# From here, whatever happens, what this run made is deleted.
SG='' PG='' IDS=''
cleanup() {
    local status=$?
    trap - EXIT INT TERM
    log "cleaning up $RUN"
    if [[ -n $IDS ]]; then
        # shellcheck disable=SC2086
        aws ec2 terminate-instances --instance-ids $IDS >/dev/null || true
        # shellcheck disable=SC2086
        aws ec2 wait instance-terminated --instance-ids $IDS || true
    fi
    [[ -n $SG ]] && { aws ec2 delete-security-group --group-id "$SG" >/dev/null || true; }
    [[ -n $PG ]] && { aws ec2 delete-placement-group --group-name "$PG" || true; }
    aws ec2 delete-key-pair --key-name "$RUN" >/dev/null 2>&1 || true
    rm -f "$KEY"
    left=$(aws ec2 describe-instances --filters "Name=tag:$TAG,Values=$RUN" \
        "Name=instance-state-name,Values=pending,running,stopping,stopped" \
        --query 'Reservations[].Instances[].InstanceId' --output text || echo unknown)
    if [[ -n $left ]]; then
        log "WARNING: instances may still be running ($left): run scripts/aws-bench.sh clean"
    fi
    exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT TERM

# The zones that have the type, AZ alone if set; each tried in turn should one be out of capacity.
ZONES=${AZ:-$(aws ec2 describe-instance-type-offerings --location-type availability-zone \
    --filters "Name=instance-type,Values=$TYPE" --query 'InstanceTypeOfferings[].Location' --output text)}
[[ -n $ZONES ]] || { echo "$TYPE isn't offered in $AWS_REGION" >&2; exit 1; }
VPC=$(aws ec2 describe-vpcs --filters Name=is-default,Values=true --query 'Vpcs[0].VpcId' --output text)
# Ubuntu for the type's architecture: x86 or Graviton.
ARCH=$(aws ec2 describe-instance-types --instance-types "$TYPE" \
    --query 'InstanceTypes[0].ProcessorInfo.SupportedArchitectures[0]' --output text)
[[ $ARCH == x86_64 ]] && ARCH=amd64
AMI=resolve:ssm:/aws/service/canonical/ubuntu/server/24.04/stable/current/$ARCH/hvm/ebs-gp3/ami-id
[[ $ARCH == arm64 ]] && RUSTFLAGS=${RUSTFLAGS--C target-feature=+lse,+crc}
RUSTFLAGS=${RUSTFLAGS:-}
# With the profile run, stacks samply can walk and name, for every run: cargo would rebuild otherwise.
asked profile && RUSTFLAGS="-C force-frame-pointers=yes${RUSTFLAGS:+ $RUSTFLAGS}"
MY_IP=$(curl -fsS https://checkip.amazonaws.com)
log "$RUN: $COUNT × $TYPE, SSH from $MY_IP"

aws ec2 create-key-pair --key-name "$RUN" --key-type ed25519 \
    --tag-specifications "ResourceType=key-pair,Tags=[{Key=$TAG,Value=$RUN}]" \
    --query KeyMaterial --output text >"$KEY"
chmod 400 "$KEY"
SG=$(aws ec2 create-security-group --group-name "$RUN" --description "$RUN" --vpc-id "$VPC" \
    --tag-specifications "ResourceType=security-group,Tags=[{Key=$TAG,Value=$RUN}]" --query GroupId --output text)
aws ec2 authorize-security-group-ingress --group-id "$SG" --protocol tcp --port 22 --cidr "$MY_IP/32" >/dev/null
# The two machines reach each other on any port.
aws ec2 authorize-security-group-ingress --group-id "$SG" --protocol -1 --source-group "$SG" >/dev/null
aws ec2 create-placement-group --group-name "$RUN" --strategy cluster \
    --tag-specifications "ResourceType=placement-group,Tags=[{Key=$TAG,Value=$RUN}]" >/dev/null
PG=$RUN

# One request, as a cluster placement group wants, in the first zone with room. A shutdown
# terminates, and each shuts itself down after MAX_MINUTES.
for AZ in $ZONES; do
    SUBNET=$(aws ec2 describe-subnets --filters "Name=availability-zone,Values=$AZ" "Name=default-for-az,Values=true" \
        --query 'Subnets[0].SubnetId' --output text)
    if IDS=$(aws ec2 run-instances --count $COUNT --instance-type "$TYPE" --image-id "$AMI" --key-name "$RUN" \
        --subnet-id "$SUBNET" --security-group-ids "$SG" --placement "GroupName=$PG" \
        --associate-public-ip-address --instance-initiated-shutdown-behavior terminate \
        --metadata-options HttpTokens=required \
        --block-device-mappings 'DeviceName=/dev/sda1,Ebs={VolumeSize=40,VolumeType=gp3,DeleteOnTermination=true}' \
        --tag-specifications "ResourceType=instance,Tags=[{Key=$TAG,Value=$RUN}]" "ResourceType=volume,Tags=[{Key=$TAG,Value=$RUN}]" \
        --user-data "$(printf '#!/bin/sh\nshutdown -h +%s\n' "$MAX_MINUTES")" \
        --query 'Instances[].InstanceId' --output text 2>"$OUT/launch.err"); then
        break
    fi
    grep -q InsufficientInstanceCapacity "$OUT/launch.err" || { cat "$OUT/launch.err" >&2; exit 1; }
    log "no $TYPE capacity in $AZ"
done
[[ -n $IDS ]] || { echo "no zone has capacity for $COUNT × $TYPE" >&2; exit 1; }
log "launched $IDS in $AZ"
# shellcheck disable=SC2086
aws ec2 wait instance-running --instance-ids $IDS
# shellcheck disable=SC2086
read -r -a PUBLIC <<<"$(aws ec2 describe-instances --instance-ids $IDS \
    --query 'Reservations[].Instances[].PublicIpAddress' --output text)"
# shellcheck disable=SC2086
read -r -a PRIVATE <<<"$(aws ec2 describe-instances --instance-ids $IDS \
    --query 'Reservations[].Instances[].PrivateIpAddress' --output text)"
# The last the initiator's, which builds and runs the benchmarks; with two, the first the acceptor's.
ACCEPTOR=${PUBLIC[0]} INITIATOR=${PUBLIC[COUNT - 1]}

SSH=(ssh -i "$KEY" -o StrictHostKeyChecking=accept-new -o UserKnownHostsFile="$OUT/known_hosts"
    -o ConnectTimeout=5 -o ConnectionAttempts=6 -o ServerAliveInterval=15 -o BatchMode=yes)
on() { local host=$1; shift; "${SSH[@]}" "ubuntu@$host" "$@"; }
# Up once cloud-init is done and a second connection works too: until then sshd may restart, and a
# first connection's success doesn't last. Even then a later one timed out once, so every
# connection tries six times (ConnectionAttempts).
for host in "${PUBLIC[@]}"; do
    for _ in $(seq 60); do
        on "$host" 'cloud-init status --wait >/dev/null; true' 2>/dev/null && on "$host" true 2>/dev/null && break
        sleep 5
    done
    on "$host" true
done
log "SSH up"

# The upper half of the CPUs isolated, which takes a reboot: before the NIC's settings and the instance store's
# mount, which a reboot undoes. The shutdown user data set runs on the first boot only, so again.
if [[ $ISOLCPUS == 1 ]]; then
    for host in "${PUBLIC[@]}"; do
        on "$host" 'set -e
            cpus=$(($(nproc) / 2))-$(($(nproc) - 1))
            echo "GRUB_CMDLINE_LINUX_DEFAULT=\"\$GRUB_CMDLINE_LINUX_DEFAULT isolcpus=nohz,domain,$cpus nohz_full=$cpus rcu_nocbs=$cpus\"" |
                sudo tee /etc/default/grub.d/99-turbojet-isolcpus.cfg >/dev/null
            sudo update-grub 2>/dev/null
            (sleep 1; sudo reboot) >/dev/null 2>&1 &'
    done
    sleep 20
    for host in "${PUBLIC[@]}"; do
        for _ in $(seq 60); do on "$host" true 2>/dev/null && break; sleep 5; done
        on "$host" "grep -q isolcpus /proc/cmdline && sudo shutdown -h +$MAX_MINUTES 2>/dev/null"
    done
    log "rebooted with CPUs isolated"
fi

# Each machine's NIC interrupt moderation, recorded, and with NIC_COALESCE_OFF turned off: the
# default route's, its one NIC.
for i in "${!PUBLIC[@]}"; do
    on "${PUBLIC[i]}" "set -e
        dev=\$(ip -o route get 1.1.1.1 | awk '{for (i = 1; i < NF; i++) if (\$i == \"dev\") print \$(i + 1)}')
        echo \"machine $i: \$dev, \$(ethtool -i \$dev | awk '/^driver/ {print \$2}')\"
        sudo ethtool -c \$dev | grep -E 'Adaptive|rx-usecs:|tx-usecs:'
        if [ $NIC_COALESCE_OFF = 1 ]; then
            sudo ethtool -C \$dev adaptive-rx off 2>/dev/null || true
            sudo ethtool -C \$dev rx-usecs 0 tx-usecs 0
            echo after:
            sudo ethtool -c \$dev | grep -E 'Adaptive|rx-usecs:|tx-usecs:'
        fi"
done >"$OUT/nic.txt"
# Interrupts on CPU 0, the default route's NIC's among them, and irqbalance stopped so that they
# stay there. An interrupt that can't move (a per-CPU timer's) keeps its place.
if [[ $PIN == 1 ]]; then
    for host in "${PUBLIC[@]}"; do
        on "$host" 'sudo systemctl stop irqbalance 2>/dev/null || true
            for irq in /proc/irq/[0-9]*; do echo 0 | sudo tee $irq/smp_affinity_list >/dev/null 2>&1 || true; done'
    done
fi
if [[ " $MALLOC " == *' jemalloc'* ]]; then
    for host in "${PUBLIC[@]}"; do
        on "$host" 'sudo apt-get -qq update && sudo NEEDRESTART_SUSPEND=1 DEBIAN_FRONTEND=noninteractive apt-get -qq install -y libjemalloc2 >/dev/null
            test -f /usr/lib/$(uname -m)-linux-gnu/libjemalloc.so.2'
    done
fi

# The initiator's machine builds both examples, and the benchmarks for the local and profile runs.
# Each commit in a directory of its own, src the first's.
UNIQUE=$(printf '%s\n' "${SHAS[@]}" | awk '!seen[$0]++')
for sha in $UNIQUE; do git archive --format=tar "$sha" | on "$INITIATOR" "mkdir -p src-$sha && tar -x -C src-$sha"; done
on "$INITIATOR" "ln -sfn src-${SHAS[0]} src"
log "building" $UNIQUE
TARGETS=''
for bench in $BENCHES; do TARGETS+=" --bench $bench"; done
CARGO_BENCH="cargo bench -p turbojet --all-features$TARGETS"
ENV=${RUSTFLAGS:+"export RUSTFLAGS='$RUSTFLAGS' &&"}
asked profile && ENV+=" export CARGO_PROFILE_BENCH_DEBUG=line-tables-only CARGO_PROFILE_RELEASE_DEBUG=line-tables-only &&"
on "$INITIATOR" "set -e
    sudo apt-get -qq update && sudo NEEDRESTART_SUSPEND=1 DEBIAN_FRONTEND=noninteractive apt-get -qq install -y build-essential >/dev/null
    curl -fsS https://sh.rustup.rs | sh -s -- -y -q --profile minimal >/dev/null
    . ~/.cargo/env"
if asked network; then
    on "$INITIATOR" ". ~/.cargo/env && cd src && $ENV cargo build -q --release -p turbojet --example ack --example load"
    # The acceptor's binary to its machine, by its private address.
    scp -q -i "$KEY" -o UserKnownHostsFile="$OUT/known_hosts" "$KEY" "ubuntu@$INITIATOR:.ssh/id_ed25519"
    on "$INITIATOR" "scp -q -o StrictHostKeyChecking=accept-new src/target/release/examples/ack ubuntu@${PRIVATE[0]}:ack"
fi
if asked local || asked profile; then
    # The instance store on /data, for the benchmarks' temporary files.
    on "$INITIATOR" 'set -e
        dev=$(lsblk -dno NAME,MODEL | awk "/Instance Storage/ {print \"/dev/\" \$1; exit}")
        [ -n "$dev" ] || { echo "no instance store: a d type is needed" >&2; exit 1; }
        sudo mkfs.ext4 -q -F "$dev" && sudo mkdir -p /data && sudo mount "$dev" /data
        sudo chown ubuntu /data && mkdir -p /data/tmp'
    for sha in $UNIQUE; do on "$INITIATOR" ". ~/.cargo/env && cd src-$sha && $ENV $CARGO_BENCH --no-run -q"; done
fi
if asked profile; then
    log "building samply $SAMPLY_VERSION"
    # Every process sampled, the kernel's stacks named, without root.
    on "$INITIATOR" ". ~/.cargo/env && cargo install -q --locked samply@$SAMPLY_VERSION
        sudo sysctl -q kernel.perf_event_paranoid=-1 kernel.kptr_restrict=0"
fi

# With PIN=1, the shell that starts process `$2` on the lower half of the CPUs from 1, and in the
# background, once `$1` threads of it that spin (spin-N and send) are running, puts each on a CPU
# of its own from the last down, recording where every thread went in pins.txt.
pinned() {
    [[ $PIN == 1 ]] || return 0
    echo "half=\$((\$(nproc) / 2)); taskset -pc 1-\$((half - 1)) \$\$ >/dev/null
        (for _ in \$(seq 200); do
            pid=\$(pgrep -x $2) && [ \$(grep -cE '^(spin-|send)' /proc/\$pid/task/*/comm | grep -c ':1') -ge $1 ] && break
            sleep 0.05
        done
        cpu=0
        for task in /proc/\$pid/task/*; do
            grep -qE '^(spin-|send)' \$task/comm || continue
            taskset -pc \$((\$(nproc) - 1 - cpu % half)) \${task##*/} >/dev/null; cpu=\$((cpu + 1))
        done
        for task in /proc/\$pid/task/*; do echo \"$2 \$(cat \$task/comm): \$(taskset -pc \${task##*/} | sed 's/.*: //')\"; done >pins.txt) &"
}

network() {
    local malloc=$1 driver=$2 us=$3 load sessions rate spin='' spinning=0
    for host in "${PUBLIC[@]}"; do on "$host" "sudo sysctl -q net.core.busy_poll=$us net.core.busy_read=$us"; done
    [[ $driver == spin ]] && spin=--spin
    for load in $LOADS; do
        sessions=${load%:*} rate=${load#*:}
        [[ $driver == spin ]] && spinning=$sessions
        on "$ACCEPTOR" "$(pinned "$spinning" ack) $(preload "$malloc") nohup ./ack --listen 0.0.0.0:$PORT $spin >ack.log 2>&1 </dev/null &
            for _ in \$(seq 50); do ss -Hltn sport = :$PORT | grep -q . && exit 0; sleep 0.1; done
            cat ack.log >&2; exit 1"
        log "run network ($malloc, $driver, busy poll $us µs): $sessions sessions, $rate orders/s"
        echo "### $malloc, $driver driver, busy poll $us µs: $sessions sessions, $rate orders/s"
        echo
        echo '```'
        on "$INITIATOR" "$(pinned $((spinning + 1)) load) $(preload "$malloc") src/target/release/examples/load ${PRIVATE[0]}:$PORT \
            --sessions $sessions --rate $rate --duration $DURATION $spin"
        if [[ $PIN == 1 ]]; then
            on "$INITIATOR" 'cat pins.txt'
            on "$ACCEPTOR" 'cat pins.txt'
        fi
        echo '```'
        echo
        on "$ACCEPTOR" 'pkill -TERM -x ack || true; while pgrep -x ack >/dev/null; do sleep 0.1; done'
    done
}

# Each bench target under samply, run as cargo runs it, in its package's directory. Its
# executable is the one cargo names: `Executable benches/codec.rs (target/release/deps/codec-...)`.
profile() {
    on "$INITIATOR" ". ~/.cargo/env && cd src && $ENV $CARGO_BENCH --no-run 2>&1" |
        sed -n 's|.*Executable benches/\([a-z_]*\)\.rs (\(.*\))|\1 \2|p' >"$OUT/targets.txt"
    [[ -s $OUT/targets.txt ]] || { echo "cargo named no bench executables" >&2; return 1; }
    local name path
    while read -r name path; do
        log "run profile: $name"
        on "$INITIATOR" ". ~/.cargo/env && cd src/crates/turbojet && $ENV TMPDIR=/data/tmp $(preload "$FIRST_MALLOC") \
            ~/.cargo/bin/samply record --save-only --unstable-presymbolicate -o ~/profile-$name.json.gz -- \
            ../../$path --bench --profile-time $PROFILE_SECS $BENCH_ARGS" </dev/null >"$OUT/profile-$name.txt" 2>&1
    done <"$OUT/targets.txt"
    on "$INITIATOR" 'tar -c profile-*' | tar -x -C "$OUT"
}

NAMES=()
for run in $RUNS; do
    case $run in
        profile) profile ;;
        network)
            for malloc in $MALLOC; do
                for driver in $DRIVERS; do for us in $BUSY_POLL; do network "$malloc" "$driver" "$us"; done; done
            done | tee "$OUT/network.md"
            ;;
        local)
            for i in "${!SHAS[@]}"; do
                sha=${SHAS[i]}
                for malloc in $MALLOC; do
                    name=$malloc
                    [[ ${#SHAS[@]} -gt 1 ]] && name=$((i + 1))-$sha-$malloc
                    NAMES+=("$name")
                    log "run local ($name): $CARGO_BENCH -- $BENCH_ARGS"
                    # Each run's reports apart: criterion would compare one with another's.
                    on "$INITIATOR" ". ~/.cargo/env && cd src-$sha && rm -rf target/criterion && $ENV TMPDIR=/data/tmp \
                        $(preload "$malloc") $CARGO_BENCH -- $BENCH_ARGS" | tee "$OUT/local-$name.txt"
                    on "$INITIATOR" "mv src-$sha/target/criterion criterion-$name && tar -c criterion-$name && rm -rf criterion-$name" |
                        tar -x -C "$OUT"
                done
            done
            ;;
    esac
done

# What it ran on.
{
    echo "# $RUN"
    echo
    echo "turbojet ${SHAS[*]}, $COUNT × $TYPE in $AZ ($AWS_REGION), one cluster placement group."
    [[ -n $RUSTFLAGS ]] && echo && echo "RUSTFLAGS=$RUSTFLAGS"
    [[ $NIC_COALESCE_OFF == 1 ]] && echo && echo "NIC interrupt moderation off (NIC_COALESCE_OFF=1)."
    echo && echo "Allocators: $MALLOC."
    [[ $PIN == 1 ]] && echo && echo "Interrupts on CPU 0; the spinning threads on a CPU each in the upper half, the rest in the lower half from 1 (PIN=1)."
    [[ $ISOLCPUS == 1 ]] && echo && echo "The upper half of the CPUs isolated: isolcpus, nohz_full, rcu_nocbs (ISOLCPUS=1)."
    if asked profile; then
        echo
        echo "Built for the profile run, with frame pointers and line tables: not numbers for the guide."
    fi
    on "$INITIATOR" 'echo; lscpu | grep -E "Model name|^CPU\(s\)"; echo "cache line: $(getconf LEVEL1_DCACHE_LINESIZE) bytes"
        uname -sr; . ~/.cargo/env && rustc --version'
    for run in $RUNS; do
        echo
        echo "## $run"
        echo
        if [[ $run == network ]]; then
            echo "\`load\` against \`ack\`, $DURATION s measured after 2 s of warm-up."
            echo
            cat "$OUT/network.md"
        elif [[ $run == profile ]]; then
            echo "Each bench target under samply, $PROFILE_SECS s a benchmark: \`samply load <file>\`."
            echo
            for profile in "$OUT"/profile-*.json.gz; do echo "- $(basename "$profile")"; done
        else
            echo "\`$CARGO_BENCH -- $BENCH_ARGS\`: criterion's medians, a median and a rate for each run (the whole output in local-<run>.txt)."
            echo
            # Each benchmark's median from criterion's reports, with its throughput as a rate.
            # shellcheck disable=SC2046 # one directory a word
            python3 -I - $(for name in "${NAMES[@]}"; do echo "$OUT/criterion-$name"; done) <<'EOF'
import json, sys
from pathlib import Path

def medians(directory):
    out = {}
    for estimates in sorted(Path(directory).glob("**/new/estimates.json")):
        bench = json.loads((estimates.parent / "benchmark.json").read_text())
        ns = json.loads(estimates.read_text())["median"]["point_estimate"]
        throughput = bench.get("throughput") or {}
        rate = ""
        if "Elements" in throughput:
            rate = f"{throughput['Elements'] / ns * 1e9:,.0f}/s"
        elif "Bytes" in throughput:
            rate = f"{throughput['Bytes'] / ns * 1e9 / 2**20:,.0f} MiB/s"
        unit, scale = next((u, s) for u, s in (("s", 1e9), ("ms", 1e6), ("µs", 1e3), ("ns", 1)) if ns >= s or u == "ns")
        out[bench["full_id"]] = f"{ns / scale:.3g} {unit} | {rate}"
    return out

columns = [medians(d) for d in sys.argv[1:]]
names = [Path(d).name.removeprefix("criterion-") for d in sys.argv[1:]]
print("| Benchmark | " + " | ".join(f"{n} median | {n} rate" for n in names) + " |")
print("|---|" + "---:|---:|" * len(names))
for bench in sorted(set().union(*columns)):
    print(f"| {bench} | " + " | ".join(c.get(bench, " | ") for c in columns) + " |")
EOF
        fi
    done
} >"$OUT/results.md"
log "results in $OUT/results.md"
