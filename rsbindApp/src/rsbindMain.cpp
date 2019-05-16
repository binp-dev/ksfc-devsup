/* rsbindMain.cpp */
/* Author:  Marty Kraimer Date:    17MAR2000 */

#include <stddef.h>
#include <stdlib.h>
#include <stddef.h>
#include <string.h>
#include <stdio.h>

#include "epicsExit.h"
#include "epicsThread.h"
#include "iocsh.h"

extern "C" long rsbind_init();
extern "C" long rsbind_quit();

int main(int argc,char *argv[])
{
	rsbind_init();
    if(argc>=2) {    
        iocsh(argv[1]);
        epicsThreadSleep(.2);
    }
    rsbind_quit();
    iocsh(NULL);
    epicsExit(0);
    return(0);
}
