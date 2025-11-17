mod convert;
mod export;
mod midi;

use clap::Parser;
use midi::MidiParser;
use std::process;

#[derive(Parser, Debug)]
#[command(
    name = "sid-convert",
    about = "Convert MIDI bass tracks to YAML format",
    version
)]
struct Args {
    /// Input MIDI file path
    #[arg(value_name = "FILE")]
    input: String,

    /// Track number to extract (0-indexed)
    #[arg(short, long)]
    track: Option<usize>,

    /// Output YAML file path
    #[arg(short, long, default_value = "sid-convert.yaml")]
    output: String,
}

fn main() {
    let args = Args::parse();

    // Parse MIDI file
    let parser = match MidiParser::from_file(&args.input) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error reading MIDI file: {}", e);
            process::exit(1);
        }
    };

    // Find bass tracks
    let bass_tracks = parser.find_bass_tracks();

    if bass_tracks.is_empty() {
        eprintln!("No bass tracks found in the MIDI file.");
        eprintln!("Try specifying a track manually with --track <number>");
        process::exit(1);
    }

    // Determine which track to use
    let track_index = if let Some(idx) = args.track {
        // User specified a track
        if idx >= bass_tracks.len() && !bass_tracks.iter().any(|t| t.index == idx) {
            eprintln!("Track {} not found or not a bass track.", idx);
            eprintln!("Available bass tracks:");
            for track in &bass_tracks {
                eprintln!(
                    "  Track {}: {} ({}/{} bass notes)",
                    track.index,
                    track.name.as_deref().unwrap_or("Unnamed"),
                    track.bass_note_count,
                    track.total_note_count
                );
            }
            process::exit(1);
        }
        idx
    } else if bass_tracks.len() == 1 {
        // Only one bass track found
        bass_tracks[0].index
    } else {
        // Multiple bass tracks found
        println!("Multiple bass tracks found. Please specify one with --track <number>:");
        for track in &bass_tracks {
            println!(
                "  Track {}: {} ({}/{} bass notes)",
                track.index,
                track.name.as_deref().unwrap_or("Unnamed"),
                track.bass_note_count,
                track.total_note_count
            );
        }
        process::exit(0);
    };

    // Extract notes from the selected track
    let output = match parser.extract_notes(track_index) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Error extracting notes: {}", e);
            process::exit(1);
        }
    };

    // Save to YAML file
    if let Err(e) = output.save_to_file(&args.output) {
        eprintln!("Error saving YAML file: {}", e);
        process::exit(1);
    }

    println!(
        "Successfully converted track '{}' to {}",
        output.track, args.output
    );
    println!("Extracted {} notes", output.notes.len());
}
