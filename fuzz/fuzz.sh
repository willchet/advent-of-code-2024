#!/bin/bash

help () {
    echo "Usage: fuzz.sh <fuzz_target> [OPTIONS] [ADDITIONAL_OPTIONS...] "
    echo
    echo "Options:"
    echo "  --clean       Clean the build artifacts. No <fuzz_target> required."
    echo "  --cov         Show coverage analysis in addition to performing fuzz testing."
    echo "  -h, --help    Show this help message and exit. No <fuzz_target> required."
    echo
    echo "Additional Options:"
    echo "  [MORE_OPTIONS...]"
    echo "                Additional options to be passed to LibFuzzer."
    echo
    echo "Arguments:"
    echo "  <fuzz_target> The target for fuzzing. Required except when only --help or --clean are specified."
    echo
    echo "Examples:"
    echo "  fuzz.sh --clean"
    echo "  fuzz.sh my_fuzz_target --cov --runs=10000"
    echo "  fuzz.sh my_fuzz_target --clean --runs=1000"
}

display_error () {
    echo "Invalid Usage. See help below:"
    help
    exit 1
}

clean () {
    rm -rf artifacts
    rm -rf corpus
    rm -rf coverage
    rm -f default_*
    rm -f crash-*
    rm -f output.html
    echo "Outputs cleaned"
}

# Get arguments
if [ $# -eq 0 ]; then
    display_error
fi

if [ "$1" = "--clean" ]; then
    clean
    shift
fi
if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    help
    shift
fi

if [ $# -eq 0 ]; then
    exit 0
fi

trg="$1"
shift
cov=false
args=()
while [[ $# -gt 0 ]]; do
    case $1 in
        --cov)
            cov=true
            shift
            ;;
        --clean)
            clean
            shift
            ;;
        *)
            args+=("$1")
            shift
            ;;
    esac
done

# Kill all descendants (background processes) when this bash script exits
trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT

# for the coverage info
rustup component add llvm-tools

# for the demangler
cargo install rustfilt

# Make sure all binaries have instrumentation
export RUSTFLAGS="-C instrument-coverage"

# Display message
echo "Fuzzer will begin momentarily. TYPE Q TO STOP THE FUZZER."
sleep 1

# Run the the fuzzer
cargo fuzz run "$trg" -- "${args[@]}" &
fuzz_pid=$!

# Monitor progress
while true; do
    read -t .1 -r -n1 input
    if [ "$input" == "Q" ]; then
        # cargo fuzz run spawns a child process which needs to be killed, rather than killing fuzz_pid itself
        pkill -P $fuzz_pid
        wait $fuzz_pid
        break
    fi

    if ! ps -p $fuzz_pid > /dev/null; then
        wait $fuzz_pid
        if [ $? -eq 0 ]; then
            break
        else
            exit 0
        fi
    fi
done

if $cov; then
    # Get the coverage info
    cargo fuzz coverage "$trg"

    # Directories
    cvg="coverage/$trg/coverage.profdata"
    cov="/home/zcs0/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-cov"
    trg="target/x86_64-unknown-linux-gnu/coverage/x86_64-unknown-linux-gnu/release/$trg"
    chr="/mnt/c/Program Files/Google/Chrome/Application/chrome.exe"

    # Get coverage
    $cov show --Xdemangler=rustfilt --instr-profile=$cvg $trg --ignore-filename-regex=crates --format=html > output.html
    "$chr" "file://wsl.localhost\Ubuntu$(pwd)\output.html"
fi