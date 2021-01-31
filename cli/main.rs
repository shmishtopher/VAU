//! @author     Shmish  "c.schmitt@my.ccsu.edu"
//! @version    0.1.0   "1/30/2021"
//! @licence    MIT     "(c) 2021 Christopher K. Schmitt"


use clap::App;
use clap::Arg;


fn main() {
    let matches = App::new("The Vocaloid Archive Utility")
        .version("0.1.0")
        .author("Shmish <c.schmitt@my.ccsu.edu>")
        .about("A tool for unpacking and repacking vocaloid sound-banks")
        .arg(Arg::with_name("archive")
            .help("Archive file (.ddb) to unpack")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("outDir")
            .help("Emit output output samples to directory")
            .long("outDir")
            .short("d")
            .takes_value(true))
        .arg(Arg::with_name("outFile")
            .help("Concatenate and emit output to single file")
            .long("outFile")
            .short("f")
            .takes_value(true))
        .get_matches();
}
