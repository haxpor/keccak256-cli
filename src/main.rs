use clap::Parser;
use std::io::{self, BufRead};
use sha3::{Digest, Keccak256};
use std::fmt::Write;

#[derive(Debug, Parser)]
#[clap(author="Wasin Thonkaew (wasin@wasin.io)")]
#[clap(name="keccak256")]
#[clap(about="keccak256 encryption cli program; accepts input text from stdin. Suitable for commandline users.")]
struct CommandlineArgs {
    /// Output as method id; only first 4 bytes are output.
    #[clap(long="method-id")]
    is_method_id_output: bool,

    /// Not include prefixed of '0x' for output.
    #[clap(long="no-0x")]
    no_0x: bool,
}

fn main() {
    let cmd_args = CommandlineArgs::parse();

    // read input from stdin
    // then form into a single long line of string
    let stdin = io::stdin();
    let mut lines_iter = stdin.lock().lines();
    let mut is_first_line = true;
    let mut input_string = String::new();

    loop {
        match lines_iter.next() {
            Some(line_res) => {
                match line_res {
                    Ok(line) => {
                        if is_first_line {
                            input_string.push_str(line.as_str());
                            is_first_line = false;   
                        }
                        else {
                            input_string.push('\n');
                            input_string.push_str(line.as_str());
                        }
                    },
                    Err(e) => {
                        eprintln!("Error reading input; err={}", e);
                        std::process::exit(1);
                    }
                }
            },
            None => break,
        }
    }

    // encrypt input string
    let mut hasher = Keccak256::new();
    hasher.update(input_string.as_bytes());
    let encrypted_bytes = hasher.finalize();

    // convert from bytes array into hex-string
    let mut encrypted_s = String::with_capacity(2 * encrypted_bytes.len());
    for byte in encrypted_bytes {
        _ = write!(encrypted_s, "{:02x}", byte);
    }

    // output as per arguments supplied at command line
    if cmd_args.is_method_id_output {
        if cmd_args.no_0x {
            println!("{}", &encrypted_s[..4*2]);
        }
        else {
            println!("0x{}", &encrypted_s[..4*2]);
        }
    }
    else {
        if cmd_args.no_0x {
            println!("{}", encrypted_s);
        }
        else {
            println!("0x{}", encrypted_s);
        }
    }
}
