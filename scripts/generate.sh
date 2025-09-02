#!/bin/bash

rm -rf transpaer_api

version=$(cargo get workspace.package.version)

openapi-generator validate -i openapi/transpaer.json

openapi-generator generate \
     -i openapi/transpaer.json \
     -o transpaer_api \
     --additional-properties packageName=transpaer-api,packageVersion=$version \
     -g rust-server

cargo typify openapi/schemas.json -a PartialEq -o transpaer_api/src/models.rs
