use crate::source::SampleSource;
use symphonia::core::audio::sample::Sample;
use symphonia::core::codecs::audio::AudioDecoder;
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatReader;

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

impl StreamingSource {
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
    fn decode_next_packet(&mut self) -> Result<Option<Vec<f32>>, Error> {
        loop {
            let packet = match self.format.next_packet()? {
                Some(p) => p,
                None => return Ok(None),
            };
            if packet.track_id != self.track_id {
                continue;
            }
            match self.decoder.decode(&packet) {
                Ok(audio_buf) => {
                    let mut buf = vec![f32::MID; audio_buf.samples_interleaved()];
                    audio_buf.copy_to_slice_interleaved(&mut buf);
                    return Ok(Some(buf));
                }
                Err(Error::DecodeError(_)) => continue,
                Err(e) => return Err(e),
            }
        }
    }

}
impl SampleSource for StreamingSource {
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    fn channels(&self) -> u16 {
        self.channels
    }

    fn read(&mut self, out: &mut [f32]) -> Result<usize, Error> {
        let mut written = 0;
        while written < out.len() {
            if self.buf_pos >= self.buf.len() {
                if self.finished {
                    break;
                }
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

            let available = self.buf.len() - self.buf_pos;
            let remaining_out = out.len() - written;
            let n = available.min(remaining_out);
            out[written..written + n].copy_from_slice(&self.buf[self.buf_pos..self.buf_pos + n]);
            self.buf_pos += n;
            written += n;
        }
        Ok(written)
    }
}

// Difficult to test independently.
