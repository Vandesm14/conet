#!/usr/bin/env bash
mkdir -p /tmp/conet
GCLOUD_BEARER=$(gcloud auth application-default print-access-token) cargo run --release