import os
from subprocess import Popen, run, PIPE
import time
import logging

logger = logging.getLogger()

def caget(pv):
    logger.debug("caget %s ..." % pv)
    ret = run(
        ["caget", "-t", pv],
        stdout=PIPE,
        universal_newlines=True
    )
    out = ret.stdout.strip()
    assert ret.returncode == 0
    logger.debug("  %s" % out)
    return out

def caput(pv, value):
    logger.debug("caput %s %s ..." % (pv, str(value)))
    ret = run(
        ["caput", "-t", pv, str(value)],
        stdout=PIPE,
        universal_newlines=True
    )
    assert ret.returncode == 0
    logger.debug("done")

class Ioc:
    def __init__(self, path):
        self.path = path
        self.proc = None

    def __enter__(self):
        with open(self.path, 'r') as file:
            exe = file.readline().strip()[2:]
        cwd, fn = self.path.rsplit('/', 1)
        self.proc = Popen(
            [exe, fn],
            cwd=cwd,
            stdout=PIPE,
            universal_newlines=True
        )
        time.sleep(1)
        logger.debug("ioc '%s' started" % self.path)

    def __exit__(self, *args):
        logger.debug("terminating '%s' ..." % self.path)
        self.proc.terminate()
        logger.debug("ioc '%s' terminated" % self.path)
