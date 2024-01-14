#!/bin/sh
trap 'exit 0' SIGTERM
while true; do
    ncat -l 8000 < response.http
done
