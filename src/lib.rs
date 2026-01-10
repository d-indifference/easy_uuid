use std::env::Args;
use std::error::Error;
use uuid::Uuid;
use std::thread;

const INVALID_ARGUMENTS_COUNT: &str = "Invalid arguments count. Enter the count of UUIDs you want to generate as an argument, or do not enter any arguments if you want to generate only one UUID.";
const PARSE_UUID_COUNT_ERROR: &str = "Parsing of UUID count error: ";
const THREAD_PANICKED: &str = "Thread panicked: ";
const THREAD_JOINING_ERROR: &str = "Thread joining error: ";
const MULTITHREAD_MODE_ACTIVATES_ON: u32 = 1000;

pub fn parse_uuid_count(args: &mut Args) -> Result<u32, String> {
    if args.len() > 2 {
        return Err(INVALID_ARGUMENTS_COUNT.to_string())
    }

    args.next();

    let uuid_count = match args.next() {
        None => { 1 },
        Some(count) => {
            count.parse::<u32>()
                .map_err(|err| { format!("{PARSE_UUID_COUNT_ERROR}{err}") })?
        }
    };

    Ok(uuid_count)
}

pub fn run(uuid_count: u32) -> Result<(), Box<dyn Error>> {
    if uuid_count > MULTITHREAD_MODE_ACTIVATES_ON {
        generate_parallel(uuid_count)
    } else {
        generate_sequential(uuid_count)
    }
}

pub fn generate_sequential(uuid_count: u32) -> Result<(), Box<dyn Error>> {
    for _ in 0..uuid_count {
        println!("{}", Uuid::new_v4());
    }

    Ok(())
}

pub fn generate_parallel(uuid_count: u32) -> Result<(), Box<dyn Error>>  {
    let usize_uuid_count: &usize = &(uuid_count as usize);

    let num_threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
        .min(*usize_uuid_count);

    let uuids_per_threads = *usize_uuid_count / num_threads;
    let remainder = *usize_uuid_count % num_threads;

    let mut handlers: Vec<_> = Vec::with_capacity(num_threads);

    let (tx, rx) = std::sync::mpsc::channel::<String>();

    for thread_id in 0..num_threads {
        let thread_tx = tx.clone();
        let mut count = uuids_per_threads;

        if thread_id == num_threads - 1 {
            count += remainder;
        }

        handlers.push(thread::spawn(move || -> Result<(), Box<dyn Error + Send>> {
            for _ in 0..count {
                let uuid = Uuid::new_v4().to_string();

                thread_tx.send(uuid).ok();
            }
            Ok(())
        }));
    }

    drop(tx);

    for uuid in rx {
        println!("{uuid}");
    }

    for handler in handlers {
        match handler.join() {
            Ok(Ok(_)) => {}
            Ok(Err(e)) => {
                return Err(format!("{THREAD_PANICKED}{:?}", e).into());
            },
            Err(e) => {
                return Err(format!("{THREAD_JOINING_ERROR}{:?}", e).into());
            }
        }
    }

    Ok(())
}