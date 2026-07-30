/// A source of decoded PCM audio samples.
///
/// Samples are always normalized `f32` values and are interleaved by channel.
///
/// Currently, the source will either be from Memory (fully decoded file), 
/// or stream (read and decoded as the file is playing).

use symphonia::core::errors::Error;

pub enum LoadMode {
    Static, // might rename later
    Streaming,
    Auto,
}

// Interface
pub trait SampleSource: Send + 'static {
    fn sample_rate(&self) -> u32;
    fn channels(&self) -> u16;
    fn read(&mut self, out_buf: &mut [f32]) -> Result<usize, Error>;
}

