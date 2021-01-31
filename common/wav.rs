//! @author     Shmish  "c.schmitt@my.ccsu.edu"
//! @version    0.1.0   "1/30/2021"
//! @licence    MIT     "(c) 2021 Christopher K. Schmitt"


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