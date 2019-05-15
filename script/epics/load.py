#!/usr/bin/python3

import os
from subprocess import run

if __name__ == "__main__":
    if os.path.exists("epics-base"):
        print("already loaded")
    else:
        # clone
        assert run([
            "git", "clone",
            "https://github.com/epics-base/epics-base.git",
            "epics-base",
            "--branch", "R7.0.2",
        ]).returncode == 0, "git clone error"

        # change config to use clang
        assert run(
            ["git", "apply", "../script/epics/clang.patch"],
            cwd="./epics-base"
        ).returncode == 0, "git apply patch error"
