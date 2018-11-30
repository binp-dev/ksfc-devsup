#include <stdio.h>
#include <stdlib.h>
#include <devSup.h>
#include <recGbl.h>
#include <alarm.h>
#include <aoRecord.h>
#include <epicsExport.h>

long report() {
    printf("report\n");
    return 0;
}

long init() {
    printf("init\n");
    return 0;
}

long init_record(aoRecord *record) {
    printf("init_record\n");
    return 0;
}

long get_ioint_info() {
    printf("get_ioint_info\n");
    return 0;
}

long read_or_write(aoRecord *record) {
    printf("read_or_write\n");
    return 0;
}

long special_linconv(aoRecord *record, int after) {
    printf("special_linconv\n");
    if (after) {
        record->eslo = (record->eguf - record->egul)/0xFFFF;
        record->eoff = record->egul;
    }
    return 0;
}

struct Scope {
    long number;
    DEVSUPFUN report;
    DEVSUPFUN init;
    DEVSUPFUN init_record;
    DEVSUPFUN get_ioint_info;
    DEVSUPFUN read_or_write;
    DEVSUPFUN special_linconv;
};

struct Scope scope = {
    6,
    report,
    init,
    init_record,
    get_ioint_info,
    read_or_write,
    special_linconv,
};

epicsExportAddress(dset, scope);
