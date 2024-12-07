use sysinfo::System;

pub fn print_system_info() {
    let mut sys = System::new_all();
    sys.refresh_all();
    println!("OS: {:?}", System::name().unwrap());
    println!(
        "System host name:        {:?}",
        System::host_name().unwrap()
    );
    println!("NB CPUs: {}", sys.cpus().len());
}
