#!/usr/bin/env bash
# The benchmark on AWS Linux: the `ack` acceptor on one machine and the `load` initiator on
# another, in one cluster placement group, so orders and their acknowledgements cross a real network.
#   scripts/aws-bench.sh         # launch, build, run, fetch the results, terminate
#   scripts/aws-bench.sh clean   # delete what an earlier run left, should one have died
# Needs the aws CLI logged in (aws login, or aws configure sso), jq, ssh and git. Builds HEAD as committed.
#
# Runs, each one's output into target/aws-bench/<run>/:
#   network   `load` against `ack` at each of LOADS, a fresh `ack` for each
#   local     the criterion benchmarks (`cargo bench -p turbojet --all-features`) on the initiator's
#             machine, their temporary files on its local NVMe; criterion's reports into criterion/
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
#   RUNS         network local, or one of them
#   LOADS        `load`'s sessions:rate pairs, space-separated (1:1000 1:10000 1:100000 8:200000)
#   DURATION     10, seconds `load` measures each, after its 2 s of warm-up
#   BENCHES      the local run's bench targets (codec session roundtrip sessions fixp fast); all by default
#   BENCH_ARGS   criterion's arguments for the local run, e.g. '--quick' or 'roundtrip'
#   RUSTFLAGS    the build's. On Graviton by default -C target-feature=+lse,+crc (atomics and CRC
#                inline, not called); on x86 none. RUSTFLAGS= builds without.
#   NIC_COALESCE_OFF  1: both machines' NICs without interrupt moderation (ethtool -C, adaptive-rx
#                off, rx-usecs 0, tx-usecs 0; ENA's defaults are adaptive, 20 and 64 µs), the
#                settings before and after in nic.txt. In turbofan's runs on c9gd this took a
#                round trip from 130 µs to 71 µs.
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
NIC_COALESCE_OFF=${NIC_COALESCE_OFF:-0}
[[ $NIC_COALESCE_OFF == 0 || $NIC_COALESCE_OFF == 1 ]] || { echo "NIC_COALESCE_OFF is 0 or 1" >&2; exit 2; }
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
    [[ $run == network || $run == local ]] || { echo "no run $run" >&2; exit 2; }
done
for load in $LOADS; do
    [[ $load =~ ^[0-9]+:[0-9]+$ ]] || { echo "LOADS are sessions:rate pairs, not $load" >&2; exit 2; }
done
for tool in aws jq ssh git; do
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
MY_IP=$(curl -fsS https://checkip.amazonaws.com)
log "$RUN: 2 × $TYPE, SSH from $MY_IP"

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
    if IDS=$(aws ec2 run-instances --count 2 --instance-type "$TYPE" --image-id "$AMI" --key-name "$RUN" \
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
[[ -n $IDS ]] || { echo "no zone has capacity for 2 × $TYPE" >&2; exit 1; }
log "launched $IDS in $AZ"
# shellcheck disable=SC2086
aws ec2 wait instance-running --instance-ids $IDS
# shellcheck disable=SC2086
read -r -a PUBLIC <<<"$(aws ec2 describe-instances --instance-ids $IDS \
    --query 'Reservations[].Instances[].PublicIpAddress' --output text)"
# shellcheck disable=SC2086
read -r -a PRIVATE <<<"$(aws ec2 describe-instances --instance-ids $IDS \
    --query 'Reservations[].Instances[].PrivateIpAddress' --output text)"
# The first the acceptor's, the second the initiator's, which builds.
ACCEPTOR=${PUBLIC[0]} INITIATOR=${PUBLIC[1]}

SSH=(ssh -i "$KEY" -o StrictHostKeyChecking=accept-new -o UserKnownHostsFile="$OUT/known_hosts"
    -o ConnectTimeout=5 -o ServerAliveInterval=15 -o BatchMode=yes)
on() { local host=$1; shift; "${SSH[@]}" "ubuntu@$host" "$@"; }
# Up once cloud-init is done and a second connection works too: until then sshd may restart, and a
# first connection's success doesn't last.
for host in "${PUBLIC[@]}"; do
    for _ in $(seq 60); do
        on "$host" 'cloud-init status --wait >/dev/null; true' 2>/dev/null && on "$host" true 2>/dev/null && break
        sleep 5
    done
    on "$host" true
done
log "SSH up"

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

# The initiator's machine builds both examples, and the benchmarks for the local run.
git archive --format=tar HEAD | on "$INITIATOR" 'mkdir -p src && tar -x -C src'
log "building $(git rev-parse --short HEAD)"
TARGETS=''
for bench in $BENCHES; do TARGETS+=" --bench $bench"; done
CARGO_BENCH="cargo bench -p turbojet --all-features$TARGETS"
ENV=${RUSTFLAGS:+"export RUSTFLAGS='$RUSTFLAGS' &&"}
on "$INITIATOR" "set -e
    sudo apt-get -qq update && sudo NEEDRESTART_SUSPEND=1 DEBIAN_FRONTEND=noninteractive apt-get -qq install -y build-essential >/dev/null
    curl -fsS https://sh.rustup.rs | sh -s -- -y -q --profile minimal >/dev/null
    . ~/.cargo/env && cd src && $ENV cargo build -q --release -p turbojet --example ack --example load"
if [[ " $RUNS " == *' local '* ]]; then
    # The instance store on /data, for the benchmarks' temporary files.
    on "$INITIATOR" 'set -e
        dev=$(lsblk -dno NAME,MODEL | awk "/Instance Storage/ {print \"/dev/\" \$1; exit}")
        [ -n "$dev" ] || { echo "no instance store: a d type is needed" >&2; exit 1; }
        sudo mkfs.ext4 -q -F "$dev" && sudo mkdir -p /data && sudo mount "$dev" /data
        sudo chown ubuntu /data && mkdir -p /data/tmp'
    on "$INITIATOR" ". ~/.cargo/env && cd src && $ENV $CARGO_BENCH --no-run -q"
fi
# The acceptor's binary to its machine, by its private address.
scp -q -i "$KEY" -o UserKnownHostsFile="$OUT/known_hosts" "$KEY" "ubuntu@$INITIATOR:.ssh/id_ed25519"
on "$INITIATOR" "scp -q -o StrictHostKeyChecking=accept-new src/target/release/examples/ack ubuntu@${PRIVATE[0]}:ack"

network() {
    local load sessions rate
    for load in $LOADS; do
        sessions=${load%:*} rate=${load#*:}
        on "$ACCEPTOR" "nohup ./ack --listen 0.0.0.0:$PORT >ack.log 2>&1 </dev/null &
            for _ in \$(seq 50); do ss -Hltn sport = :$PORT | grep -q . && exit 0; sleep 0.1; done
            cat ack.log >&2; exit 1"
        log "run network: $sessions sessions, $rate orders/s"
        echo "### $sessions sessions, $rate orders/s"
        echo
        echo '```'
        on "$INITIATOR" "src/target/release/examples/load ${PRIVATE[0]}:$PORT \
            --sessions $sessions --rate $rate --duration $DURATION"
        echo '```'
        echo
        on "$ACCEPTOR" 'pkill -TERM -x ack || true; while pgrep -x ack >/dev/null; do sleep 0.1; done'
    done
}

for run in $RUNS; do
    case $run in
        network) network | tee "$OUT/network.md" ;;
        local)
            log "run local: $CARGO_BENCH -- $BENCH_ARGS"
            on "$INITIATOR" ". ~/.cargo/env && cd src && $ENV TMPDIR=/data/tmp \
                $CARGO_BENCH -- $BENCH_ARGS" | tee "$OUT/local.txt"
            on "$INITIATOR" 'tar -c -C src/target criterion' | tar -x -C "$OUT"
            ;;
    esac
done

# What it ran on.
{
    echo "# $RUN"
    echo
    echo "turbojet $(git rev-parse --short HEAD), 2 × $TYPE in $AZ ($AWS_REGION), one cluster placement group."
    [[ -n $RUSTFLAGS ]] && echo && echo "RUSTFLAGS=$RUSTFLAGS"
    [[ $NIC_COALESCE_OFF == 1 ]] && echo && echo "NIC interrupt moderation off (NIC_COALESCE_OFF=1)."
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
        else
            echo "\`$CARGO_BENCH -- $BENCH_ARGS\`"
            echo
            echo '```'
            cat "$OUT/local.txt"
            echo '```'
        fi
    done
} >"$OUT/results.md"
log "results in $OUT/results.md"
