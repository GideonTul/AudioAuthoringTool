use audio_decode::SampleSource;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

/// Voice represents an individual audio source with its associated properties, such as volume.
pub struct Voice {
    source: Box<dyn SampleSource>,
    volume: f32,
    // More fields will be added later, such as panning, pitch, etc.
}
/// VoiceMixer is responsible for mixing multiple voices into a single audio output.
pub struct VoiceMixer {
    voices: Vec<Voice>,
    finished: Arc<AtomicBool>, // Will likely be removed eventually once the program can run indefinitely.
}
/// VoiceMixer implementation
impl VoiceMixer {
    /// Creates a new VoiceMixer instance.
    pub fn new() -> Self {
        Self { 
            voices: Vec::new(),
            finished: Arc::new(AtomicBool::new(false)), 
        }
    }
    /// Adds a new voice to the mixer with the specified audio source and volume.
    pub fn add_voice(&mut self, source: Box<dyn SampleSource>, volume: f32) {
        self.voices.push(Voice { source, volume });
    }
    /// Renders the mixed audio from all voices into the provided output buffer.
    pub fn render(&mut self, buffer: &mut [f32]) {
        buffer.fill(0.0);

        let mut temp_buffer = vec![0.0; buffer.len()];

        // Iterate through the voices and mix their audio into the output buffer.
        // The retain_mut method is used to keep only the voices that have not finished playing.
        self.voices.retain_mut(|voice| {
            temp_buffer.fill(0.0);

            match voice.source.read(&mut temp_buffer) {
                Ok(0) => false,
                Ok(n) => {
                    for i in 0..n {
                        buffer[i] += temp_buffer[i] * voice.volume;
                    }
                    true
                }
                Err(e) => {
                    eprintln!("Error reading from voice source: {}", e);
                    false
                }
            }
        });

        if self.voices.is_empty() {
            self.finished.store(true, Ordering::Relaxed);
        }
    }
    pub fn finished(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.finished)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use audio_decode::SampleSource;
    use symphonia::core::errors::Error;

    struct MockSource {
        samples: Vec<f32>,
        position: usize,
    }

    impl MockSource {
        fn new(samples: Vec<f32>) -> Self {
            Self {
                samples,
                position: 0,
            }
        }
    }

    impl SampleSource for MockSource {
        fn sample_rate(&self) -> u32 {
            44100
        }

        fn channels(&self) -> u16 {
            1
        }

        fn read(&mut self, out: &mut [f32]) -> Result<usize, Error> {
            let remaining = self.samples.len() - self.position;
            let count = remaining.min(out.len());

            out[..count]
                .copy_from_slice(&self.samples[self.position..self.position + count]);

            self.position += count;

            Ok(count)
        }
    }

    #[test]
    fn new_mixer_is_empty_and_not_finished() {
        let mixer = VoiceMixer::new();

        assert!(!mixer.finished().load(Ordering::Relaxed));
    }

    #[test]
    fn empty_mixer_outputs_silence() {
        let mut mixer = VoiceMixer::new();
        let mut buffer = [1.0, 2.0, 3.0];

        mixer.render(&mut buffer);

        assert_eq!(buffer, [0.0, 0.0, 0.0]);
        assert!(mixer.finished().load(Ordering::Relaxed));
    }

    #[test]
    fn single_voice_is_rendered() {
        let mut mixer = VoiceMixer::new();

        mixer.add_voice(
            Box::new(MockSource::new(vec![1.0, 2.0, 3.0])),
            1.0,
        );

        let mut buffer = [0.0; 3];

        mixer.render(&mut buffer);

        assert_eq!(buffer, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn voice_volume_is_applied() {
        let mut mixer = VoiceMixer::new();

        mixer.add_voice(
            Box::new(MockSource::new(vec![1.0, 2.0, 3.0])),
            0.5,
        );

        let mut buffer = [0.0; 3];

        mixer.render(&mut buffer);

        assert_eq!(buffer, [0.5, 1.0, 1.5]);
    }

    #[test]
    fn multiple_voices_are_mixed() {
        let mut mixer = VoiceMixer::new();

        mixer.add_voice(
            Box::new(MockSource::new(vec![1.0, 2.0, 3.0])),
            1.0,
        );

        mixer.add_voice(
            Box::new(MockSource::new(vec![4.0, 5.0, 6.0])),
            1.0,
        );

        let mut buffer = [0.0; 3];

        mixer.render(&mut buffer);

        assert_eq!(buffer, [5.0, 7.0, 9.0]);
    }

    #[test]
    fn render_clears_existing_buffer() {
        let mut mixer = VoiceMixer::new();

        mixer.add_voice(
            Box::new(MockSource::new(vec![1.0, 2.0])),
            1.0,
        );

        let mut buffer = [100.0, 100.0];

        mixer.render(&mut buffer);

        assert_eq!(buffer, [1.0, 2.0]);
    }

    #[test]
    fn shorter_voice_only_fills_available_samples() {
        let mut mixer = VoiceMixer::new();

        mixer.add_voice(
            Box::new(MockSource::new(vec![1.0, 2.0])),
            1.0,
        );

        let mut buffer = [0.0; 4];

        mixer.render(&mut buffer);

        assert_eq!(buffer, [1.0, 2.0, 0.0, 0.0]);
    }

    #[test]
    fn voice_progresses_between_renders() {
        let mut mixer = VoiceMixer::new();

        mixer.add_voice(
            Box::new(MockSource::new(vec![1.0, 2.0, 3.0, 4.0])),
            1.0,
        );

        let mut buffer = [0.0; 2];

        mixer.render(&mut buffer);
        assert_eq!(buffer, [1.0, 2.0]);

        mixer.render(&mut buffer);
        assert_eq!(buffer, [3.0, 4.0]);
    }

    #[test]
    fn finished_voice_is_removed() {
        let mut mixer = VoiceMixer::new();

        mixer.add_voice(
            Box::new(MockSource::new(vec![1.0, 2.0])),
            1.0,
        );

        let mut buffer = [0.0; 2];

        // Consume the voice.
        mixer.render(&mut buffer);
        assert_eq!(buffer, [1.0, 2.0]);

        // Source now returns Ok(0), so retain_mut removes it.
        mixer.render(&mut buffer);

        assert_eq!(buffer, [0.0, 0.0]);
        assert!(mixer.finished().load(Ordering::Relaxed));
    }

    #[test]
    fn finished_voice_does_not_remove_other_voices() {
        let mut mixer = VoiceMixer::new();

        mixer.add_voice(
            Box::new(MockSource::new(vec![1.0])),
            1.0,
        );

        mixer.add_voice(
            Box::new(MockSource::new(vec![10.0, 20.0])),
            1.0,
        );

        let mut buffer = [0.0];

        mixer.render(&mut buffer);
        assert_eq!(buffer, [11.0]);

        mixer.render(&mut buffer);
        assert_eq!(buffer, [20.0]);

        assert!(!mixer.finished().load(Ordering::Relaxed));

        mixer.render(&mut buffer);

        assert!(mixer.finished().load(Ordering::Relaxed));
    }
}