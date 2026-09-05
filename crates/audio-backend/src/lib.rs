//! Audio backend library
//! This library provides the audio backend functionality for the audio authoring tool.
//! Basically just a wrapper around cpal.
//! 
//! # Example
//! 
//! ``` no_run
//! # use audio_backend::AudioBackend;
//! let mut backend = AudioBackend::new().expect("Failed to initialize backend");
//! backend.start(|data, _format| {
//!     for sample in data.iter_mut() { // eventually will be conumer.read()
//!         *sample = 0.0;
//!     }
//! }).expect("Failed to initialize backend in test");
//! backend.play().expect("Failed to play stream");
//! backend.pause().expect("Failed to pause stream");
//! ```

mod audio_backend;

pub use audio_backend::{AudioBackend, AudioFormat};