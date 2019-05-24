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
        assert caget("FC:IDN").startswith("Agilent Technologies,53220A,MY57500140,")
        #assert caget("FC:FREQ_1") == "0"

if __name__ == "__main__":
    setup()
    #test_binding()
    test_ioc()
