#!/bin/bash

docker run --rm -itd -p 8080:8080 --name chonkify gcr.io/chonkify/chonkify:latest

docker logs chonkify -f
