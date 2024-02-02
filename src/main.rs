use std::process::Command;
use std::time::SystemTime;
use sysinfo::System;

fn main() {
    let mut sys = System::new();
    let mut n = 0;
    let mut timestamp_nanos: u128;

    print!("timestamp, cpu1, cpu2, cpu3, cpu4, used memory, used swap\n");

    while n < 100 {
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
