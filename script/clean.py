#!/usr/bin/python3

import os
import sys
from subprocess import run

from lib.setup import setup
from lib.tools import substitute, try_remove_file, try_remove_dir


def clean():
    try_remove_file("configure/RELEASE")

    try_remove_file("iocBoot/iocrsbind/envPaths")

    for d in ["bin", "db", "dbd", "lib"]:
        try_remove_dir(d)

    for bd in ["configure", "rsbindApp", "rsbindSup"]:
        for r, ld, _ in os.walk(bd):
            for d in ld:
                if d.startswith("O."):
                    try_remove_dir(os.path.join(r, d))

if __name__ == "__main__":
    setup()
    clean()
