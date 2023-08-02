#!/usr/bin/env bash
GCLOUD_BEARER=$(gcloud auth application-default print-access-token) cargo run
mpv audio.wav