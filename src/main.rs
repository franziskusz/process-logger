use std::process::Command;
use std::time::SystemTime;
use sysinfo::{Cpu, System};

fn main() {
    let mut sys = System::new();
    let mut n = 0;
    let mut timestamp_nanos: u128;

    while n < 10 {
        sys.refresh_cpu();

        let duration_since_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        timestamp_nanos = duration_since_epoch.as_nanos();

        print!("{},", timestamp_nanos);
        for cpu in sys.cpus() {
            print!(" {}%,", cpu.cpu_usage());
        }
        print!("\n");

        let mut wait = Command::new("sleep").arg("1").spawn().unwrap();
        let _result = wait.wait().unwrap();

        n += 1
    }
}
