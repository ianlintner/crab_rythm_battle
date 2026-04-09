use std::f64::consts::PI;

fn main() {
    // Ensure output directory exists
    std::fs::create_dir_all("assets/audio").expect("Failed to create assets/audio directory");

    let sample_rate: u32 = 44100;
    let bpm: f64 = 90.0;
    let beat_interval: f64 = 60.0 / bpm;
    let sixteenth: f64 = beat_interval / 4.0;

    // Total length: 32 bars × 16 sixteenth notes
    let total_sixteenths: usize = 32 * 16;
    let total_duration: f64 = total_sixteenths as f64 * sixteenth;
    let total_samples: usize = (total_duration * sample_rate as f64).ceil() as usize;

    let mut buffer = vec![0.0f64; total_samples];

    // ── Instrument generators ────────────────────────────────────────────────

    // Kick drum: short 60 Hz sine burst with exponential decay + click
    fn kick(buf: &mut [f64], start: usize, sample_rate: u32) {
        let sr = sample_rate as f64;
        let freq = 60.0;
        let duration_secs = 0.35;
        let n = ((duration_secs * sr) as usize).min(buf.len() - start);
        for i in 0..n {
            let t = i as f64 / sr;
            let env = (-t * 14.0).exp();
            // slight pitch drop for punchiness
            let freq_t = freq + 80.0 * (-t * 30.0).exp();
            let click = if i < (0.003 * sr) as usize {
                0.4 * (1.0 - t / 0.003) * rand_f64(i as u64 ^ 0xDEAD)
            } else {
                0.0
            };
            let sample = env * (2.0 * PI * freq_t * t).sin() * 0.9 + click;
            buf[start + i] = (buf[start + i] + sample).clamp(-1.0, 1.0);
        }
    }

    // Snare: white noise + 200 Hz sine, short decay
    fn snare(buf: &mut [f64], start: usize, sample_rate: u32) {
        let sr = sample_rate as f64;
        let duration_secs = 0.18;
        let n = ((duration_secs * sr) as usize).min(buf.len() - start);
        for i in 0..n {
            let t = i as f64 / sr;
            let env = (-t * 22.0).exp();
            let noise = rand_f64(start as u64 + i as u64) * 2.0 - 1.0;
            let tone = (2.0 * PI * 200.0 * t).sin();
            let sample = env * (noise * 0.7 + tone * 0.3) * 0.75;
            buf[start + i] = (buf[start + i] + sample).clamp(-1.0, 1.0);
        }
    }

    // Closed hi-hat: short high-frequency noise burst
    fn hihat_closed(buf: &mut [f64], start: usize, sample_rate: u32) {
        let sr = sample_rate as f64;
        let duration_secs = 0.05;
        let n = ((duration_secs * sr) as usize).min(buf.len() - start);
        for i in 0..n {
            let t = i as f64 / sr;
            let env = (-t * 80.0).exp();
            let noise = rand_f64(start as u64 * 7 + i as u64) * 2.0 - 1.0;
            // Band-pass around 8000 Hz via multiple sine beats (cheap approximation)
            let tone = (2.0 * PI * 8000.0 * t).sin()
                + 0.5 * (2.0 * PI * 10000.0 * t).sin()
                + 0.3 * (2.0 * PI * 12000.0 * t).sin();
            let sample = env * (noise * 0.4 + tone * 0.1) * 0.4;
            buf[start + i] = (buf[start + i] + sample).clamp(-1.0, 1.0);
        }
    }

    // Open hi-hat: slightly longer
    fn hihat_open(buf: &mut [f64], start: usize, sample_rate: u32) {
        let sr = sample_rate as f64;
        let duration_secs = 0.18;
        let n = ((duration_secs * sr) as usize).min(buf.len() - start);
        for i in 0..n {
            let t = i as f64 / sr;
            let env = (-t * 18.0).exp();
            let noise = rand_f64(start as u64 * 13 + i as u64) * 2.0 - 1.0;
            let tone = (2.0 * PI * 8000.0 * t).sin()
                + 0.5 * (2.0 * PI * 10500.0 * t).sin();
            let sample = env * (noise * 0.35 + tone * 0.1) * 0.38;
            buf[start + i] = (buf[start + i] + sample).clamp(-1.0, 1.0);
        }
    }

    // Bass note: sine wave at given freq, 8th note length with slight decay
    fn bass_note(buf: &mut [f64], start: usize, sample_rate: u32, freq: f64) {
        let sr = sample_rate as f64;
        let duration_secs = 0.22;
        let n = ((duration_secs * sr) as usize).min(buf.len() - start);
        for i in 0..n {
            let t = i as f64 / sr;
            let attack = if t < 0.01 { t / 0.01 } else { 1.0 };
            let decay = (-t * 5.0).exp();
            let env = attack * decay;
            let sample = env
                * ((2.0 * PI * freq * t).sin()
                    + 0.5 * (2.0 * PI * freq * 2.0 * t).sin()
                    + 0.25 * (2.0 * PI * freq * 3.0 * t).sin())
                * 0.5;
            buf[start + i] = (buf[start + i] + sample).clamp(-1.0, 1.0);
        }
    }

    // Simple deterministic pseudo-random (LCG)
    fn rand_f64(seed: u64) -> f64 {
        let v = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (v >> 11) as f64 / (1u64 << 53) as f64
    }

    // Bass note frequencies
    // E2=82.4 Hz, A2=110 Hz, G2=98 Hz, D2=73.4 Hz
    let bass_freqs: [f64; 4] = [82.4, 110.0, 98.0, 73.4];

    // Beat pattern is 2 bars (32 sixteenth notes)
    // We loop it for 32 bars total
    let pattern_length: usize = 32; // sixteenth notes in 2 bars

    // Kick: positions 0, 8 in 2-bar pattern
    let kick_positions: &[usize] = &[0, 8];
    // Snare: positions 4, 12
    let snare_positions: &[usize] = &[4, 12];
    // Hi-hat closed: every other 16th (8th notes): 0,2,4,6,8,10,12,14
    let hihat_positions: &[usize] = &[0, 2, 4, 6, 8, 10, 12, 14];
    // Open hi-hat variation at position 14
    let hihat_open_positions: &[usize] = &[14];
    // Bass: syncopated 8th note pattern
    let bass_positions: &[(usize, usize)] = &[
        (0, 0),  // E2
        (3, 2),  // G2
        (6, 1),  // A2
        (8, 3),  // D2
        (11, 0), // E2
        (14, 1), // A2
    ];

    let num_pattern_repeats = total_sixteenths / pattern_length;

    for rep in 0..num_pattern_repeats {
        let base_16th = rep * pattern_length;

        for &pos in kick_positions {
            let sixteenth_idx = base_16th + pos;
            let sample_start = (sixteenth_idx as f64 * sixteenth * sample_rate as f64) as usize;
            if sample_start < total_samples {
                kick(&mut buffer, sample_start, sample_rate);
            }
        }

        for &pos in snare_positions {
            let sixteenth_idx = base_16th + pos;
            let sample_start = (sixteenth_idx as f64 * sixteenth * sample_rate as f64) as usize;
            if sample_start < total_samples {
                snare(&mut buffer, sample_start, sample_rate);
            }
        }

        for &pos in hihat_positions {
            // Skip position 14 because we'll use open hi-hat there
            if hihat_open_positions.contains(&pos) {
                continue;
            }
            let sixteenth_idx = base_16th + pos;
            let sample_start = (sixteenth_idx as f64 * sixteenth * sample_rate as f64) as usize;
            if sample_start < total_samples {
                hihat_closed(&mut buffer, sample_start, sample_rate);
            }
        }

        for &pos in hihat_open_positions {
            let sixteenth_idx = base_16th + pos;
            let sample_start = (sixteenth_idx as f64 * sixteenth * sample_rate as f64) as usize;
            if sample_start < total_samples {
                hihat_open(&mut buffer, sample_start, sample_rate);
            }
        }

        for &(pos, freq_idx) in bass_positions {
            let sixteenth_idx = base_16th + pos;
            let sample_start = (sixteenth_idx as f64 * sixteenth * sample_rate as f64) as usize;
            if sample_start < total_samples {
                bass_note(&mut buffer, sample_start, sample_rate, bass_freqs[freq_idx]);
            }
        }
    }

    // Normalize to avoid clipping
    let max_val = buffer.iter().map(|x| x.abs()).fold(0.0_f64, f64::max);
    let gain = if max_val > 0.001 { 0.9 / max_val } else { 1.0 };

    // Convert to i16
    let samples: Vec<i16> = buffer
        .iter()
        .map(|&s| ((s * gain).clamp(-1.0, 1.0) * i16::MAX as f64) as i16)
        .collect();

    write_wav("assets/audio/beat.wav", &samples, sample_rate);
    println!("Generated beat.wav successfully ({} samples, {:.1}s)", samples.len(), total_duration);
}

fn write_wav(path: &str, samples: &[i16], sample_rate: u32) {
    use std::io::Write;
    let mut file = std::fs::File::create(path).unwrap();
    let num_samples = samples.len() as u32;
    let num_channels: u16 = 1;
    let bits_per_sample: u16 = 16;
    let byte_rate = sample_rate * num_channels as u32 * bits_per_sample as u32 / 8;
    let block_align = num_channels * bits_per_sample / 8;
    let data_size = num_samples * 2;

    // RIFF header
    file.write_all(b"RIFF").unwrap();
    file.write_all(&(36 + data_size).to_le_bytes()).unwrap();
    file.write_all(b"WAVE").unwrap();
    // fmt chunk
    file.write_all(b"fmt ").unwrap();
    file.write_all(&16u32.to_le_bytes()).unwrap();
    file.write_all(&1u16.to_le_bytes()).unwrap(); // PCM
    file.write_all(&num_channels.to_le_bytes()).unwrap();
    file.write_all(&sample_rate.to_le_bytes()).unwrap();
    file.write_all(&byte_rate.to_le_bytes()).unwrap();
    file.write_all(&block_align.to_le_bytes()).unwrap();
    file.write_all(&bits_per_sample.to_le_bytes()).unwrap();
    // data chunk
    file.write_all(b"data").unwrap();
    file.write_all(&data_size.to_le_bytes()).unwrap();
    for &s in samples {
        file.write_all(&s.to_le_bytes()).unwrap();
    }
}
