use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use colored::Colorize;

/// ===============================
/// Tracker core data structures
/// ===============================

const VISIBLE_ROWS: usize = 8;

#[derive(Clone, Copy, Debug)]
struct Note {
    pitch: Option<u8>, // MIDI note number (None = empty)
    volume: f32,       // 0.0–1.0
    stop: bool
}

impl Note {
    fn empty() -> Self {
        Self {
            pitch: None,
            volume: 0.0,
            stop: false,
        }
    }

    fn stop() -> Self {
        Self {
            pitch: None,
            volume: 0.0,
            stop: true,
        }
    }
}


#[derive(Clone, Debug)]
struct Row {
    channels: Vec<Note>,
}

#[derive(Clone, Debug)]
struct Pattern {
    rows: Vec<Row>,
}

#[derive(Clone, Debug)]
struct Song {
    patterns: Vec<Pattern>,
    order: Vec<usize>, // sequence of pattern indices
    bpm: f32,
    speed: u32, // ticks per row
}

/// ===============================
/// Project metadata and UI state
/// ===============================

struct ProjectMeta {
    name: String,
    author: String,
    track_names: Vec<String>, // 6 chars each ideally
    autostep: u32,
    ppq: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CursorRegion {
    Sequencer,
    Pattern,
    SideInfo,
    Command,
}

struct CursorState {
    region: CursorRegion,
    row: usize,
    col: usize,
    row_start: usize,
}

impl CursorState {
    fn new() -> Self {
        Self {
            region: CursorRegion::Pattern,
            row: 0,
            col: 0,
            row_start: 0,
        }
    }
}

struct UiState {
    project: ProjectMeta,
    cursor: CursorState,
    current_pattern_index: usize,
    show_cmd: bool,
    cmd_buffer: String,
    current_sample_name: String,
    is_playing: bool,
}

impl UiState {
    fn new(project: ProjectMeta) -> Self {
        Self {
            project,
            cursor: CursorState::new(),
            current_pattern_index: 0,
            show_cmd: false,
            cmd_buffer: String::new(),
            current_sample_name: "SQUARE1".to_string(),
            is_playing: false,
        }
    }
}

/// ===============================
/// Simple square wave synth
/// ===============================

struct ChannelState {
    freq: f32,
    phase: f32,
    volume: f32,
}

impl ChannelState {
    fn new() -> Self {
        Self {
            freq: 0.0,
            phase: 0.0,
            volume: 0.0,
        }
    }

    fn note_to_freq(note: u8) -> f32 {
        440.0 * 2f32.powf((note as f32 - 69.0) / 12.0)
    }

    fn trigger(&mut self, note: Note) {
        if let Some(p) = note.pitch {
            self.freq = Self::note_to_freq(p);
            self.volume = note.volume;
        }
    }

    fn sample(&mut self, dt: f32) -> f32 {
        if self.freq <= 0.0 || self.volume <= 0.0 {
            return 0.0;
        }

        self.phase += self.freq * dt;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        let s = if self.phase < 0.5 { 1.0 } else { -1.0 };
        s * self.volume
    }
}

/// ===============================
/// Playback engine
/// ===============================

struct Tracker {
    song: Song,
    current_order: usize,
    current_row: usize,
    tick: u32,
    tick_duration: Duration,
    channels: Vec<ChannelState>,
    is_playing: bool,
}

impl Tracker {
    fn new(song: Song, num_channels: usize) -> Self {
        let tick_duration = Duration::from_secs_f32(2.5 / song.bpm);

        Self {
            song,
            current_order: 0,
            current_row: 0,
            tick: 0,
            tick_duration,
            channels: (0..num_channels).map(|_| ChannelState::new()).collect(),
            is_playing: false,
        }
    }

    fn advance_tick(&mut self) {
        if !self.is_playing {
            return;
        }

        self.tick += 1;

        if self.tick >= self.song.speed {
            self.tick = 0;
            self.advance_row();
        }
    }

    fn advance_row(&mut self) {
        self.current_row += 1;

        let pat_idx = self.song.order[self.current_order];
        let pattern = &self.song.patterns[pat_idx];

        if self.current_row >= pattern.rows.len() {
            self.current_row = 0;
            self.current_order = (self.current_order + 1) % self.song.order.len();
        }

        let pattern = &self.song.patterns[self.song.order[self.current_order]];
        let row = &pattern.rows[self.current_row];

        for (i, note) in row.channels.iter().enumerate() {
            if note.stop {
                if let Some(ch) = self.channels.get_mut(i) {
                ch.volume = 0.0;   // silence channel
                ch.freq = 0.0;
                }
            } else if note.pitch.is_some() {
                if let Some(ch) = self.channels.get_mut(i) {
                    ch.trigger(*note);
                }
            }
        }
    }

    fn mix(&mut self, dt: f32) -> f32 {
        if !self.is_playing {
            return 0.0;
        }

        let mut sum = 0.0;
        for ch in &mut self.channels {
            sum += ch.sample(dt);
        }
        sum * 0.2
    }
}

/// ===============================
/// Example song + project metadata
/// ===============================

fn example_song_and_project() -> (Song, ProjectMeta) {
    let c4 = Some(60);
    let e4 = Some(64);
    let g4 = Some(67);

    let row = |a, b, c| Row {
        channels: vec![
            Note { pitch: a, volume: 0.8, stop: false },
            Note { pitch: b, volume: 0.8, stop: false },
            Note { pitch: c, volume: 0.8, stop: false },
        ],
    };

    let pattern = Pattern {
        rows: vec![
            row(c4, None, None),
            row(None, e4, None),
            row(None, None, g4),
            row(c4, e4, g4),
            Row {
                channels: vec![Note::empty(); 3],
            },
            Row {
                channels: vec![Note::empty(); 3],
            },
            Row {
                channels: vec![Note::empty(); 3],
            },
            Row {
                channels: vec![Note::empty(); 3],
            },
        ],
    };

    let song = Song {
        patterns: vec![pattern],
        order: vec![0, 0, 0, 0, 0, 0, 0, 0],
        bpm: 125.0,
        speed: 6,
    };

    let project = ProjectMeta {
        name: "DemoProject".to_string(),
        author: "Jonas".to_string(),
        track_names: vec![
            "TRK1  ".to_string(),
            "TRK2  ".to_string(),
            "TRK3  ".to_string(),
            "TRK4  ".to_string(),
            "TRK5  ".to_string(),
            "TRK6  ".to_string(),
            "TRK7  ".to_string(),
            "TRK8  ".to_string(),
        ],
        autostep: 1,
        ppq: 96,
    };

    (song, project)
}

/// ===============================
/// Rendering helpers
/// ===============================

/// Render a note as 6-character cell like "A#1C01" or "......"
fn format_note_cell(note: &Note) -> String {
    if note.stop {
        return "======".to_string();
    }

    if let Some(pitch) = note.pitch {
        let names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
        let n = (pitch % 12) as usize;
        let octave = (pitch / 12) as i32 - 1;
        let note_name = names[n];
        format!("{:<3}C01", format!("{}{}", note_name, octave))
    } else {
        "......".to_string()
    }
}

/// very simple "highlight": wrap in []
fn highlight(cell: &str) -> String {
    format!("{}", cell.reversed())
}

/// ===============================
/// UI drawing (ANSI, no raw mode)
/// ===============================

fn draw_ui(song: &Song, ui: &UiState, tracker: &Tracker) {
    // Clear screen and move cursor home
    print!("\x1B[2J\x1B[H");

    // Header
    println!(
        "{} - {} - EXPGSTracker",
        ui.project.name, ui.project.author
    );

    // Sequencer line
    let seq_str = song
        .order
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let label = format!("[{}]", p + 1);
            if ui.cursor.region == CursorRegion::Sequencer && ui.cursor.col == i {
                highlight(&label)
            } else {
                label
            }
        })
        .collect::<Vec<_>>()
        .join("");

    println!("{} |Pattern: {}", seq_str, ui.current_pattern_index + 1);
    println!("--------------------------------------------------------------------");

    // Track header, also should print as variable later.
    println!("TRACK1|TRACK2|TRACK3|TRACK4|TRACK5|TRACK6|TRACK7|TRACK8|INFO:");

    // Up arrow if scrolled
    if ui.cursor.row_start > 0 {
        println!("^^^^");
    }

    // Pattern rows (8 visible)
    let pattern = &song.patterns[ui.current_pattern_index];
    let start = ui.cursor.row_start;
    let end = (start + VISIBLE_ROWS).min(pattern.rows.len());

    for row_idx in start..end {
        let row = &pattern.rows[row_idx];

        // 8 tracks, update to variable later
        for ch in 0..9 {
            let base_cell = if ch < row.channels.len() {
                format_note_cell(&row.channels[ch])
            } else {
                "......".to_string()
            };

            let is_cursor = ui.cursor.region == CursorRegion::Pattern
                && ui.cursor.row == row_idx
                && ui.cursor.col == ch;

            if is_cursor {
                print!("{}", highlight(&base_cell));
            } else {
                print!("{}", base_cell);
            }
        }

        // Right panel, need to fix this to show row cursor is on since it seems to be wrong.
        let rel = row_idx - start;
        let is_cursor = ui.cursor.region == CursorRegion::SideInfo
            && ui.cursor.row == rel; // rel = 0..VISIBLE_ROWS-1

        match rel {
            0 => {
                if is_cursor { print!("{}", highlight("Sample:")); }
                else { print!(" Sample:"); }
                println!(" {}", ui.current_sample_name);
            }
            1 => {
                if is_cursor { print!("{}", highlight("Autostep:")); }
                else { print!(" Autostep:"); }
                println!(" {}", ui.project.autostep);
            }
            2 => {
                if is_cursor { print!("{}", highlight("PPQ:")); }
                else { print!(" PPQ:"); }
                println!(" {}", ui.project.ppq);
            }
            3 => {
                if is_cursor { print!("{}", highlight("BPM:")); }
                else { print!(" BPM:"); }
                println!(" {}", song.bpm);
            }
            _ => println!(),
        }
    }

    // Down arrow if more rows exist
    if end < pattern.rows.len() {
        println!("\\/\\/\\/");
    }

    // Info line
    println!(
        "Order: {} Row: {} Tick: {} | Playing: {}",
        tracker.current_order,
        tracker.current_row,
        tracker.tick,
        if ui.is_playing { "Yes" } else { "No" }
    );

    // CMD line only shows if we enter command mode
    if ui.show_cmd {
        print!("CMD: {}", ui.cmd_buffer);
    }

    io::stdout().flush().ok();
}

/// ===============================
/// Input handling
/// ===============================

fn handle_key(song: &mut Song, ui: &mut UiState, key: KeyEvent) -> bool {
    // returns true if UI needs redraw
    let mut changed = false;

    if ui.show_cmd {
        match key.code {
            KeyCode::Esc => {
                ui.show_cmd = false;
                ui.cmd_buffer.clear();
                ui.cursor.region = CursorRegion::Pattern;
                changed = true;
            }
            KeyCode::Enter => {
                // eventually parse commands here
                ui.show_cmd = false;
                ui.cmd_buffer.clear();
                ui.cursor.region = CursorRegion::Pattern;
                changed = true;
            }
            KeyCode::Char(c) => {
                ui.cmd_buffer.push(c);
                changed = true;
            }
            KeyCode::Backspace => {
                ui.cmd_buffer.pop();
                changed = true;
            }
            _ => {}
        }
        return changed;
    }

    match key.code {
        KeyCode::Char('`') => {
            ui.show_cmd = true;
            ui.cursor.region = CursorRegion::Command;
            changed = true;
        }
        KeyCode::Char(' ') => {
            // toggle playback
            ui.is_playing = !ui.is_playing;
            changed = true;
        }
        KeyCode::Char('s') => {
            ui.cursor.region = CursorRegion::Sequencer;
            ui.cursor.row = 0;
            ui.cursor.col = 0;
            changed = true;
        }
        KeyCode::Char('p') => {
            ui.cursor.region = CursorRegion::Pattern;
            changed = true;
        }
        KeyCode::Char('i') => {
            ui.cursor.region = CursorRegion::SideInfo;
            changed = true;
        }
        KeyCode::Left => match ui.cursor.region {
            CursorRegion::Pattern => {
                if ui.cursor.col > 0 {
                    ui.cursor.col -= 1;
                    changed = true;
                }
            }
            CursorRegion::Sequencer => {
                if ui.cursor.col > 0 {
                    ui.cursor.col -= 1;
                    changed = true;
                }
            }
            _ => {}
        },
        KeyCode::Right => match ui.cursor.region {
            CursorRegion::Pattern => {
                ui.cursor.col += 1;
                changed = true;
            }
            CursorRegion::Sequencer => {
                ui.cursor.col += 1;
                changed = true;
            }
            _ => {}
        },
        KeyCode::Up => match ui.cursor.region {
            CursorRegion::Pattern => {
                if ui.cursor.row > 0 {
                    ui.cursor.row -= 1;
                    changed = true;
                }
            }
            _ => {}
        },
        KeyCode::Down => match ui.cursor.region {
            CursorRegion::Pattern => {
                let pattern_len = song.patterns[ui.current_pattern_index].rows.len();

                // If cursor is not at bottom of visible window, just move it
                if ui.cursor.row < ui.cursor.row_start + (VISIBLE_ROWS - 1)
                    && ui.cursor.row + 1 < pattern_len
                {
                    ui.cursor.row += 1;
                }
                // Otherwise scroll the window
                else if ui.cursor.row_start + VISIBLE_ROWS < pattern_len {
                    ui.cursor.row_start += 1;
                    ui.cursor.row += 1;
                }

                changed = true;
            }
            _ => {}
        },
        KeyCode::Char(c) => {
            // note entry
            if ui.cursor.region == CursorRegion::Pattern {
                if let Some(pattern) = song.patterns.get_mut(ui.current_pattern_index) {
                    let row = ui.cursor.row.min(pattern.rows.len().saturating_sub(1));
                    if pattern.rows[row].channels.is_empty() {
                        pattern.rows[row].channels = vec![Note::empty(); 8];
                    }
                    let col = ui.cursor.col.min(pattern.rows[row].channels.len().saturating_sub(1));
                    let note = &mut pattern.rows[row].channels[col];

                    match c {
                        'z' => note.pitch = Some(60), // C4
                        'x' => note.pitch = Some(62),
                        'c' => note.pitch = Some(64),
                        'v' => note.pitch = Some(65),
                        'b' => note.pitch = Some(67),
                        'n' => note.pitch = Some(69),
                        'm' => note.pitch = Some(71),
                        '-' => *note = Note::empty(),
                        '0' => *note = Note::stop(),
                        _ => {}
                    }
                    changed = true;
                }
            }
        }
        KeyCode::F(1) => {
            ui.cursor.region = CursorRegion::Sequencer;
            ui.cursor.row = 0;
            ui.cursor.col = 0;
            changed = true;
        }

        KeyCode::F(2) => {
            ui.cursor.region = CursorRegion::SideInfo;
            ui.cursor.row = 0; // or whichever field you want selected first
            ui.cursor.col = 0;
            changed = true;
        }
        KeyCode::Enter => {
            match ui.cursor.region {
                CursorRegion::SideInfo => {
                    ui.show_cmd = true;
                    ui.cmd_buffer.clear();
                    ui.cursor.region = CursorRegion::Command;
                    changed = true;
                }
                CursorRegion::Sequencer => {
                    ui.show_cmd = true;
                    ui.cmd_buffer.clear();
                    ui.cursor.region = CursorRegion::Command;
                    changed = true;
                }
                _ => {}
            }
        }
        _ => {}
    }

    changed
}

/// ===============================
/// Audio output
/// ===============================

fn build_stream_f32(
    device: cpal::Device,
    config: cpal::StreamConfig,
    tracker: Arc<Mutex<Tracker>>,
) -> cpal::Stream {
    let sample_rate = config.sample_rate.0 as f32;
    let mut last_tick = Instant::now();

    let err_fn = |err| eprintln!("Stream error: {:?}", err);

    device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _info: &cpal::OutputCallbackInfo| {
                let mut tr = tracker.lock().unwrap();

                for sample in data.iter_mut() {
                    let now = Instant::now();
                    if now.duration_since(last_tick) >= tr.tick_duration {
                        tr.advance_tick();
                        last_tick = now;
                    }

                    *sample = tr.mix(1.0 / sample_rate);
                }
            },
            err_fn,
            None,
        )
        .expect("Failed to build output stream")
}

/// ===============================
/// Main
/// ===============================

fn main() -> io::Result<()> {
    let (song, project) = example_song_and_project();
    let num_channels = 3;

    let tracker = Arc::new(Mutex::new(Tracker::new(song, num_channels)));
    let ui_state = Arc::new(Mutex::new(UiState::new(project)));

    // Audio setup
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device available");
    let config = device
        .default_output_config()
        .expect("Failed to get default output config");

    let sample_format = config.sample_format();
    let config: cpal::StreamConfig = config.into();

    let tracker_clone = tracker.clone();
    let stream = match sample_format {
        cpal::SampleFormat::F32 => build_stream_f32(device, config, tracker_clone),
        other => panic!("Unsupported sample format: {:?}", other),
    };

    if let Err(e) = stream.play() {
        eprintln!("Audio error: {:?}", e);
    }

    let mut ui_dirty = true;

    println!("Controls: q=quit, space=play/stop, arrows=move, z/x/c/v/b/n/m=notes, -=clear, `=cmd");

    loop {
        if ui_dirty {
            let tr = tracker.lock().unwrap();
            let ui = ui_state.lock().unwrap();
            draw_ui(&tr.song, &ui, &tr);
            ui_dirty = false;
        }

        if event::poll(Duration::from_millis(16)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if key.kind != KeyEventKind::Press
                {
                    continue;
                }

                if key.code == KeyCode::Char('q') {
                    break;
                }

                let mut tr = tracker.lock().unwrap();
                let mut ui = ui_state.lock().unwrap();

                // sync tracker play state with UI
                let before = ui.is_playing;
                if handle_key(&mut tr.song, &mut ui, key) {
                    ui_dirty = true;
                }
                if ui.is_playing != before {
                    tr.is_playing = ui.is_playing;
                    ui_dirty = true;
                }
            }
        }
    }

    println!("\nExiting tracker.");
    Ok(())
}