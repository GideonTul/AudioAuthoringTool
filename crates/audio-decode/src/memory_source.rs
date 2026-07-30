use crate::source::SampleSource;
use symphonia::core::errors::Error;
pub struct MemorySource {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u16,
    pos: usize,
}

impl MemorySource {
    pub(crate) fn new(samples: Vec<f32>, sample_rate: u32, channels: u16) -> Self {
        Self { samples, sample_rate, channels, pos: 0 }
    }
}

impl SampleSource for MemorySource {
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    fn channels(&self) -> u16 {
        self.channels
    }
    fn read(&mut self, out: &mut [f32]) -> Result<usize, Error> {
        let remaining = self.samples.len() - self.pos;
        let n = remaining.min(out.len());
        out[..n].copy_from_slice(&self.samples[self.pos..self.pos + n]);
        self.pos += n;
        Ok(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_in_one_shot_returns_all_samples() {
        let mut source = MemorySource::new(vec![1.0, 2.0, 3.0, 4.0], 44_100, 1);
        let mut out = [0.0f32; 4];
        assert_eq!(source.read(&mut out).unwrap(), 4);
        assert_eq!(out, [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn read_in_small_chunks_reassembles_correctly() {
        let mut source = MemorySource::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], 44_100, 1);
        let mut collected = Vec::new();
        let mut chunk = [0.0f32; 2];
        loop {
            let n = source.read(&mut chunk).unwrap();
            if n == 0 {
                break;
            }
            collected.extend_from_slice(&chunk[..n]);
        }
        assert_eq!(collected, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    }

    #[test]
    fn read_after_exhaustion_returns_ok_zero_repeatedly() {
        let mut source = MemorySource::new(vec![1.0, 2.0], 44_100, 1);
        let mut out = [0.0f32; 8];
        assert_eq!(source.read(&mut out).unwrap(), 2);
        assert_eq!(source.read(&mut out).unwrap(), 0, "should be exhausted");
        assert_eq!(source.read(&mut out).unwrap(), 0, "should stay exhausted, not panic or wrap");
    }
}