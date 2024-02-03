use csv;
use std::process::Command;
use std::time::SystemTime;
use std::{error::Error, fs, fs::OpenOptions, path::Path, process};
use sysinfo::System;

fn main() {
    const PROCESS_NAME: &str = "DodgeRust";
    let mut sys = System::new();
    let mut n = 0;
    let mut timestamp_nanos: u128;

    if let Err(err) = make_stats_dir_if_not_exists() {
        print!("{}", err);
        process::exit(1);
    }

    if let Err(err) = write_to_csv() {
        print!("{}", err);
        process::exit(1);
    }

    println!("timestamp, process_id, cpu_usage, memory, virtual_memory, read bytes, written bytes");

    while n < 100 {
        sys.refresh_cpu();
        sys.refresh_memory();
        sys.refresh_processes();
        //let dodge_process = sys.processes_by_exact_name(PROCESS_NAME);

        let duration_since_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        timestamp_nanos = duration_since_epoch.as_nanos();

        for (pid, process) in sys.processes() {
            if process.name() == PROCESS_NAME {
                println!(
                    "{} [{pid}], {:?}%, {}, {}, {}, {}",
                    timestamp_nanos,
                    process.cpu_usage(),
                    process.memory(),
                    process.virtual_memory(),
                    process.disk_usage().total_read_bytes,
                    process.disk_usage().total_written_bytes,
                );
            }
        }

        let mut wait = Command::new("sleep").arg("1").spawn().unwrap();
        let _result = wait.wait().unwrap();

        n += 1
    }
}

fn make_stats_dir_if_not_exists() -> Result<(), Box<dyn Error>> {
    let path = "../system_stats/";
    if !Path::new(path).exists() {
        fs::create_dir(path)?;
        Ok(())
    } else {
        Ok(())
    }
}

fn write_to_csv() -> Result<(), Box<dyn Error>> {
    //TODO: move writer declaration and initialisation to main
    //TODO: then call this function from while loop in main
    let path = "../system_stats/stats.csv";

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true) //remove this option if file is ought to be truncated every run
        .open(path)
        .unwrap();
    let mut writer = csv::Writer::from_writer(file);

    let test_arg = 42;

    let args = &[
        test_arg.to_string(),
        test_arg.to_string(),
        test_arg.to_string(),
        test_arg.to_string(),
        test_arg.to_string(),
        test_arg.to_string(),
    ];

    writer.write_record(args)?;

    writer.flush()?;
    Ok(())
    //write local vars to csv file
}
