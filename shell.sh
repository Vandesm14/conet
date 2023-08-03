#!/usr/bin/env bash

mkdir -p /tmp/conet
export GCLOUD_BEARER=$(gcloud auth application-default print-access-token)