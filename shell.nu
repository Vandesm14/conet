^mkdir -p /tmp/conet
let-env GCLOUD_BEARER = (gcloud auth application-default print-access-token)