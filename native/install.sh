#!/bin/sh
set -e

DIR="$( cd "$( dirname "$0" )" && pwd )"
if [ "$(uname -s)" = "Darwin" ]; then
  if [ "$(whoami)" = "root" ]; then
    CHROME_TARGET_DIR="/Library/Google/Chrome/NativeMessagingHosts"
    EDGE_TARGET_DIR="/Library/Microsoft/Edge/NativeMessagingHosts"
  else
    CHROME_TARGET_DIR="$HOME/Library/Application Support/Google/Chrome/NativeMessagingHosts"
    EDGE_TARGET_DIR="$HOME/Library/Application Support/Microsoft Edge/NativeMessagingHosts"
  fi
else
  # Linux paths (simplified for now)
  CHROME_TARGET_DIR="$HOME/.config/google-chrome/NativeMessagingHosts"
  EDGE_TARGET_DIR="$HOME/.config/microsoft-edge/NativeMessagingHosts"
fi

HOST_NAME=org.cardid.webcard.native

# Create directory to store native messaging host.
mkdir -p "$CHROME_TARGET_DIR"
mkdir -p "$EDGE_TARGET_DIR"

# Copy native messaging host manifest.
cp "$DIR/$HOST_NAME.json" "$CHROME_TARGET_DIR"
cp "$DIR/$HOST_NAME.json" "$EDGE_TARGET_DIR"

# Update host path in the manifest.
# Pointing to the Rust debug build for development
HOST_PATH="$DIR/target/debug/webcard"

# Escape the path for sed
ESCAPED_HOST_PATH=${HOST_PATH////\\/}

sed -i -e "s/HOST_PATH/$ESCAPED_HOST_PATH/" "$CHROME_TARGET_DIR/$HOST_NAME.json"
sed -i -e "s/HOST_PATH/$ESCAPED_HOST_PATH/" "$EDGE_TARGET_DIR/$HOST_NAME.json"

echo "Native messaging helper $HOST_NAME has been installed."
echo "Host path set to: $HOST_PATH"
