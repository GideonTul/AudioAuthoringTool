use std::fs::File;
use std::path::Path;

use symphonia::core::codecs::audio::{AudioCodecParameters, AudioDecoder, AudioDecoderOptions};
use symphonia::core::errors::Error;
use symphonia::core::formats::probe::Hint;
use symphonia::core::formats::{FormatReader, Track, TrackType};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;

use crate::source::{LoadMode, SampleSource};
use crate::memory_source::MemorySource;
use crate::streaming_source::StreamingSource;

const AUTO_STREAM_THRESHOLD_BYTES: u64 = 1024 * 1024;

pub struct SymphoniaLoader;

impl SymphoniaLoader {
    // Fully decodes 'path' into memory at its native sample rate/channels.
    pub fn decode<P: AsRef<Path>>(path: P) -> Result<MemorySource, Error> {
        let mut stream = Self::stream(path)?;
        let sample_rate = stream.sample_rate();
        let channels = stream.channels();

        let mut samples = Vec::new();
        let mut chunk = [0.0; 4096];
        loop {
            let n = stream.read(&mut chunk)?;
            if n == 0 {
                break;
            }
            samples.extend_from_slice(&chunk[..n]);
        }

        Ok(MemorySource::new(samples, sample_rate, channels))
    }

    // Opens 'path' for on-demand decoding
    pub fn stream<P: AsRef<Path>>(path: P) -> Result<StreamingSource, Error> {
        let mut format = Self::open_format(&path)?;
        let track = Self::select_track_type(format.as_mut())?;
        let track_id = track.id;
        let audio_params = Self::audio_params(track)?;

        let sample_rate = Self::get_sample_rate(audio_params);
        let channels = Self::get_channels(audio_params);
        let decoder = Self::get_decoder(audio_params)?;

        Ok(StreamingSource::new(format, decoder, track_id, sample_rate, channels))
    }

    pub fn create<P: AsRef<Path>>(path: P, mode: LoadMode) -> Result<Box<dyn SampleSource>, Error> {
        let mode = match mode {
            LoadMode::Auto => Self::resolve_auto(path.as_ref()),
            explicit => Ok(explicit),
        };
        match mode {
            Ok(LoadMode::Static) => Ok(Box::new(Self::decode(path)?)),
            Ok(LoadMode::Streaming) => Ok(Box::new(Self::stream(path)?)),
            Ok(LoadMode::Auto) => unreachable!("resolve_auto never returns Auto"),
            Err(_) => todo!(),
        }
    }

    pub fn auto<P: AsRef<Path>>(path: P) -> Result<Box<dyn SampleSource>, Error> {
        Self::create(path, LoadMode::Auto)
    }

    fn resolve_auto(path: &Path) -> Result<LoadMode, Error> {
        // let size = std::fs::metadata(path)
        // .map(|m| m.len())
        // .unwrap_or(u64::MAX);
        let size = std::fs::metadata(path)?.len();

        println!("Size {}", size);
        if size < AUTO_STREAM_THRESHOLD_BYTES {
            Ok(LoadMode::Static)
        } else {
            Ok(LoadMode::Streaming)
        }
    }

    // Helpers
    fn open_format<P: AsRef<Path>>(path: P) -> Result<Box<dyn FormatReader>, Error> {
        let file = File::open(&path)?;
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let mut hint = Hint::new();
        if let Some(ext) = path.as_ref().extension().and_then(|e| e.to_str()) {
            hint.with_extension(ext);
        }

        let format_opts = Default::default();
        let meta_opts: MetadataOptions = Default::default();

        symphonia::default::get_probe().probe(&hint, mss, format_opts, meta_opts)
    }

    fn select_track_type(format: &mut dyn FormatReader) -> Result<&Track, Error> {
        format
            .default_track(TrackType::Audio)
            .ok_or(Error::DecodeError("No audio track"))
    }

    fn audio_params(track: &Track) -> Result<&AudioCodecParameters, Error> {
        track
            .codec_params
            .as_ref()
            .ok_or(Error::DecodeError("Missing codec params"))?
            .audio()
            .ok_or(Error::DecodeError("Track is not audio"))
    }

    fn get_sample_rate(audio_params: &AudioCodecParameters) -> u32 {
        audio_params.sample_rate.unwrap_or(44100)
    }

    fn get_channels(audio_params: &AudioCodecParameters) -> u16 {
        audio_params.channels.as_ref().map(|c| c.count()).unwrap_or(1) as u16
    }

    fn get_decoder(audio_params: &AudioCodecParameters) -> Result<Box<dyn AudioDecoder>, Error> {
        let dec_opts: AudioDecoderOptions = Default::default();
        symphonia::default::get_codecs().make_audio_decoder(audio_params, &dec_opts)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;



    #[test]
    fn resolve_auto_picks_static_below_threshold() {
        let file =  Path::new("tests/assets/Bongos.mp3");
        assert!(matches!(SymphoniaLoader::resolve_auto(file), Ok(LoadMode::Static)));
    }

    #[test]
    fn resolve_auto_picks_streaming_at_or_above_threshold() {
        let file =  Path::new("tests/assets/unl1.wav");
        assert!(matches!(SymphoniaLoader::resolve_auto(file), Ok(LoadMode::Streaming)));
    }

}