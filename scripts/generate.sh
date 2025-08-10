#!/bin/bash

rm -rf transpaer_api

openapi-generator validate -i openapi/transpaer.json

openapi-generator generate \
     -i openapi/transpaer.json \
     -o transpaer_api \
     --additional-properties packageName=transpaer-api,packageVersion=0.3.0 \
     -g rust-server

cargo typify openapi/schemas.json -a PartialEq -o transpaer_api/src/models.rs
