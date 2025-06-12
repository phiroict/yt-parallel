#!/usr/bin/env bash
set -euo pipefail
TARGET=/run/media/phiro/huge/media/youtube
LIST="$HOME/Desktop/videolist.txt"
YTDLP_OPTS=(
  --sponsorblock-remove default
  --fragment-retries infinite
  --buffer-size 16K
)
FOLDER=$(date +%Y%m%d)
mkdir -p "${TARGET}/${FOLDER}" || exit 2
cd "${TARGET}/${FOLDER}" || exit 1
# Read each URL, fire off yt-dlp in the background, silencing stdout+stderr
while IFS= read -r url; do
  yt-dlp "${YTDLP_OPTS[@]}" "$url" > /dev/null 2>&1 &
done < "$LIST"

# Wait for all background jobs to finish
wait

cd -