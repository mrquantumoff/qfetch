use std::{fs, process::Command, env};
use colored::*;
fn main() {
    let asciiart = get_ascii_art();
    if asciiart !="NOASCII" {
        println!("{}", asciiart.bright_cyan());
    }
    
    let ost = "OS: ".bright_yellow();
    let totmem = "Total RAM (Gb): ".bright_green();
    let avmem = "Available RAM (Gb): ".green();
    let usedmem = "Used RAM (Gb): ".yellow();
    let totalroot = "Total Root Partition (Gb): ".bright_green();
    let avroot = "Available Root Partition (Gb): ".green();
    let usedroot = "Used Root Partition (Gb): ".yellow();
    let cpu = "CPU: ".green();
    let desktop = "Desktop: ".green();
    let session_type = "Session Type: ".yellow();
    println!("{}","=======================".bright_red());
    println!("{ost}{}", get_distro());
    println!("{totmem}{}", get_total_ram());   
    println!("{avmem}{}", get_available_ram());
    println!("{usedmem}{}", get_used_ram());
    println!("{totalroot}{}", get_total_disk());
    println!("{avroot}{}", get_available_disk());
    println!("{usedroot}{}", get_used_disk());
    println!("{cpu}{}", get_cpu_name());
    println!("{desktop}{}", get_de());
    println!("{session_type}{}", get_session_type());
    println!("{}","=======================".bright_red());
}

fn get_session_type() -> String {
    let de = get_de();
    if de!="Unknown or tty"{
        let session_type = env::var("XDG_SESSION_TYPE").unwrap();
        if session_type!="" {
            return session_type;
        }    
        else{
            return "Unknown or tty".to_string();
        }
    }
    else {
        return "Unknown or tty".to_string();
    }
}

fn get_ascii_art() -> String {
    let homedir = env::var("HOME").unwrap_or("".to_string());
    let ascii_art_path=homedir+"/.config/qfetch/ascii_art.txt";
    if fs::metadata(ascii_art_path.clone()).is_ok() {
        let ascii_art = fs::read_to_string(ascii_art_path).expect("Error reading ascii art");
        return ascii_art;
    }
    else {
        return "NOASCII".to_owned();
    }
}


fn get_de() -> String {
    if env::var("XDG_CURRENT_DESKTOP").unwrap_or("".to_string())=="" {
        return "Unknown or tty".to_string();
    }
    else {
        return env::var("XDG_CURRENT_DESKTOP").unwrap_or("".to_string()); 
    }
}

fn get_cpu_name() -> String {
    let meminfo = fs::read_to_string("/proc/cpuinfo").unwrap();
    let lines: Vec<&str> = meminfo.split('\n').collect();
    let mut av = lines[4].split(" ").to_owned().collect::<Vec<&str>>();
    av.remove(0);
    av.remove(0);
    av.remove(0);
    let strav = av.join(" ");
    return strav;
}

fn get_used_disk() -> String {
    let output = Command::new("df")
        .arg("-h")
        .arg("/")
        .output()
        .expect("failed to execute process");
    let output_string = String::from_utf8_lossy(&output.stdout);
    let mut lines = output_string.lines();
    lines.next();
    let tt = lines.next().unwrap().split_whitespace().nth(2).unwrap().to_string();
    return tt.to_owned();  
}

fn get_available_disk() -> String {
    let output = Command::new("df")
        .arg("-h")
        .arg("/")
        .output()
        .expect("failed to execute process");
        let output_string = String::from_utf8_lossy(&output.stdout);
        let mut lines = output_string.lines();
        lines.next();
        let av = lines.next().unwrap().split_whitespace().nth(3).unwrap().to_string();
        return av.to_owned();
}

fn get_total_disk() -> String {
    let output = Command::new("df")
        .arg("-h")
        .arg("/")
        .output()
        .expect("failed to execute process");
    let output_string = String::from_utf8_lossy(&output.stdout);
    let mut lines = output_string.lines();
    lines.next();
    let tt = lines.next().unwrap().split_whitespace().nth(1).unwrap().to_string();
    return tt.to_owned();
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
