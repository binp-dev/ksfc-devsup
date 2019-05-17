#!/usr/bin/python3

import os
import sys
from subprocess import run

from lib.setup import setup
from lib.tools import substitute, copy_file, try_make_dir


def prepare():
    substitute(
        "configure/RELEASE.pre",
        "configure/RELEASE",
        {
            "%{EPICS_BASE}": os.environ["EPICS_BASE"],
        },
    )
    try_make_dir("./lib")
    try_make_dir("./lib/" + os.environ["EPICS_HOST_ARCH"])

def binding():
    assert run(["cargo", "build"], cwd="./device_support").returncode == 0
    src_dir = "./device_support/target/debug/"
    dst_dir = "./lib/" + os.environ["EPICS_HOST_ARCH"] + "/"
    libs = ["libdevice_support.a", "libdevice_support.so"]
    try_make_dir(dst_dir)
    for lib in libs:
        copy_file(src_dir + lib, dst_dir + lib)

def build():
    prepare()
    binding()
    assert run(["make", "-f", "Makefile.pre"]).returncode == 0

if __name__ == "__main__":
    setup()
    build()
