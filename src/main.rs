use std::process::Command;
use std::time::SystemTime;
use std::{error::Error, fs, fs::OpenOptions, path::Path, process};
use sysinfo::System;

use csv;

fn main() {
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

    print!("timestamp, cpu1, cpu2, cpu3, cpu4, used memory, used swap\n");

    while n < 10 {
        sys.refresh_cpu();
        sys.refresh_memory();
        //let components = Components::new_with_refreshed_list();

        let duration_since_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        timestamp_nanos = duration_since_epoch.as_nanos();

        print!("{},", timestamp_nanos);
        for cpu in sys.cpus() {
            print!(" {}%,", cpu.cpu_usage());
        }

        print!(" {},", sys.used_memory());
        print! {" {}", sys.used_swap()};

        //for component in &components {
        //    print!("{component:?}");
        //}
        print!("\n");

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
    let path = "../system_stats/stats.csv";

    //godot_print!("{}", path_globalised); debug

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
