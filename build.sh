#!/bin/bash
# Cross-platform build script for compiling the project
# Matthew Cross <blacklightgfx@gmail.com>
# Copyright 2015

if [ "$(uname)" == *"Darwin"* ]; then
    cargo build
elif [ "$(expr substr $(uname -s) 1 5)" == *"Linux"* ]; then
    cargo build
elif [ "$(expr substr $(uname -s) 1 10)" == *"MINGW32_NT"* ]; then
    env OPENSSL_LIB_DIR=C:/OpenSSL-Win64 OPENSSL_INCLUDE_DIR=C:/OpenSSL-Win64/include cargo build
fi