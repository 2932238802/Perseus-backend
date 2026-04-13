#!/bin/bash

set -e

PROJECT_NAME=$1

if [ -z "$PROJECT_NAME" ]; then
    echo "Error: Project name is null!"
    exit 1
fi

echo "creating rust project $PROJECT_NAME ..."
cargo new "$PROJECT_NAME"

