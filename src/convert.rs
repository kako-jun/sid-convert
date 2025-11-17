/// Convert MIDI note number to pitch name (e.g., 40 -> "E2")
pub fn note_to_pitch(note: u8) -> String {
    const NOTE_NAMES: [&str; 12] = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];

    let octave = (note as i32 / 12) - 1;
    let note_index = (note % 12) as usize;

    format!("{}{}", NOTE_NAMES[note_index], octave)
}

/// Convert MIDI ticks to note duration (quarter, eighth, half, whole)
/// Assumes 480 ticks per quarter note (standard)
pub fn ticks_to_duration(ticks: u32, tpq: u16) -> String {
    let tpq = tpq as u32;
    let quarters = ticks as f64 / tpq as f64;

    // Round to nearest standard duration
    if (quarters - 4.0).abs() < 0.25 {
        "whole".to_string()
    } else if (quarters - 2.0).abs() < 0.25 {
        "half".to_string()
    } else if (quarters - 1.0).abs() < 0.25 {
        "quarter".to_string()
    } else if (quarters - 0.5).abs() < 0.125 {
        "eighth".to_string()
    } else {
        // Default to the closest approximation
        if quarters >= 3.0 {
            "whole".to_string()
        } else if quarters >= 1.5 {
            "half".to_string()
        } else if quarters >= 0.75 {
            "quarter".to_string()
        } else {
            "eighth".to_string()
        }
    }
}

/// Check if a note is in bass range (E1 to G3)
pub fn is_bass_range(note: u8) -> bool {
    const E1: u8 = 28;
    const G3: u8 = 55;
    note >= E1 && note <= G3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_to_pitch() {
        assert_eq!(note_to_pitch(28), "E1");
        assert_eq!(note_to_pitch(40), "E2");
        assert_eq!(note_to_pitch(55), "G3");
    }

    #[test]
    fn test_ticks_to_duration() {
        assert_eq!(ticks_to_duration(480, 480), "quarter");
        assert_eq!(ticks_to_duration(240, 480), "eighth");
        assert_eq!(ticks_to_duration(960, 480), "half");
        assert_eq!(ticks_to_duration(1920, 480), "whole");
    }

    #[test]
    fn test_is_bass_range() {
        assert!(is_bass_range(28)); // E1
        assert!(is_bass_range(40)); // E2
        assert!(is_bass_range(55)); // G3
        assert!(!is_bass_range(27)); // Below E1
        assert!(!is_bass_range(56)); // Above G3
    }
}
