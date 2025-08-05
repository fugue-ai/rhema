#!/bin/bash

set -e

docker run -p 6333:6333 -p 6334:6334 -d \
  -v "$(pwd)/target/qdrant_storage:/qdrant/storage:z" \
  --name rhema-qdrant \
  --rm \
  qdrant/qdrant