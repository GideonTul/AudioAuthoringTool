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

