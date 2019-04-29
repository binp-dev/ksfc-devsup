#include <stdio.h>
#include <stdlib.h>
#include <devSup.h>
#include <recGbl.h>
#include <alarm.h>
#include <aoRecord.h>
#include <epicsExport.h>

long report() {
    printf("scope.report\n");
    return 0;
}

long init() {
    printf("scope.init\n");
    return 0;
}

long init_record(aoRecord *record) {
    printf("scope.init_record %s\n", record->name);
    return 0;
}

long get_ioint_info() {
    printf("scope.get_ioint_info\n");
    return 0;
}

long read_or_write(aoRecord *record) {
    printf("scope.read_or_write %s\n", record->name);
    return 0;
}

long special_linconv(aoRecord *record, int after) {
    printf("scope.special_linconv %s\n", record->name);
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
