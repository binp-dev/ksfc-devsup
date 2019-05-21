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

#include <iocsh.h>

extern void rsbind_init(void);
extern void rsbind_quit(void);

extern long rsbind_get_ioint_info  (int cmd, struct dbCommon *rec, IOSCANPVT *ppvt);

extern long rsbind_ai_init_record     (struct aiRecord *rec);
extern long rsbind_ai_read_ai         (struct aiRecord *rec);
extern long rsbind_ai_special_linconv (struct aiRecord *rec, int after);

extern long rsbind_ao_init_record     (struct aoRecord *rec);
extern long rsbind_ao_write_ao        (struct aoRecord *rec);
extern long rsbind_ao_special_linconv (struct aoRecord *rec, int after);

extern long rsbind_bi_init_record     (struct biRecord *rec);
extern long rsbind_bi_read_bi         (struct biRecord *rec);

extern long rsbind_bo_init_record     (struct boRecord *rec);
extern long rsbind_bo_write_bo        (struct boRecord *rec);

extern long rsbind_longin_init_record    (struct biRecord *rec);
extern long rsbind_longin_read_longin    (struct biRecord *rec);

extern long rsbind_longout_init_record   (struct boRecord *rec);
extern long rsbind_longout_write_longout (struct boRecord *rec);

extern long rsbind_stringin_init_record      (struct biRecord *rec);
extern long rsbind_stringin_read_stringin    (struct biRecord *rec);

extern long rsbind_stringout_init_record     (struct boRecord *rec);
extern long rsbind_stringout_write_stringout (struct boRecord *rec);

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
struct RecLongin {
    long number;
    DEVSUPFUN report;
    DEVSUPFUN init;
    DEVSUPFUN init_record;
    DEVSUPFUN get_ioint_info;
    DEVSUPFUN read_longin;
};
struct RecLongout {
    long number;
    DEVSUPFUN report;
    DEVSUPFUN init;
    DEVSUPFUN init_record;
    DEVSUPFUN get_ioint_info;
    DEVSUPFUN write_longout;
};
struct RecStringin {
    long number;
    DEVSUPFUN report;
    DEVSUPFUN init;
    DEVSUPFUN init_record;
    DEVSUPFUN get_ioint_info;
    DEVSUPFUN read_stringin;
};
struct RecStringout {
    long number;
    DEVSUPFUN report;
    DEVSUPFUN init;
    DEVSUPFUN init_record;
    DEVSUPFUN get_ioint_info;
    DEVSUPFUN write_stringout;
};

struct RecAi rec_ai = {
    6,
    NULL,
    NULL,
    rsbind_ai_init_record,
    rsbind_get_ioint_info,
    rsbind_ai_read_ai,
    rsbind_ai_special_linconv
};
struct RecAo rec_ao = {
    6,
    NULL,
    NULL,
    rsbind_ao_init_record,
    rsbind_get_ioint_info,
    rsbind_ao_write_ao,
    rsbind_ao_special_linconv
};
struct RecBi rec_bi = {
    5,
    NULL,
    NULL,
    rsbind_bi_init_record,
    rsbind_get_ioint_info,
    rsbind_bi_read_bi
};
struct RecBo rec_bo = {
    5,
    NULL,
    NULL,
    rsbind_bo_init_record,
    rsbind_get_ioint_info,
    rsbind_bo_write_bo
};
struct RecLongin rec_longin = {
    5,
    NULL,
    NULL,
    rsbind_longin_init_record,
    rsbind_get_ioint_info,
    rsbind_longin_read_longin
};
struct RecLongout rec_longout = {
    5,
    NULL,
    NULL,
    rsbind_longout_init_record,
    rsbind_get_ioint_info,
    rsbind_longout_write_longout
};
struct RecStringin rec_stringin = {
    5,
    NULL,
    NULL,
    rsbind_stringin_init_record,
    rsbind_get_ioint_info,
    rsbind_stringin_read_stringin
};
struct RecStringout rec_stringout = {
    5,
    NULL,
    NULL,
    rsbind_stringout_init_record,
    rsbind_get_ioint_info,
    rsbind_stringout_write_stringout
};

epicsExportAddress(dset, rec_ai);
epicsExportAddress(dset, rec_ao);
epicsExportAddress(dset, rec_bi);
epicsExportAddress(dset, rec_bo);
epicsExportAddress(dset, rec_longin);
epicsExportAddress(dset, rec_longout);
epicsExportAddress(dset, rec_stringin);
epicsExportAddress(dset, rec_stringout);

static void rsbind(void) {
    rsbind_init();
}

epicsExportRegistrar(rsbind);
