#!/usr/bin/env bash

dirs=$( ls -d */ )

for dir in $dirs; do
  cd $dir
  cargo clean
  cd ..
done
