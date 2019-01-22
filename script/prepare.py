#!/usr/bin/python3

import os
import sys

from setup import setup
from tools import substitute, try_remove


def prepare():
    substitute(
        "configure/RELEASE.pre",
        "configure/RELEASE",
        {
            "%{EPICS_BASE}": os.environ["EPICS_BASE"],
        },
    )

def clean():
    try_remove("configure/RELEASE")

if __name__ == "__main__":
    setup()
    if "--clean" not in sys.argv:
        prepare()
    else:
        clean()
