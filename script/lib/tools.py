import os
import shutil

import logging

logger = logging.getLogger()

def substitute(ifp, ofp, rep):
    with open(ifp, 'r') as file :
        data = file.read()

    logger.debug("substitute '%s' -> '%s':" % (ifp, ofp))
    for k, v in rep.items():
        data = data.replace(k, v)
        logger.debug("  %s -> %s" % (k, v))

    with open(ofp, 'w') as file:
        file.write(data)

def try_remove(fp):
    try:
        os.remove(fp)
    except FileNotFoundError:
        pass
    else:
        logger.debug("removed '%s'" % fp)

def try_remove_dir(dp):
    try:
        shutil.rmtree(dp)
    except FileNotFoundError:
        pass
    else:
        logger.debug("removed dir '%s'" % dp)
