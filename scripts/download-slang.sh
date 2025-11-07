#!/usr/bin/env bash

VERSION="${1:-}"
PLATFORM="${2:-}"

if [ -z "$PLATFORM" ]; then
  echo "Usage: $0 <version> <platform>"
  exit 1
fi

ASSET_NAME="slang-${VERSION}-${PLATFORM}.zip"
URL="https://github.com/shader-slang/slang/releases/download/v${VERSION}/${ASSET_NAME}"
echo "Downloading Slang ${VERSION} for ${PLATFORM}"
echo "URL: ${URL}"

curl -L -o "slang-release.zip" "$URL"

echo "Extracting..."
mkdir -p slang_dir
unzip -q slang-release.zip -d slang_dir_tmp
mv slang_dir_tmp/*/* slang_dir/
rm -rf slang_dir_tmp slang-release.zip

SLANG_DIR=$(cd slang_dir && pwd)

echo "Extracted to: ${SLANG_DIR}"
echo "SLANG_DIR=${SLANG_DIR}"
