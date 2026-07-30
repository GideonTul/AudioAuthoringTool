use audio_decode::{LoadMode, SampleSource, SymphoniaLoader};

const TEST_WAV_SONG: &str = "tests/assets/unl1.wav";
const TEST_SHORT: &str = "tests/assets/Bongos.mp3";

#[test]
fn auto_creates_working_source() {
    let mut source = SymphoniaLoader::auto(TEST_SHORT).unwrap();

    let mut buffer = [0.0; 512];

    assert!(source.read(&mut buffer).unwrap() > 0);
}

#[test]
fn missing_file_returns_error_not_panic() {
    let result = SymphoniaLoader::decode("this_file_does_not_exist_12345.wav");
    assert!(result.is_err());
}

#[test]
fn stream_and_decode_produce_identical_samples() {
    println!("cwd: {:?}", std::env::current_dir().unwrap());
    let static_source = SymphoniaLoader::decode(TEST_SHORT).expect("decode should succeed");

    let mut stream = SymphoniaLoader::stream(TEST_SHORT).expect("stream should succeed");
    let mut streamed = Vec::new();
    let mut chunk = [0.0f32; 512];
    loop {
        let n = stream.read(&mut chunk).unwrap();
        if n == 0 {
            break;
        }
        streamed.extend_from_slice(&chunk[..n]);
    }

    assert_eq!(stream.sample_rate(), static_source.sample_rate);
    assert_eq!(stream.channels(), static_source.channels);
    assert_eq!(streamed.len(), static_source.samples.len());
    for (i, (a, b)) in streamed.iter().zip(static_source.samples.iter()).enumerate() {
        assert!((a - b).abs() < 1e-6, "sample {i} differs: streamed={a}, static={b}");
    }
}

#[test]
#[ignore = "Long test"]
fn stream_read_in_small_chunks_matches_full_read() {
    let mut stream_a = SymphoniaLoader::stream(TEST_WAV_SONG).unwrap();
    let mut full = Vec::new();
    let mut big = [0.0f32; 8192];
    loop {
        let n = stream_a.read(&mut big).unwrap();
        if n == 0 {
            break;
        }
        full.extend_from_slice(&big[..n]);
    }

    let mut stream_b = SymphoniaLoader::stream(TEST_WAV_SONG).unwrap();
    let mut piecemeal = Vec::new();
    let mut tiny = [0.0f32; 7]; // deliberately awkward size
    loop {
        let n = stream_b.read(&mut tiny).unwrap();
        if n == 0 {
            break;
        }
        piecemeal.extend_from_slice(&tiny[..n]);
    }

    assert_eq!(full, piecemeal, "read chunk size shouldn't change the decoded output");
}


#[test]
fn create_and_auto_produce_working_sources() {
    let mut static_src = SymphoniaLoader::create(TEST_SHORT, LoadMode::Static).unwrap();
    let mut stream_src = SymphoniaLoader::create(TEST_SHORT, LoadMode::Streaming).unwrap();
    let mut auto_src = SymphoniaLoader::auto(TEST_SHORT).unwrap();

    let mut buf = [0.0f32; 4096];
    assert!(static_src.read(&mut buf).unwrap() > 0);
    assert!(stream_src.read(&mut buf).unwrap() > 0);
    assert!(auto_src.read(&mut buf).unwrap() > 0);
}

#[test]
fn source_reaches_eof() {
    let mut source = SymphoniaLoader::decode(TEST_SHORT).unwrap();

    let mut buffer = [0.0; 4096];

    loop {
        let n = source.read(&mut buffer).unwrap();

        if n == 0 {
            break;
        }
    }

    let n = source.read(&mut buffer).unwrap();

    assert_eq!(n, 0);
}