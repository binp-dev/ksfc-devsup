#include <stdio.h>
#include <stdlib.h>
#include <devSup.h>
#include <recGbl.h>
#include <alarm.h>
#include <aoRecord.h>
#include <boRecord.h>
#include <epicsExport.h>

long rec_ao__report() {
    printf("rec_ao.report\n");
    return 0;
}

long rec_ao__init() {
    printf("rec_ao.init\n");
    return 0;
}

long rec_ao__init_record(aoRecord *record) {
    printf("rec_ao.init_record %s\n", record->name);
    return 0;
}

long rec_ao__get_ioint_info() {
    printf("rec_ao.get_ioint_info\n");
    return 0;
}

long rec_ao__write_ao(aoRecord *record) {
    printf("rec_ao.write_ao %s\n", record->name);
    return 0;
}

long rec_ao__special_linconv(aoRecord *record, int after) {
    printf("rec_ao.special_linconv %s\n", record->name);
    return 0;
}


struct RecAo {
    long number;
    DEVSUPFUN report;
    DEVSUPFUN init;
    DEVSUPFUN init_record;
    DEVSUPFUN get_ioint_info;
    DEVSUPFUN write_ao;
    DEVSUPFUN special_linconv;
};

struct RecAo rec_ao = {
    6,
    rec_ao__report,
    rec_ao__init,
    rec_ao__init_record,
    rec_ao__get_ioint_info,
    rec_ao__write_ao,
    rec_ao__special_linconv
};

epicsExportAddress(dset, rec_ao);


long rec_bo__report() {
    printf("rec_bo.report\n");
    return 0;
}

long rec_bo__init() {
    printf("rec_bo.init\n");
    return 0;
}

long rec_bo__init_record(boRecord *record) {
    printf("rec_bo.init_record %s\n", record->name);
    return 0;
}

long rec_bo__get_ioint_info() {
    printf("rec_bo.get_ioint_info\n");
    return 0;
}

long rec_bo__write_bo(boRecord *record) {
    printf("rec_bo.write_bo %s\n", record->name);
    return 0;
}

struct RecBo {
    long number;
    DEVSUPFUN report;
    DEVSUPFUN init;
    DEVSUPFUN init_record;
    DEVSUPFUN get_ioint_info;
    DEVSUPFUN write_bo;
};

struct RecAo rec_bo = {
    5,
    rec_bo__report,
    rec_bo__init,
    rec_bo__init_record,
    rec_bo__get_ioint_info,
    rec_bo__write_bo
};

epicsExportAddress(dset, rec_bo);
