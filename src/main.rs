use std::io;
use std::env;
use regex::Regex;
use structopt::StructOpt;
use colored::*;

// 設定可用的參數
#[derive(StructOpt, Debug)]
#[structopt(
    name = "bmapconv", 
    about = "BitMap(64位)轉換工具",
    author = "Daniel Lan <daniellan@aitc.com.tw>",
)]
struct Opt {
    /// 輸入16進位
    #[structopt(short = "H", long = "hex")]
    hex: Option<String>,

    /// 輸入2進位
    #[structopt(short = "B", long = "binary")]
    bin: Option<String>,

    /// 輸入選擇項
    #[structopt(short = "S", long = "select")]
    sel: Option<Vec<String>>,

    // #[structopt(short = "h", long = "help")]
    // help: bool
}

// 靜態常數
lazy_static::lazy_static! {
    // Regex模板
    static ref HEX_PATTERN: Regex = Regex::new(r"^[0-9a-fA-F]{16}$").unwrap();
    static ref BIN_PATTERN: Regex = Regex::new(r"^[01]{64}$").unwrap();
    static ref SEL_PATTERN: Regex = Regex::new(r"^([0-9]{1,2}\s){1,64}").unwrap();
}

/// 16進位轉2進位
fn hex_to_bin(arg: &str) -> String {
    let decimal_number = u64::from_str_radix(arg, 16)
        .expect("Invalid hexadecimal input");
    let binary_number: String = format!("{:b}", decimal_number);
    binary_number
}

/// 2進位轉16進位
fn bin_to_hex(arg: &str) -> String {
    let decimal_number = u64::from_str_radix(arg, 2)
        .expect("Invalid binary input");
    let hexadecimal_number: String = format!("{:0>16X}", decimal_number);
    hexadecimal_number
}

/// 2進位轉BitMap的index
fn bin_to_select(binary_number: &str) -> String {
    let mut sel_vec: Vec<String> = Vec::new();
    let bytes: &[u8] = binary_number.as_bytes();

    for i in 0..bytes.len() {
        if bytes[i] == b'1' {
            sel_vec.push((i + 1).to_string());
        }
    }
    sel_vec.join(" ")
}

/// BitMap的index轉2進位
fn select_to_bin(args: &str) -> String {
    let mut vec_of_zeros: Vec<&str> = vec!["0"; 64];
    for arg in args.split_whitespace() {
        let index: usize = arg.parse().expect("Error parsing integer");
        if let Some(element) = vec_of_zeros.get_mut(index - 1){
            *element = "1";
        } else {
            println!("Index out of bounds");
        }
    }
    vec_of_zeros.join("")
}

/// 無限循環模式
fn _loop_mode() {
    loop {
        // 讀取使用者的輸入
        let mut input: String = String::new();
        println!("Enter a hexadecimal number or a binary number:");
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // 移除換行符號
        if let Some('\n') = input.chars().next_back() {
            input.pop();
        }

        // 輸入exit時結束
        if input.to_lowercase() == "exit" {
            break;
        }

        // 依據輸入的格式
        if HEX_PATTERN.is_match(&input) {
            let bin = hex_to_bin(&input);
            println!("Binary: \t{}", bin.red());
            println!("Select: \t{}", bin_to_select(&bin).yellow());
        } else if BIN_PATTERN.is_match(&input) {
            println!("Hex: \t{}", bin_to_hex(&input).blue());
            println!("Select: \t{}", bin_to_select(&input).yellow());
        } else if SEL_PATTERN.is_match(&input) {
            let bin = select_to_bin(&input);
            println!("Binary: \t{}", bin.red());
            println!("Hex: \t{}", bin_to_hex(&bin).blue());
        } else {
            println!("Invalid input. Please enter a hexadecimal or binary number.");
        }

    }
}

fn main() {
    // args[0] 是自己的絕對路徑，所以跳過
    let args: Vec<String> = env::args().skip(1).collect();
    
    if args.len() > 0 {
        let opt = Opt::from_args();

        // if opt.help {
        //     println!("AAAAAAAA");
        //     return;
        // }

        if let Some(hex) = opt.hex.as_ref().filter(|&h| HEX_PATTERN.is_match(h.as_str())) {
            let bin: String = hex_to_bin(hex);
            println!("Hex: {}", bin.red());
            println!("Select: {}", bin_to_select(&bin).yellow());
        }

        if let Some(bin) = opt.bin.as_ref().filter(|&b| BIN_PATTERN.is_match(b.as_str())) {
            println!("Hex: {}", bin_to_hex(&bin).blue());
            println!("Select: {}", bin_to_select(&bin).yellow());
        }

        if let Some(sel) = opt.sel.as_ref().filter(|&s| SEL_PATTERN.is_match(&s.join(" "))) {
            let bin: String = select_to_bin(&sel.join(" "));
            println!("Binary: {}", bin.red());
            println!("Hex: {}", bin_to_hex(&bin).blue());
        }
    } else {
        _loop_mode();
    }
}
