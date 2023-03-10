#!/usr/bin/env bash
trunk build -d docs
gsed -i 's#/rust-powered-homepage#/rust-powered-homepage/rust-powered-homepage#g' docs/index.html
