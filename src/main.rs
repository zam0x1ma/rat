use clap::Parser;
use std::path::PathBuf;

/// Concatenate FILE(s) to standard output.
///
/// With no FILE, or when FILE is -, read standard input.
#[derive(Parser, Debug)]
#[clap(about)]
struct Args {
    /// Equivalent to -vET
    #[clap(short = 'A', long)]
    show_all: bool,

    /// Number nonempty output lines, overrides -n
    #[clap(short = 'b', long)]
    number_nonblank: bool,

    /// Equivalent to -vE
    #[clap(short = 'e')]
    show_nonprinting_ends: bool,

    /// Display $ at end of each line
    #[clap(short = 'E', long)]
    show_ends: bool,

    /// Number all output lines
    #[clap(short, long)]
    number: bool,

    /// Suppress repeated empty output lines
    #[clap(short, long)]
    squeeze_blank: bool,

    /// Equivalent to -vT
    #[clap(short = 't')]
    show_nonprinting_tabs: bool,

    /// Display TAB characters as ^I
    #[clap(short = 'T', long)]
    show_tabs: bool,

    /// (ignored)
    #[clap(short = 'u')]
    u: bool,

    /// Use ^ and M- notation, except for LFD and TAB
    #[clap(short = 'v', long)]
    show_nonprinting: bool,

    #[clap(required = true, parse(from_os_str), value_name = "FILE")]
    files: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
