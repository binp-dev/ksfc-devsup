#!/usr/bin/python3

import os
import sys

from lib.setup import setup
from lib.epics import CaRepeater, Ioc, caget, caput


def test():
    with CaRepeater(), Ioc("iocBoot/iocrsbind/st.cmd"):

        assert caget("AO_0") == "0"
        caput("AO_0", "42")
        assert caget("AO_0") == "42"

        assert caget("AO_1") == "0"
        caput("AO_1", "24")
        assert caget("AO_1") == "24"


if __name__ == "__main__":
    setup()
    test()
