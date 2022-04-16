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

    /// Output encoded result separating line by line.
    /// It is usually suitable when there is multiple lines as input text but
    /// each one is independent to each other; especially input from file.
    ///
    /// It might be beneficial to combine with '--include-input` flag for ease
    /// of parsing the output by also include the input text.
    #[clap(long="each-line")]
    each_line_output: bool,

    /// Also show input string in fully parsable output format.
    /// An input text follows by a single space, then follow by an output text.
    /// e.g. input_text output_text
    ///
    /// This flag only allowed to be used only when there is only one line of
    /// input text. This is by designed and it makes more sense to use this flag
    /// for one-line input only.
    #[clap(long="include-input")]
    is_include_input: bool,
}

fn main() {
    let cmd_args = CommandlineArgs::parse();

    // encode input text according to the configurations applied from command line
    // 1. encode line by line
    if cmd_args.each_line_output {
        let stdin = io::stdin();
        let mut lines_iter = stdin.lock().lines();

        loop {
            match lines_iter.next() {
                Some(line_res) => {
                    match line_res {
                        Ok(line) => {
                            // encode the input text
                            // NOTE: I don't like this line much as we need
                            // to incur cost in creating hasher every new line
                            // but due to it doesn't implement Copy trait.
                            let mut hasher = Keccak256::new();
                            hasher.update(line.as_bytes());
                            let encrypted_bytes = hasher.finalize();

                            let mut encrypted_s = String::with_capacity(2 * encrypted_bytes.len());
                            for byte in encrypted_bytes {
                                _ = write!(encrypted_s, "{:02x}", byte);
                            }

                            // output as per arguments supplied at command line
                            if cmd_args.is_method_id_output {
                                if cmd_args.no_0x {
                                    if cmd_args.is_include_input {
                                        println!("{} {}", line, &encrypted_s[..4*2]);
                                    }
                                    else {
                                        println!("{}", &encrypted_s[..4*2]);
                                    }
                                }
                                else {
                                    if cmd_args.is_include_input {
                                        println!("{} 0x{}", line, &encrypted_s[..4*2]);
                                    }
                                    else {
                                        println!("0x{}", &encrypted_s[..4*2]);
                                    }
                                }
                            }
                            else {
                                if cmd_args.no_0x {
                                    if cmd_args.is_include_input {
                                        println!("{} {}", line, encrypted_s);
                                    }
                                    else {
                                        println!("{}", encrypted_s);
                                    }
                                }
                                else {
                                    if cmd_args.is_include_input {
                                        println!("{} 0x{}", line, encrypted_s);
                                    }
                                    else {
                                        println!("0x{}", encrypted_s);
                                    }
                                }
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
    }
    // 2. encode the whole input lines
    else {
        // read input from stdin
        // then form into a single long line of string
        let stdin = io::stdin();
        let mut lines_iter = stdin.lock().lines();
        let mut is_first_line = true;
        let mut input_string = String::new();
        let mut len = 0_usize;

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

                            len = len + 1;
                        },
                        Err(e) => {
                            eprintln!("Error reading input; err={}", e);
                            std::process::exit(1);
                        }
                    }
                },
                None => {
                    break;
                },
            }
        }

        // check if --include-input is used on multiple lines input
        // input with length 0 is still possible to be encoded with keccak256
        if len > 1 && cmd_args.is_include_input{
            eprintln!("Error, cannot use --include-input on multiple lines input.
This flag is meant to be used for a single line text input only.");
            std::process::exit(1);
        }

        // encrypt input string
        let mut hasher = Keccak256::new();
        hasher.update(input_string.as_bytes());
        let encrypted_bytes = hasher.finalize();

        // convert from bytes array into hex-string
        let mut encrypted_s = String::with_capacity(2 * encrypted_bytes.len());
        for byte in encrypted_bytes {
            // FIXME: check the retunred result...
            _ = write!(encrypted_s, "{:02x}", byte);
        }

        // output as per arguments supplied at command line
        if cmd_args.is_method_id_output {
            if cmd_args.no_0x {
                if cmd_args.is_include_input {
                    println!("{} {}", input_string, &encrypted_s[..4*2]);
                }
                else {
                    println!("{}", &encrypted_s[..4*2]);
                }
            }
            else {
                if cmd_args.is_include_input {
                    println!("{} 0x{}", input_string, &encrypted_s[..4*2]);
                }
                else {
                    println!("0x{}", &encrypted_s[..4*2]);
                }
            }
        }
        else {
            if cmd_args.no_0x {
                if cmd_args.is_include_input {
                    println!("{} {}", input_string, encrypted_s);
                }
                else {
                    println!("{}", encrypted_s);
                }
            }
            else {
                if cmd_args.is_include_input {
                    println!("{} 0x{}", input_string, encrypted_s);
                }
                else {
                    println!("0x{}", encrypted_s);
                }
            }
        }
    }
}
