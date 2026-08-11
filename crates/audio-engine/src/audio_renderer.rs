use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use audio_decode::SampleSource;

/// AudioRenderer is responsible for rendering audio from multiple SampleSource instances into a single output buffer.
pub struct AudioRenderer {
    temp: Vec<f32>,
    finished: Arc<AtomicBool>,
}

/// AudioRenderer implementation
impl AudioRenderer {
    /// Creates a new AudioRenderer instance.
    pub fn new() -> Self {
        Self {
            temp: Vec::new(),
            finished: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Renders audio from multiple SampleSource instances into the provided output buffer.
    pub fn render(
        &mut self,
        buffer: &mut [f32],
        sources: &mut Vec<Box<dyn SampleSource>>,
    ) {
        // Ensure the temporary buffer is the same length as the output buffer.
        if self.temp.len() != buffer.len() {
            self.temp.resize(buffer.len(), 0.0);
        }
        // Clear the output buffer before mixing.
        buffer.fill(0.0);

        // Iterate through the sources and mix their audio into the output buffer.
        let mut i = 0;
        while i < sources.len() {
            self.temp.fill(0.0);

            // Read samples from the current source into the temporary buffer.
            match sources[i].read(&mut self.temp) {
                // If the source has finished, remove it from the list.
                Ok(0) => {
                    sources.remove(i);
                    continue;
                }
                // If samples were read, mix them into the output buffer.
                Ok(n) => {
                    for j in 0..n {
                        buffer[j] += self.temp[j];
                    }
                }
                // If there was an error reading from the source, log it and remove the source.
                Err(e) => {
                    eprintln!("Decode error: {e}");
                    sources.remove(i);
                    continue;
                }
            }

            i += 1;
        }
        // If all sources have finished, set the finished flag to true.
        if sources.is_empty() {
            self.finished.store(true, Ordering::Relaxed);
        }
    }

    pub fn finished(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.finished)
    }
}

#[cfg(test)]
mod tests {
    use symphonia::core::errors::Error;
    use super::*;
    use audio_decode::SampleSource;
    use std::io;

    struct MockSource {
        samples: Vec<f32>,
        pos: usize,
        fail: bool,
    }

    impl MockSource {
        fn new(samples: Vec<f32>) -> Self {
            Self {
                samples,
                pos: 0,
                fail: false,
            }
        }

        fn failing() -> Self {
            Self {
                samples: Vec::new(),
                pos: 0,
                fail: true,
            }
        }
    }

    impl SampleSource for MockSource {
        fn sample_rate(&self) -> u32 {
            44100
        }

        fn channels(&self) -> u16 {
            2
        }

        fn read(
            &mut self,
            out: &mut [f32],
        ) -> Result<usize, Error> {
            if self.fail {
                return Err(io::Error::other("decode failed").into());
            }

            let remaining = self.samples.len() - self.pos;
            let n = remaining.min(out.len());

            out[..n].copy_from_slice(&self.samples[self.pos..self.pos + n]);
            self.pos += n;

            Ok(n)
        }
    }

    #[test]
    fn render_single_source() {
        let mut renderer = AudioRenderer::new();

        let mut buffer = [0.0; 4];

        let mut sources: Vec<Box<dyn SampleSource>> = vec![
            Box::new(MockSource::new(vec![1.0, 2.0, 3.0, 4.0])),
        ];

        renderer.render(&mut buffer, &mut sources);

        assert_eq!(buffer, [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(sources.len(), 1);
        assert!(!renderer.finished().load(Ordering::Relaxed));
    }

    #[test]
    fn render_mixes_multiple_sources() {
        let mut renderer = AudioRenderer::new();

        let mut buffer = [0.0; 4];

        let mut sources: Vec<Box<dyn SampleSource>> = vec![
            Box::new(MockSource::new(vec![1.0, 2.0, 3.0, 4.0])),
            Box::new(MockSource::new(vec![0.5, 1.0, -1.0, 2.0])),
        ];

        renderer.render(&mut buffer, &mut sources);

        assert_eq!(buffer, [1.5, 3.0, 2.0, 6.0]);
    }

    #[test]
    fn removes_finished_sources() {
        let mut renderer = AudioRenderer::new();

        let mut buffer = [0.0; 4];

        let mut sources: Vec<Box<dyn SampleSource>> =
            vec![Box::new(MockSource::new(vec![]))];

        renderer.render(&mut buffer, &mut sources);

        assert!(sources.is_empty());
        assert!(renderer.finished().load(Ordering::Relaxed));
    }

    #[test]
    fn removes_failed_sources() {
        let mut renderer = AudioRenderer::new();

        let mut buffer = [0.0; 4];

        let mut sources: Vec<Box<dyn SampleSource>> =
            vec![Box::new(MockSource::failing())];

        renderer.render(&mut buffer, &mut sources);

        assert!(sources.is_empty());
        assert!(renderer.finished().load(Ordering::Relaxed));
    }

    #[test]
    fn renderer_can_render_different_buffer_sizes() {
        let mut renderer = AudioRenderer::new();

        let mut sources: Vec<Box<dyn SampleSource>> = vec![
            Box::new(MockSource::new((0..16).map(|x| x as f32).collect())),
        ];

        let mut small = [0.0; 4];
        renderer.render(&mut small, &mut sources);

        assert_eq!(small, [0.0, 1.0, 2.0, 3.0]);

        let mut large = [0.0; 8];
        renderer.render(&mut large, &mut sources);

        assert_eq!(large, [4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0]);
    }
}