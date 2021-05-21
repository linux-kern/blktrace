use clap::{App, Arg, ArgMatches};
use std::fs::File;

fn handle_args<'a>(ctx: &'a mut BlktraceCtx) {
    let blktrace_about = format!(
        "\n{} \n\t{}",
        "THIS IS A RUST VERSION OF BLKTRACE WRITEN BY", "Jens Axboe <axboe@kernel.dk>"
    );

    ctx.matches = App::new("Block IO Tracing")
        .bin_name("blktrace")
        .version("2.0.0")
        .author("Norman Kern <norman.kern@gmx.com>")
        .about(&blktrace_about[..])
        .arg(
            Arg::with_name("dev")
                .short("d")
                .long("dev")
                .takes_value(true)
                .multiple(true)
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
        .arg(
            Arg::with_name("time")
                .short("w")
                .long("stopwatch")
                .takes_value(true)
                .help("Stop after defined time, in seconds"),
        )
        .arg(
            Arg::with_name("action field")
                .short("a")
                .long("act-mask")
                .takes_value(true)
                .help("Only trace specified actions. See documentation"),
        )
        .arg(
            Arg::with_name("action mask")
                .short("A")
                .long("set-mask")
                .takes_value(true)
                .help("Give trace mask as a single value. See documentation"),
        )
        .arg(
            Arg::with_name("size")
                .short("b")
                .long("buffer-size")
                .takes_value(true)
                .help("Sub buffer size in KiB (default 512)"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("num-sub-buffers")
                .takes_value(true)
                .help("Number of sub buffers (default 4)"),
        )
        .arg(
            Arg::with_name("listen")
                .short("l")
                .long("listen")
                .takes_value(false)
                .help("Run in network listen mode (blktrace server)"),
        )
        .arg(
            //NOTE: The C version of blktrace uses -h not -H.
            Arg::with_name("hostname")
                .short("H")
                .long("host")
                .takes_value(true)
                .help("Run in network client mode, connecting to the given host"),
        )
        .arg(
            Arg::with_name("port number")
                .short("p")
                .long("port")
                .takes_value(true)
                .help("Network port to use (default 8462)"),
        )
        .arg(
            Arg::with_name("no-sendfile")
                .short("s")
                .long("no-sendfile")
                .takes_value(false)
                .help("Make the network client NOT use sendfile() to transfer data"),
        )
        .arg(
            Arg::with_name("devs file")
                .short("I")
                .long("input-devs")
                .takes_value(true)
                .help("Add devices found in <devs file>"),
        )
        .get_matches();

    let devices: Vec<&str> = ctx.matches.values_of("dev").unwrap().collect();

    println!("Values: {:?}", devices);
}

pub struct BlktraceCtx<'a> {
    pub devices: Vec<&'a str>,
    pub devpaths: Vec<DevPath>,
    matches: ArgMatches<'a>,
}

pub struct DevPath {
    pub path: String,
    pub file: File,
}

fn main() {
    let mut blktrace_ctx = BlktraceCtx {
        devices: Vec::new(),
        devpaths: Vec::new(),
        matches: ArgMatches::new(),
    };

    handle_args(&mut blktrace_ctx);
}
