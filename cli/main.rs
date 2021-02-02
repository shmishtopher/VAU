//! @author     Shmish  "c.schmitt@my.ccsu.edu"
//! @version    0.1.0   "1/30/2021"
//! @licence    MIT     "(c) 2021 Christopher K. Schmitt"


use std::path::Path;
use std::fs;
use clap::clap_app;
use clap::value_t;
use common::ddb::Samples;
use common::wav::Wav;
use common::wav::AudioFormat;


fn main() { 
    let matches = clap_app!(app =>
        (version: "0.2.0-alpha")
        (author: "Shmish <c.schmitt@my.ccsu.edu")
        (about: "A tool to unpack and repack vocaloid voicebanks")
        (@arg ARCHIVE: <archive> {validate_archive} "The archive file (.ddb) to unpack")
        (@arg BIT_DEPTH: -b --bits [bit_depth] "Configures the bit depth for of the samples, defaults to 32")
        (@arg SAMPLE_RATE: -r --rate [sample_rate] "Configures the sample rate of the samples, defaults to 22050")
        (@arg OUT_FILE: -f --outFile [out_file] "Concatenate and emit samples to a single file")
        (@arg OUT_DIR: -d --outDir [out_dir] "Place each sample in its own file in this directory")
    ).get_matches();

    // Default bit depth and sample rate
    let bit_depth = value_t!(matches, "BIT_DEPTH", u16).unwrap_or(32);
    let sample_rate = value_t!(matches, "SAMPLE_RATE", u32).unwrap_or(22050);

    // Open up archive file
    let archive = fs::read(matches.value_of("ARCHIVE").unwrap()).unwrap();

    // Write samples to out file (if present)
    if let Some(out_file) = matches.value_of("OUT_FILE") {
        let mut wav = Wav::new(AudioFormat::PCM, 1, bit_depth, sample_rate);
        let samples = Samples::from_bytes(&archive);

        for sample in samples {
            wav.write(sample);
        }

        let wav: Vec<u8> = wav.into();
        Path::new(out_file).parent().map(fs::create_dir_all);
        
        if fs::write(out_file, wav).is_ok() {
            println!("Successfully unpacked archive to {}", out_file);
        }
        else {
            println!("Failed to unpack archive");
        }
    }

    // Write samples to directory (if present)
    if let Some(out_dir) = matches.value_of("OUT_DIR") {
        if fs::create_dir_all(out_dir).is_ok() {
            for (i, sample) in Samples::from_bytes(&archive).enumerate() {
                let mut wav = Wav::new(AudioFormat::PCM, 1, bit_depth, sample_rate);
                wav.write(sample);
                let wav: Vec<u8> = wav.into();

                if fs::write(format!("{}/{}.wav", out_dir, i), wav).is_ok() {
                    println!("Successfully unpacked sample #{}", i);
                }
                else {
                    println!("Failed to unpack sample #{}", i);
                }
            }
        }
    }
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
