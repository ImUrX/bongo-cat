#!/bin/sh
cd builds
for dir in ./*
do
    cd "$dir" && tar -czf "../$dir.tar.gz" * && cd - && rm -rf "$dir" && echo "compressed $dir"
done