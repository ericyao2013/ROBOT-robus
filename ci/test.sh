#!/bin/bash

if [ $TARGET = x86_64-unknown-linux-gnu ]; then
  cargo test -- --test-threads=1
fi
