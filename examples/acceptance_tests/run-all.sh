#!/usr/bin/env bash

find . -regex ".*[0-9]\{3\}" -type d | xargs -P 8 -I {} -- ./run.sh {}
