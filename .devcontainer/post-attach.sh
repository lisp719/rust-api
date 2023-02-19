#!/bin/sh

rustup component add rustfmt

git config --global --add safe.directory /workspaces/rust-api
