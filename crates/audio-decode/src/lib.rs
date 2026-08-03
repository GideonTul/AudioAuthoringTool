//! Audio decoding library.
//! 
//! Provides a common interface for loading audio files from different
//! backends and exposing them as PCM sample streams.
//! 
//! # Example
//!
//! ```no_run
//! use audio_decode::{SampleSource, SymphoniaLoader};
//!
//! let mut source = SymphoniaLoader::auto("music.mp3")?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

mod loader;
mod source;
mod memory_source;
mod streaming_source;

pub use loader::SymphoniaLoader;
pub use source::{LoadMode, SampleSource};
pub use memory_source::MemorySource;
pub use streaming_source::StreamingSource;