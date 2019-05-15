#!/usr/bin/python3

import os
import sys
from subprocess import run

from lib.setup import setup
from lib.epics import CaRepeater, Ioc, caget, caput

def test_binding():
    assert run(["cargo", "test"], cwd="./binding").returncode == 0

def test_ioc():
    with CaRepeater(), Ioc("iocBoot/iocrsbind/st.cmd"):

        assert caget("AO_0") == "0"
        caput("AO_0", "42")
        assert caget("AO_0") == "42"

        assert caget("AO_1") == "0"
        caput("AO_1", "24")
        assert caget("AO_1") == "24"

if __name__ == "__main__":
    setup()
    test_binding()
    test_ioc()
