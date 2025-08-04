#!/bin/bash

rm -rf sustainity_api

openapi-generator validate -i openapi/sustainity.json

openapi-generator generate \
     -i openapi/sustainity.json \
     -o sustainity_api \
     --additional-properties packageName=sustainity-api,packageVersion=0.3.0 \
     -g rust-server

cargo typify openapi/schemas.json -a PartialEq -o sustainity_api/src/models.rs
