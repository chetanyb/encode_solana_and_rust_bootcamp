#!/bin/bash

for txtfile in *.txt; do
  foldername=$(basename "$txtfile" .txt)
  if [ -d "$foldername" ]; then
    mv "$txtfile" "$foldername/"
  fi
done
