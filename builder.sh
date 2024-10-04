#!/bin/bash
docker build -t filler .
docker run -v "$(pwd)/solution":/filler/solution -it filler