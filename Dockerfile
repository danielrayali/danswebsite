# syntax=docker/dockerfile:1

FROM ubuntu:24.04
WORKDIR /work
COPY website webapp/src/Rocket.toml webapp/target/release/dans-web-app /work/
ENTRYPOINT ["/work/dans-web-app"]
