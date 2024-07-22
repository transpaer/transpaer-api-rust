#!/bin/bash

rm -rf sustainity_api

openapi-generator validate -i openapi/sustainity.yaml

openapi-generator generate \
     -i openapi/sustainity.yaml \
     -o sustainity_api \
     --additional-properties packageName=sustainity-api,packageVersion=0.3.0 \
     -g rust-server

yj openapi/schemas.yaml -o openapi/schemas.json

cargo typify openapi/schemas.json -a PartialEq -o sustainity_api/src/models.rs
