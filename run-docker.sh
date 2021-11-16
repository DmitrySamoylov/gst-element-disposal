#!/bin/bash

SCRIPT_DIR=$(dirname "$0")
TARGET_DIR=$(readlink -f "$SCRIPT_DIR/target/debug")

docker run \
  --rm \
  -e GST_DEBUG=2,GST_REFCOUNTING:5 \
  -v "$TARGET_DIR:/target" \
  restreamio/gstreamer:1.18.4-prod-dbg /target/gst-element-disposal
