#!/usr/bin/python3

import os
import sys
from subprocess import run

from lib.setup import setup
from lib.tools import substitute, try_remove


def prepare():
    substitute(
        "configure/RELEASE.pre",
        "configure/RELEASE",
        {
            "%{EPICS_BASE}": os.environ["EPICS_BASE"],
        },
    )

def build():
    prepare()
    assert run(["make", "-f", "Makefile.pre"]).returncode == 0

if __name__ == "__main__":
    setup()
    build()
