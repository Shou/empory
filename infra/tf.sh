#!/usr/bin/env bash

set -a
source ../.env.dev
set +a

export TF_VAR_access_key="$AWS_ACCESS_KEY_ID"
export TF_VAR_secret_key="$AWS_SECRET_ACCESS_KEY"
export TF_VAR_endpoint="$S3_ENDPOINT"

terraform "$@"