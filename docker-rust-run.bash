#!/bin/bash
docker run --rm -it --name rust -v $PWD/src:/root/src --workdir="/root/src" rust
