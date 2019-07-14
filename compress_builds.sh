#!/bin/sh
for dir in ./builds/*
do
    (cd "builds/$dir" && test -r ./ && test -w ./ && tar -czf "$dir.tar.gz" ./*)
    rm -rf "builds/$dir"
    echo "compressed $dir"
done