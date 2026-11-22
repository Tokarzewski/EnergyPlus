#!/usr/bin/env bash
# Builds this unit's shim with the shared plain-number script.
exec bash "$(dirname "$0")/../build_plain_shim.sh" "$(dirname "$0")"
