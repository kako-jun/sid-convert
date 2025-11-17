use midly::{MetaMessage, MidiMessage, Smf, TrackEventKind};
use std::collections::HashMap;
use std::fs;

use crate::convert::{is_bass_range, note_to_pitch, ticks_to_duration};
use crate::export::Output;

pub struct MidiParser {
    _data: Vec<u8>,
    smf: Smf<'static>,
}

#[derive(Debug)]
pub struct TrackInfo {
    pub index: usize,
    pub name: Option<String>,
    pub bass_note_count: usize,
    pub total_note_count: usize,
}

impl MidiParser {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let data = fs::read(path)?;
        // Leak the data to get a 'static reference
        let data_static: &'static [u8] = Box::leak(data.clone().into_boxed_slice());
        let smf = Smf::parse(data_static)?;

        Ok(Self { _data: data, smf })
    }

    pub fn get_ticks_per_quarter(&self) -> u16 {
        match self.smf.header.timing {
            midly::Timing::Metrical(tpq) => tpq.as_int(),
            midly::Timing::Timecode(_, _) => 480, // Default fallback
        }
    }

    /// Find all potential bass tracks
    pub fn find_bass_tracks(&self) -> Vec<TrackInfo> {
        let mut track_infos = Vec::new();

        for (index, track) in self.smf.tracks.iter().enumerate() {
            let mut name: Option<String> = None;
            let mut bass_note_count = 0;
            let mut total_note_count = 0;

            for event in track {
                match event.kind {
                    TrackEventKind::Meta(MetaMessage::TrackName(bytes)) => {
                        name = String::from_utf8(bytes.to_vec()).ok();
                    }
                    TrackEventKind::Midi {
                        message: MidiMessage::NoteOn { key, .. },
                        ..
                    } => {
                        total_note_count += 1;
                        if is_bass_range(key.as_int()) {
                            bass_note_count += 1;
                        }
                    }
                    _ => {}
                }
            }

            // Include track if it has "Bass" in name or mostly bass-range notes
            let is_bass_by_name = name
                .as_ref()
                .map(|n| n.to_lowercase().contains("bass"))
                .unwrap_or(false);
            let is_bass_by_range = total_note_count > 0
                && (bass_note_count as f64 / total_note_count as f64) > 0.7;

            if is_bass_by_name || is_bass_by_range {
                track_infos.push(TrackInfo {
                    index,
                    name,
                    bass_note_count,
                    total_note_count,
                });
            }
        }

        track_infos
    }

    /// Extract notes from a specific track
    pub fn extract_notes(&self, track_index: usize) -> Result<Output, Box<dyn std::error::Error>> {
        if track_index >= self.smf.tracks.len() {
            return Err("Track index out of bounds".into());
        }

        let track = &self.smf.tracks[track_index];
        let tpq = self.get_ticks_per_quarter();

        // Get track name
        let mut track_name = format!("Track {}", track_index);
        for event in track {
            if let TrackEventKind::Meta(MetaMessage::TrackName(bytes)) = event.kind {
                if let Ok(name) = String::from_utf8(bytes.to_vec()) {
                    track_name = name;
                    break;
                }
            }
        }

        let mut output = Output::new(track_name);

        // Track active notes (key -> (start_tick, velocity))
        let mut active_notes: HashMap<u8, u32> = HashMap::new();
        let mut current_tick = 0u32;

        for event in track {
            current_tick += event.delta.as_int();

            match event.kind {
                TrackEventKind::Midi {
                    message: MidiMessage::NoteOn { key, vel },
                    ..
                } => {
                    let note = key.as_int();
                    let velocity = vel.as_int();

                    if velocity > 0 {
                        // Note on
                        active_notes.insert(note, current_tick);
                    } else {
                        // Note off (velocity 0 is note off)
                        if let Some(start_tick) = active_notes.remove(&note) {
                            let duration_ticks = current_tick - start_tick;
                            let pitch = note_to_pitch(note);
                            let length = ticks_to_duration(duration_ticks, tpq);
                            output.add_note(start_tick, length, pitch);
                        }
                    }
                }
                TrackEventKind::Midi {
                    message: MidiMessage::NoteOff { key, .. },
                    ..
                } => {
                    let note = key.as_int();
                    if let Some(start_tick) = active_notes.remove(&note) {
                        let duration_ticks = current_tick - start_tick;
                        let pitch = note_to_pitch(note);
                        let length = ticks_to_duration(duration_ticks, tpq);
                        output.add_note(start_tick, length, pitch);
                    }
                }
                _ => {}
            }
        }

        Ok(output)
    }
}
