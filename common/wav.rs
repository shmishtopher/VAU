//! @author     Shmish  "c.schmitt@my.ccsu.edu"
//! @version    0.1.0   "1/30/2021"
//! @licence    MIT     "(c) 2021 Christopher K. Schmitt"


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