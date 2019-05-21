#!/usr/bin/python3

import os
import sys
import time
from subprocess import run

from lib.setup import setup
from lib.epics import CaRepeater, Ioc, caget, caput

def test_binding():
    assert run(["cargo", "test"], cwd="./binding").returncode == 0

def test_ioc():
    with CaRepeater(), Ioc("iocBoot/iocrsbind/st.cmd"):
        """
        assert caget("AO") == "0"
        caput("AO", "42")
        assert caget("AO") == "42"
        caput("AO", "-17")
        assert caget("AO") == "-17"

        assert caget("AI") == "0"

        caput("BO", "0")
        """

        time.sleep(10)

if __name__ == "__main__":
    setup()
    #test_binding()
    test_ioc()
