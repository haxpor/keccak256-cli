use clap::Parser;

/// Struct that holds the command line's arguments.
#[derive(Debug, Parser)]
#[clap(author="Wasin Thonkaew (wasin@wasin.io)")]
#[clap(name="keccak256")]
#[clap(about="keccak256 encryption cli program; accepts input text from stdin. Suitable for commandline users.")]
pub struct CommandlineArgs {
    /// Output as method id; only first 4 bytes are output.
    #[clap(long="method-id")]
    pub is_method_id_output: bool,

    /// Not include prefixed of '0x' for output.
    #[clap(long="no-0x")]
    pub no_0x: bool,

    /// Output encoded result separating line by line.
    /// It is usually suitable when there is multiple lines as input text but
    /// each one is independent to each other; especially input from file.
    ///
    /// It might be beneficial to combine with '--include-input` flag for ease
    /// of parsing the output by also include the input text.
    #[clap(long="each-line")]
    pub each_line_output: bool,

    /// Also show input string in fully parsable output format.
    /// An input text follows by a single space, then follow by an output text.
    /// e.g. input_text output_text
    ///
    /// This flag only allowed to be used only when there is only one line of
    /// input text. This is by designed and it makes more sense to use this flag
    /// for one-line input only.
    #[clap(long="include-input")]
    pub is_include_input: bool,
}
