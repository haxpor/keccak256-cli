mod types;
mod util;

use types::CommandlineArgs;
use clap::Parser;

fn main() {
    let cmd_args = CommandlineArgs::parse();

    // encode input text according to the configurations applied from command line
    // 1. encode line by line
    if cmd_args.each_line_output {
        util::handle_each_line(&cmd_args);
    }
    // 2. encode the whole input lines
    else {
        util::handle_whole(&cmd_args);
    }
}
