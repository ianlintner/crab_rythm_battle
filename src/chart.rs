use crate::constants::SIXTEENTH_NOTE;

#[derive(Clone, Debug)]
pub struct NoteEvent {
    pub hit_time: f64,
    pub lane: usize,
}

pub fn build_chart() -> Vec<NoteEvent> {
    let mut notes: Vec<NoteEvent> = Vec::new();

    // Helper macro/closure
    let mut add = |bar: usize, pos: usize, lane: usize| {
        let time = (bar * 16 + pos) as f64 * SIXTEENTH_NOTE;
        notes.push(NoteEvent { hit_time: time, lane });
    };

    // Bars 2-5: Simple quarter notes
    for bar in 2..6 {
        for &(pos, lane) in &[(0usize, 0usize), (4, 2), (8, 3), (12, 1)] {
            add(bar, pos, lane);
        }
    }

    // Bars 6-9: 8th notes
    for bar in 6..10 {
        for &(pos, lane) in &[
            (0usize, 0usize), (2, 1), (4, 2), (6, 3),
            (8, 3), (10, 2), (12, 1), (14, 0),
        ] {
            add(bar, pos, lane);
        }
    }

    // Bars 10-13: Syncopated
    for bar in 10..14 {
        for &(pos, lane) in &[
            (0usize, 0usize), (3, 2), (4, 3), (6, 1),
            (8, 2), (9, 3), (12, 0), (12, 1), (14, 2),
        ] {
            add(bar, pos, lane);
        }
    }

    // Bars 14-17: Dense
    for bar in 14..18 {
        for &(pos, lane) in &[
            (0usize, 0usize), (1, 1), (3, 2), (4, 3),
            (6, 0), (7, 1), (8, 2), (9, 3),
            (10, 0), (11, 1), (12, 2), (13, 3),
        ] {
            add(bar, pos, lane);
        }
    }

    // Bars 18-25: Boss mode - dense patterns with double notes
    for bar in 18..26 {
        let bar_mod = bar % 4;
        match bar_mod {
            0 => {
                for &(pos, lane) in &[
                    (0usize, 0usize), (0, 1), (2, 2), (4, 3), (4, 2),
                    (6, 1), (8, 0), (8, 3), (10, 2), (12, 1), (12, 0),
                    (14, 3), (14, 2),
                ] {
                    add(bar, pos, lane);
                }
            }
            1 => {
                for &(pos, lane) in &[
                    (0usize, 0usize), (1, 1), (2, 2), (3, 3),
                    (4, 3), (5, 2), (6, 1), (7, 0),
                    (8, 0), (9, 1), (10, 2), (11, 3),
                    (12, 2), (13, 1), (14, 0), (15, 3),
                ] {
                    add(bar, pos, lane);
                }
            }
            2 => {
                for &(pos, lane) in &[
                    (0usize, 0usize), (0, 2), (3, 1), (3, 3), (6, 0),
                    (8, 2), (8, 0), (9, 3), (11, 1), (12, 3), (14, 0), (14, 2),
                ] {
                    add(bar, pos, lane);
                }
            }
            _ => {
                for &(pos, lane) in &[
                    (0usize, 1usize), (2, 0), (2, 2), (4, 3), (5, 1),
                    (6, 0), (7, 2), (8, 1), (8, 3), (10, 0), (11, 3),
                    (12, 1), (13, 2), (14, 0), (15, 3),
                ] {
                    add(bar, pos, lane);
                }
            }
        }
    }

    // Bars 26-29: Finale - big finish
    for bar in 26..30 {
        for &(pos, lane) in &[
            (0usize, 0usize), (0, 1), (0, 2), (0, 3),
            (4, 0), (4, 3), (6, 1), (6, 2),
            (8, 0), (9, 1), (10, 2), (11, 3),
            (12, 0), (12, 1), (12, 2), (12, 3),
            (14, 0), (14, 3),
        ] {
            add(bar, pos, lane);
        }
    }

    // Sort by hit_time
    notes.sort_by(|a, b| a.hit_time.partial_cmp(&b.hit_time).unwrap());
    notes
}
