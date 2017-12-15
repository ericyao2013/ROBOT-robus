#!/bin/bash

if [ $TARGET = thumbv6m-none-eabi ]; then
  xargo build --target $TARGET --examples
fi
