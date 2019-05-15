#![allow(dead_code)]
#![allow(unused_variables)]

use libc::{c_int, c_long};

use crate::epics::{
    dbCommon, aiRecord, aoRecord, biRecord, boRecord,
};

use crate::record::{Record, AiRecord, AoRecord, BiRecord, BoRecord};

// ai record

#[no_mangle]
extern fn rec_ai_init_record(rec: *mut aiRecord) -> c_long {
    let mut r = AiRecord::new(unsafe { rec.as_mut().unwrap() });
    r.set_val(0.0);
    println!("rec_ai_init_record({}): {}", r.name(), r.val());
    0
}
#[no_mangle]
extern fn rec_ai_get_ioint_info(cmd: c_int, rec: *mut dbCommon) -> c_long {
    let r = Record::new(unsafe { rec.as_mut().unwrap() });
    println!("rec_ai_get_ioint_info({})", r.name());
    0
}
#[no_mangle]
extern fn rec_ai_read_ai(rec: *mut aiRecord) -> c_long {
    let r = AiRecord::new(unsafe { rec.as_mut().unwrap() });
    println!("rec_ai_read_ai({})", r.name());
    0
}
#[no_mangle]
extern fn rec_ai_special_linconv(rec: *mut aiRecord, after: c_int) -> c_long {
    let r = AiRecord::new(unsafe { rec.as_mut().unwrap() });
    println!("rec_ai_special_linconv({})", r.name());
    0
}

// ao record

#[no_mangle]
extern fn rec_ao_init_record(rec: *mut aoRecord) -> c_long {
    let mut r = AoRecord::new(unsafe { rec.as_mut().unwrap() });
    r.set_val(0.0);
    println!("rec_ao_init_record({}): {}", r.name(), r.val());
    0
}
#[no_mangle]
extern fn rec_ao_get_ioint_info(cmd: c_int, rec: *mut dbCommon) -> c_long {
    let r = Record::new(unsafe { rec.as_mut().unwrap() });
    println!("rec_ao_get_ioint_info({})", r.name());
    0
}
#[no_mangle]
extern fn rec_ao_write_ao(rec: *mut aoRecord) -> c_long {
    let r = AoRecord::new(unsafe { rec.as_mut().unwrap() });
    println!("rec_ao_write_ao({}): {}", r.name(), r.val());
    0
}
#[no_mangle]
extern fn rec_ao_special_linconv(rec: *mut aoRecord, after: c_int) -> c_long {
    let r = AoRecord::new(unsafe { rec.as_mut().unwrap() });
    println!("rec_ao_special_linconv({})", r.name());
    0
}

// bi record

#[no_mangle]
extern fn rec_bi_init_record(rec: *mut biRecord) -> c_long {
    let mut r = BiRecord::new(unsafe { rec.as_mut().unwrap() });
    r.set_val(false);
    println!("rec_bi_init_record({}): {}", r.name(), r.val());
    0
}
#[no_mangle]
extern fn rec_bi_get_ioint_info(cmd: c_int, rec: *mut dbCommon) -> c_long {
    let r = Record::new(unsafe { rec.as_mut().unwrap() });
    println!("rec_bi_get_ioint_info({})", r.name());
    0
}
#[no_mangle]
extern fn rec_bi_read_bi(rec: *mut biRecord, after: c_int) -> c_long {
    let r = BiRecord::new(unsafe { rec.as_mut().unwrap() });
    println!("rec_bi_read_bi({})", r.name());
    0
}

// bo record

#[no_mangle]
extern fn rec_bo_init_record(rec: *mut boRecord) -> c_long {
    let mut r = BoRecord::new(unsafe { rec.as_mut().unwrap() });
    r.set_val(false);
    println!("rec_bo_init_record({}): {}", r.name(), r.val());
    0
}
#[no_mangle]
extern fn rec_bo_get_ioint_info(cmd: c_int, rec: *mut dbCommon) -> c_long {
    let r = Record::new(unsafe { rec.as_mut().unwrap() });
    println!("rec_bo_get_ioint_info({})", r.name());
    0
}
#[no_mangle]
extern fn rec_bo_write_bo(rec: *mut boRecord, after: c_int) -> c_long {
    let r = BoRecord::new(unsafe { rec.as_mut().unwrap() });
    println!("rec_bo_write_bo({}): {}", r.name(), r.val());
    0
}
