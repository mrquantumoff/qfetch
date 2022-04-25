use std::{fs, process::Command, env};
use colored::*;
fn main() {
    let asciiart = get_ascii_art();
    if asciiart !="NOASCII" {
        println!("{}", asciiart.bright_cyan());
    }
    let mut showversion: bool = false;
    let args: Vec<String> = env::args().collect();
    for arg in args {
        if arg == "--help" || arg == "-h" {
            get_help();
            std::process::exit(0);
        }
        else if arg=="--version" || arg=="-v" {
            showversion = true;
        }
    }
    let style = get_style();
    if style == "default" {
        
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        let ost = "OS: ".magenta();
        let totmem = "Total RAM (Gb): ".blue();
        let avmem = "Available RAM (Gb): ".blue();
        let usedmem = "Used RAM (Gb): ".blue();
        let totalroot = "Total Root Partition (Gb): ".red();
        let avroot = "Available Root Partition (Gb): ".red();
        let usedroot = "Used Root Partition (Gb): ".red();
        let cpu = "CPU: ".magenta();
        let desktop = "Desktop: ".yellow();
        let session_type = "Session Type: ".yellow();
        let username = "Username: ".yellow();
        let editor = "Editor: ".yellow();
        let version = "Version: ".magenta();
        let kernel = "Kernel: ".magenta();
        let hostname = "Hostname: ".magenta();
        if showversion {println!("{}{}", version, VERSION);}
        println!("{}","=======================".green());
        println!("{ost}{}", get_distro());
        println!("{hostname} {}", get_hostname());
        println!("{cpu}{}", get_cpu_name());
        println!("{kernel}{}", get_kernel());
        println!("{totmem}{}", get_total_ram());   
        println!("{avmem}{}", get_available_ram());
        println!("{usedmem}{}", get_used_ram());
        println!("{totalroot}{}", get_total_disk());
        println!("{avroot}{}", get_available_disk());
        println!("{usedroot}{}", get_used_disk());    
        println!("{desktop}{}", get_de());
        println!("{session_type}{}", get_session_type());
        println!("{username}{}", get_user());
        println!("{editor}{}", get_editor());
        println!("{}","=======================".green());
        std::process::exit(0);
    }
    else if style == "column" {
        
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        let ost = "OS: ".magenta();
        let totmem = "Total RAM (Gb): ".blue();
        let avmem = "Available RAM (Gb): ".blue();
        let usedmem = "Used RAM (Gb): ".blue();
        let totalroot = "Total Root Partition (Gb): ".red();
        let avroot = "Available Root Partition (Gb): ".red();
        let usedroot = "Used Root Partition (Gb): ".red();
        let cpu = "CPU: ".magenta();
        let desktop = "Desktop: ".yellow();
        let session_type = "Session Type: ".yellow();
        let username = "Username: ".yellow();
        let editor = "Editor: ".yellow();
        let version = "Version: ".magenta();
        let kernel = "Kernel: ".magenta();
        let hostname = "Hostname: ".magenta();
        let col = "||".green();
        if showversion {println!("{}{}", version, VERSION);}
        println!("{}","*================================================*".green());
        println!("{col} {ost}{} {}", get_distro(), "");
        println!("{col} {hostname} {}", get_hostname());
        println!("{col} {cpu}{} {}", get_cpu_name(), "");
        println!("{col} {kernel} {} {}", get_kernel(), "");
        println!("{col} {totmem}{} {}", get_total_ram(), "");   
        println!("{col} {avmem}{} {}", get_available_ram(), "");
        println!("{col} {usedmem}{} {}", get_used_ram(), "");
        println!("{col} {totalroot}{} {}", get_total_disk(), "");
        println!("{col} {avroot}{} {}", get_available_disk(), "");
        println!("{col} {usedroot}{}", get_used_disk());    
        println!("{col} {desktop}{}", get_de());
        println!("{col} {session_type}{}", get_session_type());
        println!("{col} {username}{}", get_user());
        println!("{col} {editor}{}", get_editor());
        println!("{}","*================================================*".green());
    }
}

fn get_help() {
    let help = "
    Usage:
    -h, --help: Show this help message
    -v, --version: Show version in fetch info
    ";
    println!("{}", help);
}

fn get_editor() -> String {
    let editor = env::var("EDITOR");
    match editor {
        Ok(val) => val,
        Err(_) => "Not Set".to_string(),
    }
}

fn get_style() -> String {
    let homedir = env::var("HOME").unwrap_or("".to_string());
    let cfgpath=homedir+"/.config/qfetch/config.txt";
    if fs::metadata(cfgpath.clone()).is_ok() {
    let contents = fs::read_to_string(cfgpath).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    let style = lines[0].to_string();
    return style;
    }
    else {
        return "default".to_string();
    }
}

fn get_user() -> String {
    let de = get_de();
    if de!="Unknown or tty"{
        let user = env::var("USERNAME").unwrap();
        let user = user.to_string();
        return user;
    }
    else {
        return "Unknown".to_string();
    }
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

fn get_kernel() -> String {
    let output = Command::new("uname")
        .arg("-r")
        .output()
        .expect("failed to execute process");
    let output_string = String::from_utf8_lossy(&output.stdout);
    let mut lines = output_string.lines();
    let tt = lines.next().unwrap().to_string();
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

fn get_hostname() -> String {
    if fs::metadata("/etc/hostname").is_ok() {
        let mut res = fs::read_to_string("/etc/hostname").unwrap();
        let rs: Vec<&str> = res.split("\n").collect();
        res = rs[0].to_string();
        return res.to_string();
    }
    else {
        return "No hostname set in /etc/hostname".to_string();
    }
}

fn get_available_ram() -> String {
    let meminfo = fs::read_to_string("/proc/meminfo").unwrap();
    let lines: Vec<&str> = meminfo.split('\n').collect();
    let mut av = lines[2].split_whitespace().nth(1).unwrap().parse::<f64>().unwrap();
    // Convert to gigabytes
    av /= 1048576.0;
    let y = (av * 100.0).round() / 100.0;
    return y.to_string().to_owned()+ " Gb";
}

fn get_used_ram() -> String {
    let meminfo = fs::read_to_string("/proc/meminfo").unwrap();
    let lines: Vec<&str> = meminfo.split('\n').collect();
    let mut av = lines[6].split_whitespace().nth(1).unwrap().parse::<f64>().unwrap();
    // Convert to gigabytes
    av /= 1048576.0;
    let y = (av * 100.0).round() / 100.0;
    return y.to_string().to_owned()+ " Gb";
}

fn get_total_ram() -> String {
    let meminfo = fs::read_to_string("/proc/meminfo").unwrap();
    let lines: Vec<&str> = meminfo.split('\n').collect();
    let mut total_mem = lines[0].split_whitespace().nth(1).unwrap().parse::<f64>().unwrap();
    // Convert to gigabytes
    total_mem /= 1048576.0;
    let y = (total_mem * 100.0).round() / 100.0;
    return y.to_string().to_owned()+ " Gb";
}

fn get_distro() -> String {
    // Get the distro name from /etc/os-release
    let distro = std::fs::read_to_string("/etc/os-release").unwrap();
    let distroname = distro.lines().find(|line| line.starts_with("PRETTY_NAME=")).unwrap().to_owned();
    let prettyname = distroname.split("=").nth(1).unwrap().trim_matches('"').to_owned();
    return prettyname;
}
