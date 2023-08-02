#!/usr/bin/env bash
mkdir -p audio
rm -rf audio/*
GCLOUD_BEARER=$(gcloud auth application-default print-access-token) cargo run