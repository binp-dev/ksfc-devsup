#include <stdio.h>
#include <stdlib.h>
#include <devSup.h>
#include <recGbl.h>
#include <alarm.h>

#include <aiRecord.h>
#include <aoRecord.h>
#include <biRecord.h>
#include <boRecord.h>

#include <epicsExport.h>


extern long rec_ai_init_record     (struct aiRecord *rec);
extern long rec_ai_get_ioint_info  (int cmd, struct dbCommon *rec, IOSCANPVT *ppvt);
extern long rec_ai_read_ai         (struct aiRecord *rec);
extern long rec_ai_special_linconv (struct aiRecord *rec, int after);

extern long rec_ao_init_record     (struct aoRecord *rec);
extern long rec_ao_get_ioint_info  (int cmd, struct dbCommon *rec, IOSCANPVT *ppvt);
extern long rec_ao_write_ao        (struct aoRecord *rec);
extern long rec_ao_special_linconv (struct aoRecord *rec, int after);

extern long rec_bi_init_record     (struct biRecord *rec);
extern long rec_bi_get_ioint_info  (int cmd, struct dbCommon *rec, IOSCANPVT *ppvt);
extern long rec_bi_read_bi         (struct biRecord *rec);

extern long rec_bo_init_record     (struct boRecord *rec);
extern long rec_bo_get_ioint_info  (int cmd, struct dbCommon *rec, IOSCANPVT *ppvt);
extern long rec_bo_write_bo        (struct boRecord *rec);


struct RecAi {
    long number;
    DEVSUPFUN report;
    DEVSUPFUN init;
    DEVSUPFUN init_record;
    DEVSUPFUN get_ioint_info;
    DEVSUPFUN read_ai;
    DEVSUPFUN special_linconv;
};
struct RecAo {
    long number;
    DEVSUPFUN report;
    DEVSUPFUN init;
    DEVSUPFUN init_record;
    DEVSUPFUN get_ioint_info;
    DEVSUPFUN write_ao;
    DEVSUPFUN special_linconv;
};
struct RecBi {
    long number;
    DEVSUPFUN report;
    DEVSUPFUN init;
    DEVSUPFUN init_record;
    DEVSUPFUN get_ioint_info;
    DEVSUPFUN read_bi;
};
struct RecBo {
    long number;
    DEVSUPFUN report;
    DEVSUPFUN init;
    DEVSUPFUN init_record;
    DEVSUPFUN get_ioint_info;
    DEVSUPFUN write_bo;
};

struct RecAi rec_ai = {
    6,
    NULL,
    NULL,
    rec_ai_init_record,
    rec_ai_get_ioint_info,
    rec_ai_read_ai,
    rec_ai_special_linconv
};
struct RecAo rec_ao = {
    6,
    NULL,
    NULL,
    rec_ao_init_record,
    rec_ao_get_ioint_info,
    rec_ao_write_ao,
    rec_ao_special_linconv
};
struct RecBi rec_bi = {
    5,
    NULL,
    NULL,
    rec_bi_init_record,
    rec_bi_get_ioint_info,
    rec_bi_read_bi
};
struct RecBo rec_bo = {
    5,
    NULL,
    NULL,
    rec_bo_init_record,
    rec_bo_get_ioint_info,
    rec_bo_write_bo
};

epicsExportAddress(dset, rec_ai);
epicsExportAddress(dset, rec_ao);
epicsExportAddress(dset, rec_bi);
epicsExportAddress(dset, rec_bo);
