//! @author     Shmish  "c.schmitt@my.ccsu.edu"
//! @version    0.1.0   "1/30/2021"
//! @licence    MIT     "(c) 2021 Christopher K. Schmitt"


/// Samples is an iterator which interates over
/// all of the PCM encoded data slices in the ddb
/// archive.
pub struct Samples<'a> {
    position: usize,
    sound_bank: &'a [u8]
}


impl<'a> Samples<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        Samples {
            position: 0,
            sound_bank: bytes
        }
    }
}


impl<'a> Iterator for Samples<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        // Find the start of the next sample
        while !self.sound_bank[self.position .. self.position + 3].starts_with(b"SND") {
            self.position += 1;
            if self.position + 3 >= self.sound_bank.len() {
                return None
            }
        }

        // Parse the length of the sample
        let mut sample_size = [0u8; 4];
        sample_size.copy_from_slice(&self.sound_bank[self.position + 4 .. self.position + 8]);
        let sample_size = u32::from_le_bytes(sample_size);

        // Return the sample and increment the counter
        let position = self.position;
        self.position += 1;
        Some(&self.sound_bank[position + 8 .. position + sample_size as usize])
    }
}