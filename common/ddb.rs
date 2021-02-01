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

        while last < self.sound_bank.len() && !self.sound_bank[head .. last].starts_with(b"SND ") {
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