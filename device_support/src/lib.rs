use std::str::from_utf8;

use epics::{
    bind_device_support,
    register_command,
    record::*,
};


fn init() {
    println!("[devsup] init");
    register_command!(fn test_command(a: i32, b: f64, c: &str) {
        println!("[devsup] test_command({}, {}, {})", a, b, c);
    });
}

fn record_init(record: &mut AnyRecord) {
    println!("[devsup] record_init {}", from_utf8(record.name()).unwrap());
}
fn record_set_scan(record: &mut RecordBase, _scan: Scan) {
    println!("[devsup] record_set_scan {}", from_utf8(record.name()).unwrap());
}
fn record_read(record: &mut ReadRecord) {
    println!("[devsup] record_read {}", from_utf8(record.name()).unwrap());
}
fn record_write(record: &mut WriteRecord) {
    println!("[devsup] record_write {}", from_utf8(record.name()).unwrap());
}


bind_device_support!(
    init,
    record_init,
    record_set_scan,
    record_read,
    record_write
);
