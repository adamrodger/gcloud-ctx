#!/bin/bash
# copied and adapted from: https://raw.githubusercontent.com/sharkdp/fd/master/doc/screencast.sh
#
# Can be executed on Windows in Docker from Git Bash with:
#     docker run --rm -it -v /$(pwd):/app rust
#     apt update && apt install -y asciinema nodejs npm pv
#     npm install -g svg-term-cli
#     cargo install gctx
#     cd /app
#     svg-term --command="bash doc/screencast.sh" --out doc/screencast.svg --padding=10 --height 25 --width 100 --window true
set -e
set -u

PROMPT="▶"

enter() {
    INPUT=$1
    DELAY=0.6

    prompt
    sleep "$DELAY"
    type "$INPUT"
    sleep 0.3
    printf '%b' "\\n"
    eval "$INPUT"
    type "\\n"
}

prompt() {
    printf '%b ' $PROMPT | pv -q
}

type() {
    printf '%b' "$1" | pv -qL $((15+(-2 + RANDOM%5)))
}

setup() {
    temp_dir=$(mktemp -d -q)
    cd "$temp_dir"
    mkdir configurations
    touch configurations/config_default
    echo "default" > active_config
    cd - > /dev/null
    export CLOUDSDK_CONFIG="$temp_dir"
}

main() {
    IFS='%'

    setup

    enter "gctx list"
    enter "gctx create test --project my-project --account a.user@example.org --zone europe-west1-d"
    enter "gctx activate test"
    enter "gctx list"
    enter "gctx copy test test-copy --activate"
    enter "gctx delete test"
    enter "gctx list"

    prompt

    sleep 3
    echo ""

    unset IFS
}

main

