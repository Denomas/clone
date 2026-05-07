# syntax=docker/dockerfile:1.7
#
# Distroless-style runtime image for clone. The static musl binary has
# no library dependencies, so this image is effectively just the binary
# itself. Build context is expected to contain the binary at ./clone
# (the CI `package` job downloads the `clone-linux-x86_64` artifact
# into the context before invoking `docker build`).
#
# Running:
#   docker run --rm -it \
#     --device /dev/kvm \
#     --cap-add=NET_ADMIN --device /dev/net/tun \
#     ghcr.io/denomas/clone:latest --help
#
# /dev/kvm and NET_ADMIN are required by the VMM to open KVM and to
# manage the TAP device for guest networking. Without them clone will
# fail at startup with EACCES.

FROM scratch

COPY clone /clone

LABEL org.opencontainers.image.source="https://github.com/Denomas/clone"
LABEL org.opencontainers.image.description="Lightweight Linux VMM"
LABEL org.opencontainers.image.licenses="MIT"
LABEL org.opencontainers.image.documentation="https://github.com/Denomas/clone/blob/master/docs/SPEC.md"

ENTRYPOINT ["/clone"]
