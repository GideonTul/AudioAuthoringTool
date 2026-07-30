use symphonia::core::errors::Error;

pub enum LoadMode {
    Static,
    Streaming,
    Auto,
}

pub trait SampleSource: Send + 'static {
    fn sample_rate(&self) -> u32;
    fn channels(&self) -> u16;
    fn read(&mut self, out: &mut [f32]) -> Result<usize, Error>;
}

