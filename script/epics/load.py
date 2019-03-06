#!/usr/bin/python3

import os
from subprocess import run

if __name__ == "__main__":
    if os.path.exists("epics-base"):
        print("already loaded")
        exit(0)
    else:
        res = run([
            "git", "clone",
            "https://github.com/epics-base/epics-base.git",
            "epics-base",
            "--branch", "R7.0.2",
        ])
        if res.returncode != 0:
            print("git clone error")
            exit(1)
