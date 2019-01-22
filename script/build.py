#!/usr/bin/python3

import os
import sys
from subprocess import run

from setup import setup
from prepare import prepare, clean as prepare_clean


def build():
    prepare()
    assert run(["make", "-f", "Makefile.pre"]).returncode == 0

def clean():
    prepare()
    assert run(["make", "-f", "Makefile.pre", "clean"]).returncode == 0
    prepare_clean()

if __name__ == "__main__":
    setup()
    if "--clean" not in sys.argv:
        build()
    else:
        clean()
