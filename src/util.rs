use crate::types::CommandlineArgs;
use std::io::{self, BufRead};
use sha3::{Digest, Keccak256};
use std::fmt::Write;

/// Handle all input lines as a whole.
/// It will exit with status code of 1 if there the following event occurs
/// - reading a line from `stdin`
/// - encoding input text
///
/// when error occurs, it will print error message accordingly.
///
/// # Arguments
/// * `cmd_args` - `CommandlineArgs`
pub fn handle_whole(cmd_args: &CommandlineArgs) {
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
        let res = write!(encrypted_s, "{:02x}", byte);
        if res.is_err() {
            eprintln!("Error encoding; err={}", res.unwrap_err());
            std::process::exit(1);
        }
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

/// Handle each line independently.
///
/// It will exit with status code of 1 if the following happens
/// - reading a line from `stdin`
///
/// Anyway if error occurs with encoding, it will not exit the process but
/// only print the error message. This is to allow other encoding result on
/// other lines to be able to continue.
///
/// # Arguments
/// * `cmd_args` - `CommandlineArgs`
pub fn handle_each_line(cmd_args: &CommandlineArgs) {
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
                        let mut has_error = false;
                        let mut error: Option<std::fmt::Error> = None;

                        for byte in encrypted_bytes {
                            let res = write!(encrypted_s, "{:02x}", byte);
                            if res.is_err() {
                                // NOTE: if error happens, we don't error
                                // out and quit now, but will print error
                                // along each line of output
                                error = Some(res.unwrap_err());
                                has_error = true;
                            }
                        }

                        // output as per arguments supplied at command line
                        if cmd_args.is_method_id_output {
                            if cmd_args.no_0x {
                                if has_error {
                                    if cmd_args.is_include_input {
                                        println!("{} 'Error encoding; err={}'", line, error.unwrap());
                                    }
                                    else {
                                        println!("'Error encoding; err={}'", error.unwrap());
                                    }
                                }
                                else {
                                    if cmd_args.is_include_input {
                                        println!("{} {}", line, &encrypted_s[..4*2]);
                                    }
                                    else {
                                        println!("{}", &encrypted_s[..4*2]);
                                    }
                                }
                            }
                            else {
                                if has_error {
                                    if cmd_args.is_include_input {
                                        println!("{} 'Error encoding; err={}'", line, error.unwrap());
                                    }
                                    else {
                                        println!("'Error encoding; err={}'", error.unwrap());
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
                        }
                        else {
                            if cmd_args.no_0x {
                                if has_error {
                                    if cmd_args.is_include_input {
                                        println!("{} 'Error encoding; err={}'", line, error.unwrap());
                                    }
                                    else {
                                        println!("'Error encoding; err={}'", error.unwrap());
                                    }
                                }
                                else {
                                    if cmd_args.is_include_input {
                                        println!("{} {}", line, encrypted_s);
                                    }
                                    else {
                                        println!("{}", encrypted_s);
                                    }
                                }
                            }
                            else {
                                if has_error {
                                    if cmd_args.is_include_input {
                                        println!("{} 'Error encoding; err={}'", line, error.unwrap());
                                    }
                                    else {
                                        println!("'Error encoding; err={}'", error.unwrap());
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
