#!/bin/bash
# verbose/exit on error
set -xe

docker build -t devalto/soccerbot_base ..
docker build --rm -t devalto/soccerbot_build .
docker run --rm -v /var/run/docker.sock:/var/run/docker.sock -ti devalto/soccerbot_build
docker rmi devalto/soccerbot_build
