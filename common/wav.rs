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


use std::convert::TryFrom;


/// AudioFormat enumerates some of the possible
/// waveform data format codes.  Many of these
/// are now obselte.  Additionally, format codes
/// for compressed data are not implemented.
pub enum AudioFormat {
    PCM,
    IeeeFloat,
    ALaw,
    ULaw,
    Extensible
}


impl From<AudioFormat> for u16 {
    fn from(format: AudioFormat) -> u16 {
        match format {
            AudioFormat::PCM        => 0x0001,
            AudioFormat::IeeeFloat  => 0x0003,
            AudioFormat::ALaw       => 0x0006,
            AudioFormat::ULaw       => 0x0007,
            AudioFormat::Extensible => 0xFFFE
        }
    }
}


impl TryFrom<usize> for AudioFormat {
    type Error = &'static str;

    fn try_from(format: usize) -> Result<Self, Self::Error> {
        match format {
            0x0001 => Ok(AudioFormat::PCM),
            0x0003 => Ok(AudioFormat::IeeeFloat),
            0x0006 => Ok(AudioFormat::ALaw),
            0x0007 => Ok(AudioFormat::ULaw),
            0xFFFE => Ok(AudioFormat::Extensible),
            _      => Err("Invalid audio format specifier")
        }
    }
}


/// Wav represents a complete .wav file
/// containing chunk headers and audio samples.
pub struct Wav {
    format: u16,
    channels: u16,
    bit_depth: u16,
    sample_rate: u32,
    samples: Vec<u8>,
}


impl Wav {
    pub fn new(format: AudioFormat, channels: u16, bit_depth: u16, sample_rate: u32) -> Self {
        Wav {
            format: format.into(),
            samples: Vec::new(),
            channels,
            bit_depth,
            sample_rate,
        }
    }

    pub fn write(&mut self, bytes: &[u8]) {
        self.samples.extend(bytes);
    }
}


impl Default for Wav {
    fn default() -> Self {
        Wav::new(AudioFormat::PCM, 1, 16, 44100)
    }
}


impl From<Wav> for Vec<u8> {
    fn from(wav: Wav) -> Self {
        let mut header = [0; 44];
        let mut bytes = Vec::new();
        
        // Write chunk descriptors
        header[0 .. 4].copy_from_slice(b"RIFF");
        header[8 .. 12].copy_from_slice(b"WAVE");
        header[12 .. 16].copy_from_slice(b"fmt ");
        header[36 .. 40].copy_from_slice(b"data");

        // Compute chunk sizes
        let data_size = wav.samples.len() as u32;
        let chunk_size = data_size + 32;

        // Write chunk sizes
        header[4 .. 8].copy_from_slice(&chunk_size.to_le_bytes());
        header[16 .. 20].copy_from_slice(&16u32.to_le_bytes());
        header[40 .. 44].copy_from_slice(&data_size.to_le_bytes());

        // Write fmt options
        header[20 .. 22].copy_from_slice(&wav.format.to_le_bytes());
        header[22 .. 24].copy_from_slice(&wav.channels.to_le_bytes());
        header[24 .. 28].copy_from_slice(&wav.sample_rate.to_le_bytes());
        header[34 .. 36].copy_from_slice(&wav.bit_depth.to_le_bytes());

        // Compute block alignment and byte rate
        let block_align = wav.channels * wav.bit_depth / 8;
        let byte_rate = wav.sample_rate * wav.channels as u32 * wav.bit_depth as u32 / 8;

        // Write block alignment and byte rate
        header[32 .. 34].copy_from_slice(&block_align.to_le_bytes());
        header[28 .. 32].copy_from_slice(&byte_rate.to_le_bytes());

        bytes.extend(header.iter());
        bytes.extend(wav.samples);

        bytes
    }
}