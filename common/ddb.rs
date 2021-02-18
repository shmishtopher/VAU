//! @author     Shmish  "c.schmitt@my.ccsu.edu"
//! @version    0.1.0   "1/30/2021"
//! @licence    MIT     "(c) 2021 Christopher K. Schmitt"


/// Samples is an iterator which interates over
/// all of the PCM encoded data slices in the ddb
/// archive.
pub struct Samples<'a> {
    start: usize,
    sound_bank: &'a [u8]
}


impl<'a> Samples<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        Samples {
            start: 0,
            sound_bank: bytes
        }
    }
}


impl<'a> Iterator for Samples<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let mut head = self.start;
        let mut last = self.start + 4;

        while last < self.sound_bank.len() && !self.sound_bank[head .. last].starts_with(b"SND") {
            head += 1;
            last += 1;
        }

        if self.sound_bank[head .. last].starts_with(b"SND ") {
            let mut size = [0u8; 4];
            size.copy_from_slice(&self.sound_bank[head + 4 .. head + 8]);
            let size = u32::from_le_bytes(size);

            self.start = head + 1;

            Some(&self.sound_bank[head + 16 .. head + 16 + size as usize])
        }
        else {
            None
        }
    }
}


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
