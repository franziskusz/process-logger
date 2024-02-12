use csv;
use std::process::Command;
use std::time::SystemTime;
use std::{error::Error, fs, fs::OpenOptions, io, io::Write, path::Path, process};
use sysinfo::System;

fn main() {
    //const PROCESS_NAME: &str = "DodgeRust";
    let mut sys = System::new();
    let mut n = 0;
    let logging_duration: u32;
    let mut timestamp_micros: u128;
    let mut path: String = "../process_stats/".to_owned();
    let duration_since_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let timestamp_secs: &str = &*duration_since_epoch.as_secs().to_string();
    let suffix = ".csv";

    let mut process_name = String::new();
    println!("Please enter the name of the process you wish to log.");
    io::stdin()
        .read_line(&mut process_name)
        .expect("Failed to read line");
    println!("You entered: {process_name}");
    let process_name_slice = process_name.as_str().trim(); //use trim to remove leading and trailing whitespace

    loop {
        println!("Please enter the desired logging duration in seconds:");
        let mut duration = String::new();
        io::stdin()
            .read_line(&mut duration)
            .expect("Failed to read Number");
        if let Ok(num) = duration.trim().parse::<u32>() {
            logging_duration = num;
            println!("logging for {logging_duration} seconds");
            break;
        } else {
            println!("Please enter a valid number.")
        }
    }

    path.push_str(process_name_slice);
    path.push_str("-");
    path.push_str(timestamp_secs);
    path.push_str(suffix);
    let path_str = path.as_str();

    if let Err(err) = make_stats_dir_if_not_exists() {
        print!("{}", err);
        process::exit(1);
    }

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true) //remove this option if file is ought to be truncated every run
        .open(path_str)
        .unwrap();

    let file_size = fs::metadata(path_str)
        .expect("file metadata not found")
        .len();

    let mut writer = csv::Writer::from_writer(file);

    if file_size == 0 {
        if let Err(err) = write_header(&mut writer) {
            print!("{}", err);
            process::exit(1);
        }
    }

    println!("timestamp, process_id, cpu_usage, memory, virtual_memory, read bytes, written bytes");

    while n < logging_duration {
        sys.refresh_processes();
        //let dodge_process = sys.processes_by_exact_name(PROCESS_NAME);

        let duration_since_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        timestamp_micros = duration_since_epoch.as_micros();

        for (pid, process) in sys.processes() {
            let process_name_var = process.name();
            //if let process_name_var = process.name() {
            match process_name_var {
                //println!("A loop {n}, {process_name_var}"); //debug
                process_name_var if process_name_var == process_name_slice => {
                    let cpu = process.cpu_usage();
                    let memory = process.memory();
                    let virtual_mem = process.virtual_memory();
                    let read_bytes = process.disk_usage().total_read_bytes;
                    let written_bytes = process.disk_usage().total_written_bytes;

                    let args = (
                        timestamp_micros,
                        cpu,
                        memory,
                        virtual_mem,
                        read_bytes,
                        written_bytes,
                    );

                    println!(
                        "{} [{pid}], {:?}%, {}, {}, {}, {}",
                        timestamp_micros,
                        process.cpu_usage(),
                        process.memory(),
                        process.virtual_memory(),
                        process.disk_usage().total_read_bytes,
                        process.disk_usage().total_written_bytes,
                    );

                    if let Err(err) = write_to_csv(&mut writer, args) {
                        print!("{}", err);
                        process::exit(1);
                    }
                }
                _ => {}
            }
        }

        let mut wait = Command::new("sleep").arg("1").spawn().unwrap();
        let _result = wait.wait().unwrap();

        n += 1;
        //println!("B loop {n}, {process_name_slice}");
    }
}

fn make_stats_dir_if_not_exists() -> Result<(), Box<dyn Error>> {
    let path = "../process_stats/";
    if !Path::new(path).exists() {
        fs::create_dir(path)?;
        Ok(())
    } else {
        Ok(())
    }
}

fn write_to_csv<W: Write>(
    writer: &mut csv::Writer<W>,
    args: (u128, f32, u64, u64, u64, u64),
) -> Result<(), Box<dyn Error>> {
    let (ts, cpu, mem, virtual_mem, read, written) = args;

    let writer_args = &[
        ts.to_string(),
        cpu.to_string(),
        mem.to_string(),
        virtual_mem.to_string(),
        read.to_string(),
        written.to_string(),
    ];

    writer.write_record(writer_args)?;

    writer.flush()?;
    Ok(())
}

fn write_header<W: Write>(writer: &mut csv::Writer<W>) -> Result<(), Box<dyn Error>> {
    let header = &[
        "timestamp",
        "cpu usage",
        "memory usage",
        "virtual memory usage",
        "read bytes",
        "written bytes",
    ];
    writer.write_record(header)?;
    writer.flush()?;
    Ok(())
}
