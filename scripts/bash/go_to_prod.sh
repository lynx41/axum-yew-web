#!/bin/bash -e

# compile frontend
cd ../../frontend && trunk build --release && \
# compile backend
cd ../server && cargo build --release && \
# run docker files
cd ../scripts && docker-compose up