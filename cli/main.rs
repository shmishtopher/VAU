//! Copyright 2021 Christopher K. Schmitt "Shmish"
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//! 
//!     http://www.apache.org/licenses/LICENSE-2.0
//! 
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.


use std::path::Path;
use std::fs;
use clap::clap_app;
use clap::value_t;
use common::ddb::Archive;
use common::wav::Wav;
use common::wav::AudioFormat;


fn main() { 
    let matches = clap_app!(app =>
        (version: "0.3.0-alpha")
        (author: "Shmish <c.schmitt@my.ccsu.edu")
        (about: "A tool to unpack and repack vocaloid voicebanks")
        (@setting GlobalVersion)
        (@setting SubcommandRequiredElseHelp)

        (@subcommand unpack =>
            (about: "Unpacks a voicebank into pcm, frm, and env chunks.  Also supports wav extraction")
            (@arg ARCHIVE: <archive> {validate_archive} "The archive file (.ddb) to unpack")
            (@arg OUTPUT: [output] "The otput directory to place all the wav, frm, and env samples in")
            (@arg BIT_DEPTH: -b --bits [bit_depth] "Configures the bit depth for of the samples, defaults to 32")
            (@arg SAMPLE_RATE: -r --rate [sample_rate] "Configures the sample rate of the samples, defaults to 22050")
            (@arg WAVE_FILE: -w --wav [wave_file] "Concatenate and emit samples to a single wav file") 
        )
    ).get_matches();


    if let ("unpack", Some(args)) = matches.subcommand() {
        // Default bit depth and sample rate
        let bit_depth = value_t!(args, "BIT_DEPTH", u16).unwrap_or(32);
        let sample_rate = value_t!(args, "SAMPLE_RATE", u32).unwrap_or(22050);

        // Default unpack loaction
        let output = value_t!(args, "OUTPUT", String).unwrap_or_else(|_| ".".to_owned());

        // Open up archive file
        println!("Loading archive");
        let archive = Archive(fs::read(args.value_of("ARCHIVE").unwrap()).unwrap());

        // Setup output directory
        fs::create_dir_all(format!("{}/wav", output)).unwrap();
        fs::create_dir_all(format!("{}/frm", output)).unwrap();
        fs::create_dir_all(format!("{}/env", output)).unwrap();

        // Extract all samples
        println!("Extracting pcm data");
        for (i, sample) in archive.snd_files().enumerate() {
            let mut wav = Wav::new(AudioFormat::PCM, 1, bit_depth, sample_rate);
            wav.write(&sample.1);
            let wav: Vec<u8> = wav.into();
            
            if fs::write(format!("{}/wav/{}.wav", output, i), wav).is_err() {
                println!("Failed to create wav file");
            }
        }

        // Emit single wav file (if present)
        if let Some(path) = args.value_of("WAVE_FILE") {
            println!("Creating .wav file");
            let mut wav = Wav::new(AudioFormat::PCM, 1, bit_depth, sample_rate);

            for sample in archive.snd_files() {
                wav.write(&sample.1);
            }
            
            Path::new(path).parent().map(fs::create_dir_all);
            let wav: Vec<u8> = wav.into();
            
            if fs::write(path, wav).is_err() {
                println!("Failed to create wav file");
            }
        }

        // Done!
        println!("Done!");
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
