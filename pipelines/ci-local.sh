#!/usr/bin/env bash
docker build --output type=tar,dest=/dev/null -f pipelines/Dockerfile.ci .
