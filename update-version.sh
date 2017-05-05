#!/bin/sh -e

version="0.1.3"

sed -e 's,^version = .*,version = "'$version'",' -i '' \
    Cargo.toml http2/Cargo.toml grpc-compiler/Cargo.toml

sed -e '/httpbis.*path/ s,version = [^ ]*,version = "'$version'",' -i '' Cargo.toml

# vim: set ts=4 sw=4 et:
