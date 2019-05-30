#!/bin/bash

source ./script/epics/env.sh &&
cd epics-base &&
make &&
cd .. &&
echo "epics built successfully"
