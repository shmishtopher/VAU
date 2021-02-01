//! @author     Shmish  "c.schmitt@my.ccsu.edu"
//! @version    0.1.0   "1/30/2021"
//! @licence    MIT     "(c) 2021 Christopher K. Schmitt"


use clap::clap_app;


fn main() {    
    let matches = clap_app!(app =>
        (version: "0.2.0-alpha")
        (author: "Shmish <c.schmitt@my.ccsu.edu")
        (about: "A tool to unpack and repack vocaloid voicebanks")
    ).get_matches();
}
