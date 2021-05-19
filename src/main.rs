use clap::{App, Arg};

fn main() {
    let matches = App::new("Block Tracer")
        .version("0.1.1")
        .author("Norman Kern <norman.kern@gmx.com>")
        .about("This is a rust version of blktrace.")
        .arg(
            Arg::with_name("dev")
                .short("d")
                .long("dev")
                .takes_value(true)
                .help("Use specified device. May also be given last after options"),
        )
        .arg(
            Arg::with_name("debugfs path")
                .short("r")
                .long("relay")
                .takes_value(true)
                .help("Path to mounted debugfs, defaults to /sys/kernel/debug"),
        )
        .arg(
            Arg::with_name("file")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("File(s) to send output to"),
        )
        .arg(
            Arg::with_name("dir")
                .short("D")
                .long("output-dir")
                .takes_value(true)
                .help("Directory to prepend to output file names"),
        )
        .get_matches();
}
