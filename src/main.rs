use std::process;
use easy_uuid::{parse_uuid_count, run};

fn main() {
    let uuid_count = parse_uuid_count(&mut std::env::args()).unwrap_or_else(|err| {
        eprintln!("Program error: {err}");
        process::exit(1);
    });

    run(uuid_count);
}
