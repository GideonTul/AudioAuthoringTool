use crate::source::SampleSource;
use symphonia::core::audio::sample::Sample;
use symphonia::core::codecs::audio::AudioDecoder;
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatReader;

/// Audio gets read and decoded in chunks, this allows for larger files to be played without a long delay.
pub struct StreamingSource {
    format: Box<dyn FormatReader>,
    decoder: Box<dyn AudioDecoder>,
    track_id: u32,
    sample_rate: u32,
    channels: u16,
    buf: Vec<f32>, // current packet's leftover samples
    buf_pos: usize,
    finished: bool,
}

/// Implementation of the StreamingSource, which provides methods to read and decode audio in chunks.
impl StreamingSource {
    // For tests
    pub(crate) fn new(
        format: Box<dyn FormatReader>,
        decoder: Box<dyn AudioDecoder>,
        track_id: u32,
        sample_rate: u32,
        channels: u16,
    ) -> Self {
        Self {
            format,
            decoder,
            track_id,
            sample_rate,
            channels,
            buf: Vec::new(),
            buf_pos: 0,
            finished: false,
        }
    }

    /// Decodes the next packet of audio data from the format reader and returns it as a vector of f32 samples.
    /// If there are no more packets to decode, returns None.
    fn decode_next_packet(&mut self) -> Result<Option<Vec<f32>>, Error> {

        // Loop until we find a packet that can be decoded or we reach the end of the stream.
        loop {
            // Call next_packet(), if it returns a packet store it
            let packet = match self.format.next_packet()? {
                Some(p) => p,
                None => return Ok(None),
            };
            // If the packet's track ID doesn't match the track we're decoding, skip it.
            if packet.track_id != self.track_id {
                continue;
            }
            // Attempt to decode the packet using the decoder. If successful, return the decoded samples.
            match self.decoder.decode(&packet) {

                // if packet decode is succesful, take the returned buffer and store it in audio_buf
                Ok(audio_buf) => {
                    // Create a buffer to hold the interleaved samples, initialized with the mid-point value for f32 (0.0).
                    // vec! is a macro that creates a new vector with the specified size and initial value.
                    let mut buf = vec![f32::MID; audio_buf.samples_interleaved()];

                    // Copy the samples from the audio buffer to the output buffer, converting them to f32.
                    // Interleaved means that the samples for each channel are stored sequentially,
                    // rather than all samples for one channel followed by all samples for the next channel.
                    audio_buf.copy_to_slice_interleaved(&mut buf);
                    return Ok(Some(buf));
                }
                // If the packet cannot be decoded, check the error type. If it's a decode error, continue to the next packet.
                Err(Error::DecodeError(_)) => continue,
                Err(e) => return Err(e),
            }
        }
    }

}

/// Implement the SampleSource trait for StreamingSource, allowing it to be used as a source of audio samples.
/// The read method reads samples from the audio source into the provided output buffer, returning the number of samples read.
impl SampleSource for StreamingSource {
    // Returns the sample rate of the audio source.
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    // Returns the number of channels in the audio source.
    fn channels(&self) -> u16 {
        self.channels
    }

    // Reads samples from the audio source into the provided output buffer, returning the number of samples read.
    fn read(&mut self, out_buf: &mut [f32]) -> Result<usize, Error> {
        let mut written = 0;
        // While we haven't filled the output buffer, keep reading and decoding packets.
        while written < out_buf.len() {
            if self.buf_pos >= self.buf.len() {
                // If we've consumed all samples in the current buffer, decode the next packet.
                if self.finished {
                    break;
                }
                // If there are no more packets to decode, mark the source as finished and break out of the loop.
                match self.decode_next_packet()? {
                    Some(buf) => {
                        self.buf = buf;
                        self.buf_pos = 0;
                    }
                    None => {
                        self.finished = true;
                        break;
                    }
                }
            }

            // Calculate how many samples we can copy from the current buffer to the output buffer.
            let available = self.buf.len() - self.buf_pos;
            let remaining_out_buf = out_buf.len() - written;
            let n = available.min(remaining_out_buf);

            // Copy the samples from the current buffer to the output buffer and update the positions.
            // Alternatively we could write something like: 
            // for i in 0..n {
            //     out_buf[written + i] = self.buf[self.buf_pos + i];
            // }
            out_buf[written..written + n].copy_from_slice(&self.buf[self.buf_pos..self.buf_pos + n]);
            self.buf_pos += n;
            written += n;
        }
        Ok(written)
    }
}

// Difficult to test independently. So all tests for this are in tests/loader.rs.
