#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo-sync-readme &> /dev/null
then
    echo "'cargo sync-readme' could not be found - install it with 'cargo install cargo-sync-readme"
    exit
fi

pushd gcloud-ctx > /dev/null
echo -n "Syncing gcloud-ctx... "
cargo sync-readme
echo "Done"
popd > /dev/null

pushd gctx > /dev/null
echo -n "Syncing gctx... "
cargo sync-readme
echo "Done"
popd > /dev/null
