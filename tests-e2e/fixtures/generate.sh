#!/usr/bin/env bash
# Regenerate the WAV fixture used by the e2e golden path.
#
# This script runs LOCALLY (never on CI). The resulting `sample.wav` is
# committed to the repo so CI does not need piper-tts installed and the test
# stays reproducible bit for bit.
#
# Prerequisites (one-time):
#   python3 -m venv .venv
#   source .venv/bin/activate
#   pip install piper-tts
#   # Download the voice model + config in this directory or pass absolute paths:
#   curl -L -o fr_FR-siwis-medium.onnx \
#     "https://huggingface.co/rhasspy/piper-voices/resolve/main/fr/fr_FR/siwis/medium/fr_FR-siwis-medium.onnx"
#   curl -L -o fr_FR-siwis-medium.onnx.json \
#     "https://huggingface.co/rhasspy/piper-voices/resolve/main/fr/fr_FR/siwis/medium/fr_FR-siwis-medium.onnx.json"
#
# Then:
#   ./generate.sh

set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
TEXT="Je murmure à l'oreille des ordinateurs"
OUT="$HERE/sample.wav"
MODEL="${PIPER_MODEL:-$HERE/fr_FR-siwis-medium.onnx}"
CONFIG="${PIPER_CONFIG:-$HERE/fr_FR-siwis-medium.onnx.json}"

if [ ! -f "$MODEL" ]; then
    echo "Missing voice model: $MODEL" >&2
    echo "Download it (see the comment at the top of this script)." >&2
    exit 1
fi

echo "$TEXT" | piper \
    --model "$MODEL" \
    --config "$CONFIG" \
    --output_file "$OUT"

echo "Generated: $OUT"
file "$OUT"
