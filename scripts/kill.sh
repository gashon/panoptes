#!/usr/bin/env bash

pgrep -f "panoptes" | xargs -I {} kill -9 {}