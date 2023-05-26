#!/bin/bash
while read json; do
  curl --json "$json" http://127.0.0.1:8080/api/product > /dev/null
done < all.jsons
