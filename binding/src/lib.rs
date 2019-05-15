#![allow(dead_code)]
#![allow(unused_variables)]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod epics;

use libc::{c_int, c_long};
use epics::{
    dbCommon, aiRecord, aoRecord, biRecord, boRecord,
};

// ai record

#[no_mangle]
extern fn rec_ai_init_record(rec: *mut aiRecord) -> c_long {
    println!("rec_ai_init_record");
    0
}
#[no_mangle]
extern fn rec_ai_get_ioint_info(cmd: c_int, rec: *mut dbCommon) -> c_long {
    println!("rec_ai_get_ioint_info");
    0
}
#[no_mangle]
extern fn rec_ai_read_ai(rec: *mut aiRecord) -> c_long {
    println!("rec_ai_read_ai");
    0
}
#[no_mangle]
extern fn rec_ai_special_linconv(rec: *mut aiRecord, after: c_int) -> c_long {
    println!("rec_ai_special_linconv");
    0
}

// ao record

#[no_mangle]
extern fn rec_ao_init_record(rec: *mut aoRecord) -> c_long {
    println!("rec_ao_init_record");
    0
}
#[no_mangle]
extern fn rec_ao_get_ioint_info(cmd: c_int, rec: *mut dbCommon) -> c_long {
    println!("rec_ao_get_ioint_info");
    0
}
#[no_mangle]
extern fn rec_ao_write_ao(rec: *mut aoRecord) -> c_long {
    println!("rec_ao_write_ao");
    0
}
#[no_mangle]
extern fn rec_ao_special_linconv(rec: *mut aoRecord, after: c_int) -> c_long {
    println!("rec_ao_special_linconv");
    0
}

// bi record

#[no_mangle]
extern fn rec_bi_init_record(rec: *mut biRecord) -> c_long {
    println!("rec_bi_init_record");
    0
}
#[no_mangle]
extern fn rec_bi_get_ioint_info(cmd: c_int, rec: *mut dbCommon) -> c_long {
    println!("rec_bi_get_ioint_info");
    0
}
#[no_mangle]
extern fn rec_bi_read_bi(rec: *mut biRecord, after: c_int) -> c_long {
    println!("rec_bi_read_bi");
    0
}

// bo record

#[no_mangle]
extern fn rec_bo_init_record(rec: *mut boRecord) -> c_long {
    println!("rec_bo_init_record");
    0
}
#[no_mangle]
extern fn rec_bo_get_ioint_info(cmd: c_int, rec: *mut dbCommon) -> c_long {
    println!("rec_bo_get_ioint_info");
    0
}
#[no_mangle]
extern fn rec_bo_write_bo(rec: *mut boRecord, after: c_int) -> c_long {
    println!("rec_bo_write_bo");
    0
}
