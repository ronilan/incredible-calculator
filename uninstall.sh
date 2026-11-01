#!/usr/bin/env bash
#
# uninstall.sh — remove the `calculator` command that install.sh put on the
# PATH. Designed to be run standalone — download (or curl | bash) and run.
#
# Usage:
#   bash uninstall.sh                 # removes `calculator`
#   bash uninstall.sh <command-name>  # remove a differently named install
#
# Requires: sudo.

set -euo pipefail

# The command installed by install.sh. Override it if the app was installed
# under another name.
COMMAND_NAME="${1:-calculator}"

DEST="/usr/local/bin"

target="${DEST}/${COMMAND_NAME}"
if [ ! -f "$target" ]; then
  echo "Nothing to do: $target does not exist."
  exit 0
fi

sudo rm -f "$target"
echo "Removed: $target"
