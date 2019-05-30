use std::str::from_utf8;

use epics::{
    bind_device_support,
    register_command,
    record::*,
    context::*,
};


fn init(context: &mut InitContext) {
    println!("[devsup] init");
    register_command!(context, fn test_command(a: i32, b: f64, c: &str) {
        println!("[devsup] test_command({}, {}, {})", a, b, c);
    });
}
fn quit(_context: &mut QuitContext) {
    println!("[devsup] quit");
}
fn record_init(_context: &mut RecInitContext, record: &mut AnyRecord) {
    println!("[devsup] record_init {}", from_utf8(record.name()).unwrap());
}
fn record_set_scan(_context: &mut RecScanContext, record: &mut RecordBase, _scan: Scan) {
    println!("[devsup] record_set_scan {}", from_utf8(record.name()).unwrap());
}
fn record_read(context: &mut RecRdContext, record: &mut ReadRecord) {
    println!("[devsup] record_read {}", from_utf8(record.name()).unwrap());
    context.request_async(record);
}
fn record_write(context: &mut RecWrContext, record: &mut WriteRecord) {
    println!("[devsup] record_write {}", from_utf8(record.name()).unwrap());
    context.request_async(record);
}
fn record_read_async(_context: &mut RecRdAContext, record: &mut ReadRecord) {
    println!("[devsup] record_read_async {}", from_utf8(record.name()).unwrap());
}
fn record_write_async(_context: &mut RecWrAContext, record: &mut WriteRecord) {
    println!("[devsup] record_write_async {}", from_utf8(record.name()).unwrap());
}

bind_device_support!(
    init,
    quit,
    record_init,
    record_set_scan,
    record_read,
    record_write,
    record_read_async,
    record_write_async,
);
