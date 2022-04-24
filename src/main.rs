use std::{fs, process::Command};
fn main() {
    let distro = get_distro();
    println!("OS: {}", distro);
    println!("Total RAM (Gb): {}", get_total_ram());   
    println!("Avaible RAM (Gb): {}", get_available_ram());
    println!("Used RAM (Gb): {}", get_used_ram());
}

fn get_available_ram() -> String {
    let meminfo = fs::read_to_string("/proc/meminfo").unwrap();
    let lines: Vec<&str> = meminfo.split('\n').collect();
    let mut av = lines[2].split_whitespace().nth(1).unwrap().parse::<u64>().unwrap();
    // Convert to gigabytes
    av /= 1048576;
    return av.to_string().to_owned();
}

fn get_used_ram() -> String {
    let meminfo = fs::read_to_string("/proc/meminfo").unwrap();
    let lines: Vec<&str> = meminfo.split('\n').collect();
    let mut av = lines[6].split_whitespace().nth(1).unwrap().parse::<u64>().unwrap();
    // Convert to gigabytes
    av /= 1048576;
    return av.to_string().to_owned();
}

fn get_total_ram() -> String {
    let meminfo = fs::read_to_string("/proc/meminfo").unwrap();
    let lines: Vec<&str> = meminfo.split('\n').collect();
    let mut total_mem = lines[0].split_whitespace().nth(1).unwrap().parse::<u64>().unwrap();
    // Convert to gigabytes
    total_mem /= 1048576;
    return total_mem.to_string().to_owned();
}

fn get_distro() -> String {
    // Get the distro name from /etc/os-release
    let distro = std::fs::read_to_string("/etc/os-release").unwrap();
    let distroname = distro.lines().find(|line| line.starts_with("PRETTY_NAME=")).unwrap().to_owned();
    let prettyname = distroname.split("=").nth(1).unwrap().trim_matches('"').to_owned();
    return prettyname;
}
