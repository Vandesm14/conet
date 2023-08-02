#!/usr/bin/env bash
mkdir audio
GCLOUD_BEARER=$(gcloud auth application-default print-access-token) cargo run