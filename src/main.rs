use std::io;
use std::env;
use regex::Regex;
use structopt::StructOpt;
use colored::Colorize;

// 設定程式基本訊息
#[derive(StructOpt, Debug)]
#[structopt(
    name = "bmapconv", 
    about = "BitMap(64位)轉換工具",
    author = "Nilm <nilm987521@gmail.com>",
    version = env!("VERGEN_GIT_SHA")
)]

// 設定可用的參數
struct Opt {
    /// 輸入16進位
    #[structopt(short = "H", long = "hex")]
    hex: Option<String>,

    /// 輸入2進位
    #[structopt(short = "B", long = "binary")]
    bin: Option<String>,

    /// 輸入選擇項
    #[structopt(short = "I", long = "indexes")]
    inds: Option<Vec<String>>,

}

// 靜態常數
lazy_static::lazy_static! {
    // 靜態常數，Regex模板
    static ref HEX_PATTERN: Regex = Regex::new(r"^[0-9a-fA-F]{16}$").unwrap();
    static ref BIN_PATTERN: Regex = Regex::new(r"^[01]{64}$").unwrap();
    static ref SEL_PATTERN: Regex = Regex::new(r"^([0-9]{1,2}\s){1,64}").unwrap();
}

/// 16進位轉2進位
fn hex_to_bin(arg: &str) -> String {
    let decimal_number = u64::from_str_radix(arg, 16).expect("Invalid hexadecimal input");
    let binary_number: String = format!("{:b}", decimal_number);
    binary_number
}

/// 2進位轉16進位
fn bin_to_hex(arg: &str) -> String {
    let decimal_number = u64::from_str_radix(arg, 2).expect("Invalid binary input");
    let hexadecimal_number: String = format!("{:0>16X}", decimal_number);
    hexadecimal_number
}

/// 2進位轉BitMap的index
fn bin_to_inds(binary_number: &str) -> String {
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
fn inds_to_bin(args: &str) -> String {
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

fn _convert_hex(hex: &str) {
    let bin = hex_to_bin(hex);
    println!("二進位: \t{}", bin.red());
    println!("選項: \t{}", bin_to_inds(&bin).yellow());
}

fn _convert_bin(bin: &str) {
    println!("十六進位: \t{}", bin_to_hex(bin).blue());
    println!("選項: \t{}", bin_to_inds(bin).yellow());
}

fn _convert_inds(sel: &str) {
    let bin = inds_to_bin(sel);
    println!("二進位: \t{}", bin.red());
    println!("十六進位: \t{}", bin_to_hex(&bin).blue());
}

/// 無限循環模式
fn _loop_mode() {
    loop {
        // 讀取使用者的輸入
        let mut input: String = String::new();
        println!("請輸入[{}]或[{}]或[{}]: ", "二進位字串".red(), "十六進位字串".blue(), "選項(多選項用空格分隔)".yellow());
        io::stdin().read_line(&mut input).expect("輸入錯誤");

        input = input.trim_end().to_string();
        // // 移除換行符號
        // if let Some('\n') = input.chars().next_back() {
        //     input.pop();
        // }

        // 輸入exit時結束
        if input.to_lowercase() == "exit" || input.to_lowercase() == "quit" || input == "結束" || input == "離開" {
            break;
        }

        // 依據輸入的格式
        if HEX_PATTERN.is_match(&input) {
            _convert_hex(&input);
        } else if BIN_PATTERN.is_match(&input) {
            _convert_bin(&input);
        } else if SEL_PATTERN.is_match(&input) {
            _convert_inds(&input);
        } else {
            print!("無效的輸入，請檢查長度及格式，");
        }

    }
}

fn main() {
    // args[0] 是自己的絕對路徑，所以跳過
    let args: Vec<String> = env::args().skip(1).collect();
    
    if args.len() > 0 {
        let opt = Opt::from_args();
        let mut act:bool = false;

        // if  有使用指定參數，並符合正規表達式
        if let Some(hex) = opt.hex.as_ref().filter(|&h| HEX_PATTERN.is_match(h.as_str())) {
            act = true;
            _convert_hex(hex);
        }

        if let Some(bin) = opt.bin.as_ref().filter(|&b| BIN_PATTERN.is_match(b.as_str())) {
            act = true;
            _convert_bin(bin);
        }

        if let Some(sel) = opt.inds.as_ref().filter(|&s| SEL_PATTERN.is_match(&s.join(" "))) {
            act = true;
            _convert_inds(&sel.join(" ").as_str());
        }

        if !act {
            println!("無效參數，請檢查長度及格式")
        }
    } else {
        _loop_mode();
    }
}

#[cfg(test)]
mod tests {
    use crate::{bin_to_hex, bin_to_inds, hex_to_bin};

    static HEX: &'static str = "F000000FF000000F";
    const BIN: &'static str = "1111000000000000000000000000111111110000000000000000000000001111";
    const INDS: &'static str = "1 2 3 4 29 30 31 32 33 34 35 36 61 62 63 64";
 
    #[test]
    fn test_hex() {
        assert_eq!(hex_to_bin(&HEX), BIN);
    }

    #[test]
    fn test_bin() {
        assert_eq!(bin_to_hex(&BIN), HEX);
    }

    #[test]
    fn test_inds() {
        assert_eq!(bin_to_inds(&BIN), INDS);
    }
}
