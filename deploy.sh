#!/usr/bin/bash

./build_docker.sh
scp -i id_ed25519 danswebengine.tar.gz admin@146.190.75.147:~/
ssh -i id_ed25519 admin@146.190.75.147 "gzip -d ./danswebengine.tar.gz"
ssh -i id_ed25519 admin@146.190.75.147 "docker container stop $(cat container_id)"
ssh -i id_ed25519 admin@146.190.75.147 "docker load -i danswebengine.tar"
ssh -i id_ed25519 admin@146.190.75.147 "docker container run -d -p 80:80 --rm danswebengine > container_id"
