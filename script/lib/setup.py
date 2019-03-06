import sys
import logging

def setup():
    if "--quiet" in sys.argv:
        level = logging.INFO
    else:
        level = logging.DEBUG

    logger = logging.getLogger()
    logger.setLevel(level)

    sh = logging.StreamHandler()
    sh.setLevel(level)
    sh.setFormatter(logging.Formatter('%(message)s'))
    logger.addHandler(sh)
