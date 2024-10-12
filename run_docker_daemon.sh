#!/usr/bin/bash

sudo docker container run -d -p 8000:8000 -p 80:80 --rm danswebengine > container_id
