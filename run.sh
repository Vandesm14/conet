#!/usr/bin/env bash
mkdir -p audio
GCLOUD_BEARER=$(gcloud auth application-default print-access-token) cargo run