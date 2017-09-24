#!/bin/bash

MY_PATH="`dirname \"$0\"`"              # relative
MY_PATH="`(cd \"$MY_PATH\" && pwd )`"  # absolutized and normalized
if [ -z "$MY_PATH" ] ; then
  # error; for some reason, the path is not accessible
  # to the script (e.g. permissions re-evaled after suid)
  exit 1  # fail
fi

HASH=$(docker run -p 8080:8080 -p 80:80 -v $MY_PATH/traefik.toml:/etc/traefik/traefik.toml -v $MY_PATH/rules.toml:/rules.toml -v /var/run/docker.sock:/var/run/docker.sock traefik)

echo $HASH
echo $HASH > $MY_PATH/traefik.docker_hash
