#!/usr/bin/python3

import os
import sys

from lib.setup import setup
from lib.epics import CaRepeater, Ioc, caget, caput


def test():
    with CaRepeater(), Ioc("iocBoot/ioclxi/st.cmd"):

        assert caget("SCOPE") == "0"
        
        caput("SCOPE", "42")
        assert caget("SCOPE") == "42"


if __name__ == "__main__":
    setup()
    test()
