use rand::Rng;
use sha2::{Sha256, Digest};
use termion::{color, style};
use std::thread;
use std::time::Duration;
use std::io;
use clearscreen::clear;
use std::process;

fn main() {

    clear();
    print!("{}", color::Fg(color::Cyan));
    println!("Welcome to CryptoX!");
    print!("{}", color::Fg(color::Cyan));
    println!("Loading...");
    thread::sleep(Duration::from_secs(2));

    clear();

    print!("{}", color::Fg(color::Cyan));
    println!("Insert your wallet: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    
    if input.len() != 65 {
        println!("Error: The input must have exactly 64 characters.");
        println!("{}", input.len());
        process::exit(1);
    }

    let mut num = true;
    let mut rng = rand::thread_rng();

    while num {
        let random_number: u64 = rng.gen_range(0..=999_999_999);
        let hash = hash_number(random_number);

        // Print the hash with cyan color
        print!("{}", color::Fg(color::Red));
        println!("BTC ---> 0.00000000 {}", hash);
        print!("{}", style::Reset);

        if random_number == 76 {
            num = false;
        }
    }
    let random_number: u64 = rng.gen_range(0..=1_200);
    let hash = hash_number(random_number);

    let mut rng = rand::thread_rng();
    let random_short: f64 = rng.gen_range(0.000001..=0.00016);
    print!("{}", color::Fg(color::Green));
    print!("BTC FOUND! ---> {}", random_short);
    print!("{}", color::Fg(color::Green));
    println!(" {}", hash);
    print!("{}", color::Fg(color::Green));
    println!("Transferring to wallet: {}", input);
    print!("{}", color::Fg(color::Green));
    println!("Please wait...");
    thread::sleep(Duration::from_secs(rng.gen_range(0..=1_574)));
    print!("{}", color::Fg(color::Green));
    println!("BTC transferred!");

}

fn hash_number(number: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(number.to_string());
    let result = hasher.finalize();
    format!("{:x}", result)
}

#[cfg(target_os = "windows")]
fn set_console_title(title: &str) {
    use winapi::um::wincon::{SetConsoleTitleW, WCHAR};
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    let wide_title: Vec<WCHAR> = OsStr::new(title).encode_wide().chain(Some(0).into_iter()).collect();
    unsafe {
        SetConsoleTitleW(wide_title.as_ptr());
    }
}

#[cfg(target_os = "linux")]
#[cfg(target_os = "macos")]
fn set_console_title(title: &str) {
    use std::ffi::CString;
    use libc::{c_char, STDOUT_FILENO, TIOCSCTTY};

    let c_title = CString::new(title).expect("CString conversion failed");
    unsafe {
        libc::ioctl(STDOUT_FILENO, TIOCSCTTY, c_title.as_ptr() as *mut c_char);
    }
}


