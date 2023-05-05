#!/usr/bin/env bash
trunk build -d docs --release --public-url '/rust-powered-homepage/'
#gsed -i 's#/rust-powered-homepage#/rust-powered-homepage/rust-powered-homepage#g' docs/index.html
