#!/bin/bash

URL=$1
DATA="$(cat $2)"

curl -H "Authorization: Bearer $TOKEN" $URL -v -d "$DATA"
