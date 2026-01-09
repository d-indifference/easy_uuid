use std::env::Args;
use uuid::Uuid;

const INVALID_ARGUMENTS_COUNT: &str = "Invalid arguments count. Enter the count of UUIDs you want to generate as an argument, or do not enter any arguments if you want to generate only one UUID.";
const PARSE_UUID_COUNT_ERROR: &str = "Parsing of UUID count error: ";

pub fn parse_uuid_count(args: &mut Args) -> Result<u32, String> {
    if args.len() > 2 {
        return Err(INVALID_ARGUMENTS_COUNT.to_string())
    }

    args.next();

    let uuid_count = match args.next() {
        None => { 1 },
        Some(count) => {
            match count.parse::<u32>() {
                Ok(count) => { count },
                Err(e) => {
                    return Err(format!("{PARSE_UUID_COUNT_ERROR}{e}").to_string());
                }
            }
        }
    };

    Ok(uuid_count)
}

pub fn run(uuid_count: u32) {
    for _ in 0..uuid_count {
        println!("{}", Uuid::new_v4());
    }
}