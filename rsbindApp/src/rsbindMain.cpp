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

extern "C" void rsbind_init();
extern "C" void rsbind_quit();

int main(int argc,char *argv[])
{
	rsbind_init();
    if(argc>=2) {    
        iocsh(argv[1]);
        epicsThreadSleep(.2);
    }
    iocsh(NULL);
    rsbind_quit();
    epicsExit(0);
    return(0);
}
