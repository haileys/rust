#!/bin/bash -e
cd "$(dirname "$0")"
exec cargo check --target=../../i386-pc-win9x-lld.json -Zbuild-std=core,alloc
