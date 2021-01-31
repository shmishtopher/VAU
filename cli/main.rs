//! @author     Shmish  "c.schmitt@my.ccsu.edu"
//! @version    0.1.0   "1/30/2021"
//! @licence    MIT     "(c) 2021 Christopher K. Schmitt"


use clap::App;
use clap::Arg;
use std::fs;
use common::ddb::Samples;
use common::wav::Wav;
use common::wav::AudioFormat;


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


    if let Ok(archive) = fs::read(matches.value_of("archive").unwrap()) {
        if let Some(out_file) = matches.value_of("outFile") {
            let mut wav = Wav::new(AudioFormat::PCM, 1, 32, 22050);

            for sample in Samples::from_bytes(&archive) {
                wav.write(sample);
            }

            let wav: Vec<u8> = wav.into();
            
            match fs::write(out_file, wav) {
                Ok(_) => println!("Unpacked archive to '{}'", out_file),
                Err(_) => println!("Failed to unpack archive")
            }
        }

        if let Some(out_dir) = matches.value_of("outDir") {
            if fs::create_dir_all(out_dir).is_ok() {
                for (i, sample) in Samples::from_bytes(&archive).enumerate() {
                    let mut wav = Wav::new(AudioFormat::PCM, 1, 32, 22050);
                    
                    wav.write(sample);
                    
                    let wav: Vec<u8> = wav.into();

                    match fs::write(format!("{}/{}.wav", out_dir, i), wav) {
                        Ok(_) => println!("Unpacked archive to '{}'", out_dir),
                        Err(_) => println!("Failed to unpack sample")
                    }
                }
            }
            else {
                println!("Failed to create some directories");
            }
        }
    }
    else {
        println!("Error: Cannot find \"{}\"", matches.value_of("archive").unwrap());
    }
}
