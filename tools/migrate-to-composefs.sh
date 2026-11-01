#!/usr/bin/env bash
# migrate-to-composefs.sh — In-place migration from ostree to composefs for ArkheOS/DMail nodes.
#
# Usage:
#   sudo ./migrate-to-composefs.sh --target-image ghcr.io/arkhe-os/dmail-arkheos:stable
#
# The script NEVER executes `bootc-migrate commit`. Commit is an operator decision
# after confirming the system boots correctly with composefs.

set -euo pipefail

TARGET_IMAGE=""
DRY_RUN=false
FORCE=false

usage() {
    cat <<EOF
Usage: $0 --target-image <image> [--dry-run] [--force]

  --target-image   Destination OCI image (required)
  --dry-run        Only checks preconditions, does not migrate
  --force          Ignores disk space check (dangerous)
EOF
    exit 1
}

# --- Parse arguments ---
while [[ $# -gt 0 ]]; do
    case "$1" in
        --target-image) TARGET_IMAGE="$2"; shift 2 ;;
        --dry-run) DRY_RUN=true; shift ;;
        --force) FORCE=true; shift ;;
        -h|--help) usage ;;
        *) echo "Unknown argument: $1" >&2; usage ;;
    esac
done

if [[ -z "$TARGET_IMAGE" ]]; then
    echo "ERROR: --target-image is required" >&2
    usage
fi

# --- Precondition checks ---

echo "==> Checking bootc-migrate..."
if ! command -v bootc-migrate &>/dev/null; then
    echo "ERROR: bootc-migrate not found. Install from https://github.com/tuna-os/bootc-migrate" >&2
    exit 1
fi

echo "==> Checking current backend..."
CURRENT_BACKEND=$(bootc status --json 2>/dev/null | jq -r '.status.backend // "unknown"')
if [[ "$CURRENT_BACKEND" != "ostree" ]]; then
    echo "ERROR: current backend is '$CURRENT_BACKEND', expected 'ostree'" >&2
    echo "This script only migrates ostree -> composefs." >&2
    exit 1
fi

echo "==> Checking disk space..."
# /var will be copied; we need free space >= size of /var
VAR_SIZE_KB=$(du -sk /var 2>/dev/null | awk '{print $1}')
FREE_KB=$(df -k /var | awk 'NR==2 {print $4}')
if [[ "$FREE_KB" -lt "$VAR_SIZE_KB" ]] && [[ "$FORCE" != "true" ]]; then
    echo "ERROR: insufficient space in /var." >&2
    echo "  Required: $((VAR_SIZE_KB / 1024)) MiB" >&2
    echo "  Available: $((FREE_KB / 1024)) MiB" >&2
    echo "Use --force to ignore (not recommended)." >&2
    exit 1
fi

echo "==> Confirming backup..."
read -r -p "Did you backup /home, /var and /etc? [y/N] " CONFIRM
if [[ "$CONFIRM" != "y" ]]; then
    echo "Migration aborted. Backup first." >&2
    exit 1
fi

# --- Dry-run ---

echo "==> Executing dry-run (no changes will be made)..."
sudo bootc-migrate \
    --target-image "$TARGET_IMAGE" \
    --dry-run

if [[ "$DRY_RUN" == "true" ]]; then
    echo "Dry-run complete. No changes made."
    exit 0
fi

# --- Migration ---

echo "==> Starting migration (~5–25 min)..."
sudo bootc-migrate \
    --target-image "$TARGET_IMAGE"

echo ""
echo "==> Migration complete. The system was NOT rebooted."
echo ""
echo "NEXT STEPS:"
echo "  1. Reboot: sudo systemctl reboot"
echo "  2. In the boot menu, the composefs entry is default."
echo "     The old OSTree entry remains as a fallback."
echo "  3. After confirming the system boots correctly,"
echo "     make the migration permanent (one-way):"
echo "       sudo bootc-migrate commit"
echo ""
echo "WARNING: After \`commit\`, the OSTree fallback is removed."
echo "Do not execute \`commit\` without confirming the boot."
