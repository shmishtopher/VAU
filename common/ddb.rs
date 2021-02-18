//! @author     Shmish  "c.schmitt@my.ccsu.edu"
//! @version    0.1.0   "1/30/2021"
//! @licence    MIT     "(c) 2021 Christopher K. Schmitt"


pub struct SNDHeader {
    pub magic_bytes: u32,
    pub file_size: u32,
    pub frame_rate: u32,
    pub channels: u16,
    pub index: u32,
}


impl From<&[u8]> for SNDHeader {
    fn from(data: &[u8]) -> Self {
        let mut magic_bytes = [0; 4];
        let mut file_size = [0; 4];
        let mut frame_rate = [0; 4];
        let mut channels = [0; 2];
        let mut index = [0; 4];

        magic_bytes.copy_from_slice(&data[0..4]);
        file_size.copy_from_slice(&data[4..8]);
        frame_rate.copy_from_slice(&data[8..12]);
        channels.copy_from_slice(&data[12..14]);
        index.copy_from_slice(&data[14..18]);

        SNDHeader {
            magic_bytes: u32::from_be_bytes(magic_bytes),
            file_size: u32::from_le_bytes(file_size),
            frame_rate: u32::from_le_bytes(frame_rate),
            channels: u16::from_le_bytes(channels),
            index: u32::from_le_bytes(index),
        }
    }
}


pub struct Archive(pub Vec<u8>);
pub struct SNDFile(pub SNDHeader, pub Vec<u8>);


impl Archive {
    pub fn snd_files(&self) -> impl Iterator<Item = SNDFile> + '_ {
        self.0
            .windows(18)
            .enumerate()
            .filter(|(_, buf)| buf.starts_with(b"SND "))
            .map(move |(i, buf)| {
                let header: SNDHeader = buf.into();
                let start = i + 18;
                let end = i + header.file_size as usize;

                SNDFile(header, self.0[start..end].to_vec())
            })
    }
}
