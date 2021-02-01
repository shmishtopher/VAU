//! @author     Shmish  "c.schmitt@my.ccsu.edu"
//! @version    0.1.0   "1/30/2021"
//! @licence    MIT     "(c) 2021 Christopher K. Schmitt"


use std::path::Path;
use clap::clap_app;


fn main() {    
    let matches = clap_app!(app =>
        (version: "0.2.0-alpha")
        (author: "Shmish <c.schmitt@my.ccsu.edu")
        (about: "A tool to unpack and repack vocaloid voicebanks")
        (usage: "vau [OPTIONS] <archive>")
        (@arg ARCHIVE: <archive> {validate_archive} "The archive file (.ddb) to unpack")
        (@arg BIT_DEPTH: -b --bits [bit_depth] "Configures the bit depth for of the samples, defaults to 32")
        (@arg SAMPLE_RATE: -r --rate [sample_rate] "Configures the sample rate of the samples, defaults to 22050")
        (@group OUTPUT_FORMAT +required =>
            (@arg OUT_FILE: -f --outFile [out_file] "Concatenate and emit samples to a single file")
            (@arg OUT_DIR: -d --outDir [out_dir] "Place each sample in its own file in this directory")
        )
    ).get_matches();
}


/// validate_archive validates an ddb file
/// provided by the user.  We check to see if the
/// provided file extension is ".ddb" and the
/// path to the archive exists.
fn validate_archive(path: String) -> Result<(), String> {
    if !path.ends_with(".ddb") {
        Err(String::from("The file format must be .ddb"))
    }
    else if !Path::new(&path).exists() {
        Err(String::from("The file could not be found"))
    }
    else {
        Ok(())
    }
}
