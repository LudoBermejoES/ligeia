use id3::{Tag, TagLike, Frame, Content, frame::ExtendedText};
use crate::models::AudioFile;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::core::audio::SampleBuffer;
use std::fs::File;
use aubio_rs::{Tempo, OnsetMode};

pub struct AudioHandler;

impl AudioHandler {
    pub fn load_audio_file_metadata(file_path: &str) -> Result<AudioFile, String> {
        let mut audio_file = AudioFile {
            id: None,
            file_path: file_path.to_string(),
            // Initialize all fields as None
            title: None,
            artist: None,
            album: None,
            album_artist: None,
            genre: None,
            year: None,
            date: None,
            track_number: None,
            total_tracks: None,
            disc_number: None,
            total_discs: None,
            duration: None,
            composer: None,
            conductor: None,
            lyricist: None,
            original_artist: None,
            remixer: None,
            arranger: None,
            engineer: None,
            producer: None,
            dj_mixer: None,
            mixer: None,
            content_group: None,
            subtitle: None,
            initial_key: None,
            bpm: None,
            language: None,
            media_type: None,
            original_filename: None,
            original_lyricist: None,
            original_release_time: None,
            playlist_delay: None,
            recording_time: None,
            release_time: None,
            tagging_time: None,
            encoding_time: None,
            encoding_settings: None,
            encoded_by: None,
            copyright: None,
            file_owner: None,
            internet_radio_station_name: None,
            internet_radio_station_owner: None,
            isrc: None,
            publisher: None,
            mood: None,
            occasion: None,
            tempo: None,
            content_type: None,
            category: None,
        };

        if let Ok(tag) = Tag::read_from_path(file_path) {
            // Basic tags
            audio_file.title = tag.title().map(|s| s.to_string());
            audio_file.artist = tag.artist().map(|s| s.to_string());
            audio_file.album = tag.album().map(|s| s.to_string());
            audio_file.album_artist = tag.album_artist().map(|s| s.to_string());
            audio_file.genre = tag.genre().map(|s| s.to_string());
            audio_file.year = tag.year().map(|y| y as i32);
            audio_file.date = tag.date_recorded().map(|d| d.to_string());
            audio_file.track_number = tag.track();
            audio_file.total_tracks = tag.total_tracks();
            audio_file.disc_number = tag.disc();
            audio_file.total_discs = tag.total_discs();
            
            // Duration from file analysis (not from tag)  
            audio_file.duration = tag.duration().map(|d| d as f64);
            
            // Extended tags
            for frame in tag.frames() {
                match frame.id() {
                    "TCOM" => audio_file.composer = frame.content().text().map(|s| s.to_string()),
                    "TPE3" => audio_file.conductor = frame.content().text().map(|s| s.to_string()),
                    "TEXT" => audio_file.lyricist = frame.content().text().map(|s| s.to_string()),
                    "TOPE" => audio_file.original_artist = frame.content().text().map(|s| s.to_string()),
                    "TPE4" => audio_file.remixer = frame.content().text().map(|s| s.to_string()),
                    "TIPL" => {
                        // Involved people list - could contain arranger, engineer, producer, etc.
                        if let Some(text) = frame.content().text() {
                            if text.to_lowercase().contains("arranger") {
                                audio_file.arranger = Some(text.to_string());
                            } else if text.to_lowercase().contains("engineer") {
                                audio_file.engineer = Some(text.to_string());
                            } else if text.to_lowercase().contains("producer") {
                                audio_file.producer = Some(text.to_string());
                            } else if text.to_lowercase().contains("dj") {
                                audio_file.dj_mixer = Some(text.to_string());
                            } else if text.to_lowercase().contains("mix") {
                                audio_file.mixer = Some(text.to_string());
                            }
                        }
                    },
                    "TIT1" => audio_file.content_group = frame.content().text().map(|s| s.to_string()),
                    "TIT3" => audio_file.subtitle = frame.content().text().map(|s| s.to_string()),
                    "TKEY" => audio_file.initial_key = frame.content().text().map(|s| s.to_string()),
                    "TBPM" => audio_file.bpm = frame.content().text().and_then(|s| s.parse().ok()),
                    "TLAN" => audio_file.language = frame.content().text().map(|s| s.to_string()),
                    "TMED" => audio_file.media_type = frame.content().text().map(|s| s.to_string()),
                    "TOFN" => audio_file.original_filename = frame.content().text().map(|s| s.to_string()),
                    "TOLY" => audio_file.original_lyricist = frame.content().text().map(|s| s.to_string()),
                    "TORY" => audio_file.original_release_time = frame.content().text().map(|s| s.to_string()),
                    "TDLY" => audio_file.playlist_delay = frame.content().text().and_then(|s| s.parse().ok()),
                    "TDRC" => audio_file.recording_time = frame.content().text().map(|s| s.to_string()),
                    "TDRL" => audio_file.release_time = frame.content().text().map(|s| s.to_string()),
                    "TDTG" => audio_file.tagging_time = frame.content().text().map(|s| s.to_string()),
                    "TDEN" => audio_file.encoding_time = frame.content().text().map(|s| s.to_string()),
                    "TSSE" => audio_file.encoding_settings = frame.content().text().map(|s| s.to_string()),
                    "TENC" => audio_file.encoded_by = frame.content().text().map(|s| s.to_string()),
                    "TCOP" => audio_file.copyright = frame.content().text().map(|s| s.to_string()),
                    "TOWN" => audio_file.file_owner = frame.content().text().map(|s| s.to_string()),
                    "TRSN" => audio_file.internet_radio_station_name = frame.content().text().map(|s| s.to_string()),
                    "TRSO" => audio_file.internet_radio_station_owner = frame.content().text().map(|s| s.to_string()),
                    "TSRC" => audio_file.isrc = frame.content().text().map(|s| s.to_string()),
                    "TPUB" => audio_file.publisher = frame.content().text().map(|s| s.to_string()),
                    "TMOO" => audio_file.mood = frame.content().text().map(|s| s.to_string()),
                    _ => {} // Ignore unknown frames
                }
            }
        }

        Ok(audio_file)
    }

    pub fn update_audio_file_tags(file_path: &str, updates: &AudioFile) -> Result<(), String> {
        // Check if file exists before attempting to update tags
        if !std::path::Path::new(file_path).exists() {
            return Err(format!("File not found: {}", file_path));
        }
        
        let mut tag = Tag::read_from_path(file_path).unwrap_or_else(|_| Tag::new());
        
        // Update basic tags
        if let Some(title) = &updates.title {
            tag.set_title(title);
        }
        if let Some(artist) = &updates.artist {
            tag.set_artist(artist);
        }
        if let Some(album) = &updates.album {
            tag.set_album(album);
        }
        if let Some(album_artist) = &updates.album_artist {
            tag.set_album_artist(album_artist);
        }
        if let Some(genre) = &updates.genre {
            tag.set_genre(genre);
        }
        if let Some(year) = updates.year {
            tag.set_year(year);
        }
        if let Some(track) = updates.track_number {
            tag.set_track(track);
        }
        if let Some(total_tracks) = updates.total_tracks {
            tag.set_total_tracks(total_tracks);
        }
        if let Some(disc) = updates.disc_number {
            tag.set_disc(disc);
        }
        if let Some(total_discs) = updates.total_discs {
            tag.set_total_discs(total_discs);
        }
        
        // Update extended tags using frame manipulation
        if let Some(composer) = &updates.composer {
            tag.set_text("TCOM", composer);
        }
        if let Some(conductor) = &updates.conductor {
            tag.set_text("TPE3", conductor);
        }
        if let Some(lyricist) = &updates.lyricist {
            tag.set_text("TEXT", lyricist);
        }
        if let Some(original_artist) = &updates.original_artist {
            tag.set_text("TOPE", original_artist);
        }
        if let Some(remixer) = &updates.remixer {
            tag.set_text("TPE4", remixer);
        }
        if let Some(content_group) = &updates.content_group {
            tag.set_text("TIT1", content_group);
        }
        if let Some(subtitle) = &updates.subtitle {
            tag.set_text("TIT3", subtitle);
        }
        if let Some(initial_key) = &updates.initial_key {
            tag.set_text("TKEY", initial_key);
        }
        if let Some(bpm) = updates.bpm {
            tag.set_text("TBPM", &bpm.to_string());
        }
        if let Some(language) = &updates.language {
            tag.set_text("TLAN", language);
        }
        if let Some(media_type) = &updates.media_type {
            tag.set_text("TMED", media_type);
        }
        if let Some(copyright) = &updates.copyright {
            tag.set_text("TCOP", copyright);
        }
        if let Some(encoded_by) = &updates.encoded_by {
            tag.set_text("TENC", encoded_by);
        }
        if let Some(publisher) = &updates.publisher {
            tag.set_text("TPUB", publisher);
        }
        if let Some(mood) = &updates.mood {
            tag.set_text("TMOO", mood);
        }
        
        // Write the updated tag back to the file
        tag.write_to_path(file_path, id3::Version::Id3v24)
            .map_err(|e| format!("Failed to write tags: {}", e))?;
        
        Ok(())
    }

    pub fn write_rpg_tags_to_file(file_path: &str, rpg_tags: &[(String, Vec<String>)]) -> Result<(), String> {
        // Check if file exists before attempting to write RPG tags
        if !std::path::Path::new(file_path).exists() {
            return Err(format!("File not found: {}", file_path));
        }
        
        let mut tag = Tag::read_from_path(file_path).unwrap_or_else(|_| Tag::new());
        
        // Remove existing RPG TXXX frames first
        tag.remove("TXXX:Occasion");
        tag.remove("TXXX:Keywords");
        tag.remove("TXXX:Quality");
        
        // Group tags by type
        let mut occasions = Vec::new();
        let mut keywords = Vec::new();
        let mut quality = None;
        
        for (tag_type, tag_values) in rpg_tags {
            match tag_type.as_str() {
                "occasion" => occasions.extend(tag_values.clone()),
                "keyword" => keywords.extend(tag_values.clone()),
                "quality" => quality = tag_values.first().cloned(),
                _ => {} // Skip other tag types
            }
        }
        
        // Write TXXX:Occasion (semicolon-separated as per TAGS.md)
        if !occasions.is_empty() {
            let occasion_value = occasions.join("; ");
            let extended_text = ExtendedText {
                description: "Occasion".to_string(),
                value: occasion_value,
            };
            tag.add_frame(Frame::with_content("TXXX", Content::ExtendedText(extended_text)));
        }
        
        // Write TXXX:Keywords (semicolon-separated as per TAGS.md)
        if !keywords.is_empty() {
            let keywords_value = keywords.join("; ");
            let extended_text = ExtendedText {
                description: "Keywords".to_string(),
                value: keywords_value,
            };
            tag.add_frame(Frame::with_content("TXXX", Content::ExtendedText(extended_text)));
        }
        
        // Write TXXX:Quality
        if let Some(quality_value) = quality {
            let extended_text = ExtendedText {
                description: "Quality".to_string(),
                value: quality_value,
            };
            tag.add_frame(Frame::with_content("TXXX", Content::ExtendedText(extended_text)));
        }
        
        // Write the updated tag back to the file
        tag.write_to_path(file_path, id3::Version::Id3v24)
            .map_err(|e| format!("Failed to write RPG tags: {}", e))?;
        
        Ok(())
    }

    pub fn calculate_audio_duration(file_path: &str) -> Result<f64, String> {
        // Open the media source
        let src = File::open(file_path)
            .map_err(|e| format!("Failed to open file: {}", e))?;
        
        // Create the media source stream
        let mss = MediaSourceStream::new(Box::new(src), Default::default());
        
        // Create a probe hint using the file extension
        let mut hint = Hint::new();
        if let Some(extension) = std::path::Path::new(file_path).extension() {
            if let Some(ext_str) = extension.to_str() {
                hint.with_extension(ext_str);
            }
        }
        
        // Use the default options for metadata and format readers
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();
        
        // Probe the media source
        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &fmt_opts, &meta_opts)
            .map_err(|e| format!("Failed to probe format: {}", e))?;
        
        // Get the instantiated format reader
        let format = probed.format;
        
        // Find the first audio track with a known duration
        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
            .ok_or("No suitable audio track found")?;
        
        let track_id = track.id;
        
        // Calculate duration if we have the necessary information
        if let (Some(n_frames), Some(sample_rate)) = (
            track.codec_params.n_frames,
            track.codec_params.sample_rate,
        ) {
            let duration_seconds = n_frames as f64 / sample_rate as f64;
            Ok(duration_seconds)
        } else {
            // If we can't get duration from metadata, count frames
            Self::calculate_duration_by_decoding(format, track_id)
        }
    }

    fn calculate_duration_by_decoding(
        mut format: Box<dyn symphonia::core::formats::FormatReader>,
        track_id: u32,
    ) -> Result<f64, String> {
        // Get codec parameters
        let track = format
            .tracks()
            .iter()
            .find(|t| t.id == track_id)
            .ok_or("Track not found")?;
        
        let codec_params = &track.codec_params;
        let sample_rate = codec_params.sample_rate.ok_or("No sample rate found")?;
        
        // Create decoder
        let mut decoder = symphonia::default::get_codecs()
            .make(&codec_params, &Default::default())
            .map_err(|e| format!("Failed to create decoder: {}", e))?;
        
        let mut total_frames = 0u64;
        
        // Decode packets and count frames
        loop {
            // Get the next packet from the media format
            let packet = match format.next_packet() {
                Ok(packet) => packet,
                Err(symphonia::core::errors::Error::ResetRequired) => {
                    // The track list has been changed. Re-examine it and create a new set of decoders,
                    // then restart the decode loop. This is an advanced feature that may not be
                    // implemented in all format readers.
                    break;
                }
                Err(symphonia::core::errors::Error::IoError(e)) 
                    if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                    // End of stream
                    break;
                }
                Err(e) => return Err(format!("Decode error: {}", e)),
            };
            
            // If the packet does not belong to the selected track, skip over it
            if packet.track_id() != track_id {
                continue;
            }
            
            // Decode the packet into audio samples
            match decoder.decode(&packet) {
                Ok(decoded) => {
                    // Count the frames in this packet
                    total_frames += decoded.frames() as u64;
                }
                Err(symphonia::core::errors::Error::IoError(e)) 
                    if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                    // End of stream
                    break;
                }
                Err(symphonia::core::errors::Error::DecodeError(_)) => {
                    // Decode error, skip this packet
                    continue;
                }
                Err(e) => return Err(format!("Decode error: {}", e)),
            }
        }
        
        // Calculate duration from total frames and sample rate
        let duration_seconds = total_frames as f64 / sample_rate as f64;
        Ok(duration_seconds)
    }

    pub fn calculate_audio_bpm(file_path: &str) -> Result<f32, String> {
        // Open the media source
        let src = File::open(file_path)
            .map_err(|e| format!("Failed to open file: {}", e))?;
        
        // Create the media source stream
        let mss = MediaSourceStream::new(Box::new(src), Default::default());
        
        // Create a probe hint using the file extension
        let mut hint = Hint::new();
        if let Some(extension) = std::path::Path::new(file_path).extension() {
            if let Some(ext_str) = extension.to_str() {
                hint.with_extension(ext_str);
            }
        }
        
        // Use the default options for metadata and format readers
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();
        
        // Probe the media source
        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &fmt_opts, &meta_opts)
            .map_err(|e| format!("Failed to probe format: {}", e))?;
        
        // Get the instantiated format reader
        let mut format = probed.format;
        
        // Find the first audio track
        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
            .ok_or("No suitable audio track found")?;
        
        let track_id = track.id;
        let codec_params = &track.codec_params;
        let sample_rate = codec_params.sample_rate.ok_or("No sample rate found")?;
        
        // Create decoder
        let mut decoder = symphonia::default::get_codecs()
            .make(&codec_params, &Default::default())
            .map_err(|e| format!("Failed to create decoder: {}", e))?;
        
        // Initialize aubio tempo detection
        let mut tempo = Tempo::new(OnsetMode::Energy, 1024, 512, sample_rate)
            .map_err(|e| format!("Failed to create tempo detector: {:?}", e))?;
        
        // Get channels
        let channels = codec_params.channels.unwrap_or(symphonia::core::audio::Channels::FRONT_LEFT | symphonia::core::audio::Channels::FRONT_RIGHT);
        
        // Create signal spec for sample buffer using simple constructor
        let spec = symphonia::core::audio::SignalSpec::new(sample_rate, channels);
        
        // Sample buffer for f32 samples
        let mut sample_buf = SampleBuffer::<f32>::new(
            codec_params.n_frames.unwrap_or(1024) as u64,
            spec
        );
        
        let mut total_samples_processed = 0;
        let max_samples = sample_rate * 60; // Process up to 60 seconds for BPM detection
        
        // Process packets for BPM detection
        loop {
            if total_samples_processed >= max_samples {
                break; // Process enough audio for reliable BPM detection
            }
            
            // Get the next packet from the media format
            let packet = match format.next_packet() {
                Ok(packet) => packet,
                Err(symphonia::core::errors::Error::ResetRequired) => break,
                Err(symphonia::core::errors::Error::IoError(e)) 
                    if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(_) => continue,
            };
            
            // If the packet does not belong to the selected track, skip over it
            if packet.track_id() != track_id {
                continue;
            }
            
            // Decode the packet into audio samples
            match decoder.decode(&packet) {
                Ok(decoded) => {
                    // Store frame count before moving decoded
                    let frames = decoded.frames();
                    
                    // Convert to f32 samples
                    sample_buf.copy_interleaved_ref(decoded);
                    let samples = sample_buf.samples();
                    
                    // Process samples in chunks suitable for aubio (hop size = 512)
                    for chunk in samples.chunks(512) {
                        if chunk.len() == 512 {
                            tempo.do_result(chunk)
                                .map_err(|e| format!("Failed to process tempo: {:?}", e))?;
                        }
                    }
                    
                    total_samples_processed += frames as u32;
                }
                Err(symphonia::core::errors::Error::IoError(e)) 
                    if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(symphonia::core::errors::Error::DecodeError(_)) => continue,
                Err(_) => continue,
            }
        }
        
        // Get the detected BPM
        let bpm = tempo.get_bpm();
        
        if bpm > 0.0 && bpm < 300.0 { // Reasonable BPM range
            Ok(bpm)
        } else {
            Err("Could not detect valid BPM".to_string())
        }
    }

    pub fn calculate_duration_and_bpm(file_path: &str) -> Result<(Option<f64>, Option<f32>), String> {
        let duration = match Self::calculate_audio_duration(file_path) {
            Ok(d) => Some(d),
            Err(e) => {
                eprintln!("Failed to calculate duration for {}: {}", file_path, e);
                None
            }
        };
        
        let bpm = match Self::calculate_audio_bpm(file_path) {
            Ok(b) => Some(b),
            Err(e) => {
                eprintln!("Failed to calculate BPM for {}: {}", file_path, e);
                None
            }
        };
        
        if duration.is_none() && bpm.is_none() {
            Err("Failed to calculate both duration and BPM".to_string())
        } else {
            Ok((duration, bpm))
        }
    }
}