use std::{collections::btree_map::Range, io, process::Stdio, time::Duration, io};
use std::env; //You know, we could combine these with the line above
use std::fs;
use rand::{rng, Rng, RngCore};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    FromSample, Sample, SizedSample, I24,
};
// Now with more rng!!!

use audio;
use crossterm::{
    event::{
        poll, read, DisableBracketedPaste, DisableFocusChange, DisableMouseCapture,
        EnableBracketedPaste, EnableFocusChange, EnableMouseCapture, Event,
    },
    execute,
};
use tokio::time::{sleep, Duration};
// Allow focus to affect things, or not, I don't know what's better

//In which I try and make a better tracker program than OpenMPT
//Good luck
// I have been working on this for ~30min 2:00pm 9/23/2025, add to time card
// Have a current sample selected, presets, and other fancy stuff
// Another 30 minutes at 5:00pm
// I have found an audio crate: https://docs.rs/audio/latest/audio/ which should help keep my sanity.
// Add variables for tracker and initialize them, this includes audio buffer and default values
// Allow editing defaults later
// Add demo scene at beggining if bored
// 30 minutes 9:15 9/24/2025
// 30 mintues 11:30PM 9/24/2025
// 1 hour 9/26/2025 12:00 PM
// 30 minutes 9/26/2025
// 30 minutes 10/1/2025 2:00pm
// 1hr 10/3/2025 5:30pm
// https://crates.io/crates/crossterm
// NOTE: When using audio you have to access whatever audio device that person's computer is using
// Here is a cross platform library: https://github.com/RustAudio/cpal, but that uses WASAPI, not ASIO
// Whatever, figure it out
// (please don't make people use ASIO4All)
// But ASIO is kind of unneccesary for this software
// Unless you care about having multiple audio in/outs

// Reuirements
// Variables: mutable and unmutable - done (unmutable by default)
// loops - done
// functions: transfer ownership or borrow refernce - done
// vec data structure - done
// match keyword - pretty much done, just need to put the code in place

const SAMPLE_BITRATE: i32 = 22 * 1024;
const SAMPLE_LENGTH: i32 = 10 * SAMPLE_BITRATE;
const VERSION: i32 = 1; // This does limit us to 2^32 versions we can release, so sad
const MODE_MENU: i8 = 0;
const MODE_SEQUENCER: i8 = 1;
const MODE_TRACKER: i8 = 2;
const MODE_SAMPLE_EDIT: i8 = 3;
//Wouldn't it be funny if longer samples were just played as shorter samples played after another?
//That would allow for some fun granulization


//Comments from previous thing, remove
//Global variables
//Apparently these would be considered unsafe if mutable, so you are supposed to wrap them in synchronization.
// So where should I put this? I could make a class, but I want to do functional programming
// I guess I should put this in the main function, and then pass it into my other functions

//static mut PROJECT_NAME: String = "default";
//static mut AUTHOR: String = "anon";
//static SPLASH: String = "The best music program";

struct AudioSample { // Could just have sample made of audio buffers, but that sound like it would cause performance issues
    audio: Vec<f32>, // I have decided to use a vector to store audio data
    pitch: i8,
    reverse: bool,
    sample_name: String,
    sample_tempo: f32
}

struct Note
{
    pitch: i8, // We only need 255 notes, unless we start doing weird stuff
    sample: i8, // Reference sample number, currently only 255
    channel: i8
}

struct Step
{
    notes: Vec<Note>
}

struct Track
{
    track: Vec<Step>
}

struct Pattern
{
    pattern: Vec<Track> // Arrays may be better, but then it's dependent on what's decided on compile time, I want to be able to change that
    // Should be length 8 right now
}

struct SeqCursor // Do we need a cursor for only sequencing?
{
    position: (i32, i32)
    //row: i32, // Row, # columns varies based on row
    // You can reset column number each time, but why would you?
    // Or track based on character, then change character based on row.
    // That sounds better
    // column: i32 // What maniac has 2^32 rows on their terminal?
    // Of course, this isn't necessary to optimize since I think 64 bit architecture doesn't have an 8 bit word size anyway
    // Then snap to nearest position on row change
}
// Implementation for cursor?
impl SeqCursor
{
    fn new() -> SeqCursor
    {
        SeqCursor
        {
            position: (0, 0)
        }
    }

    fn cursor_up(&mut self)
    {
        // Where are we now, where are we going?
        // Do we even have the help screen?
        // The command function doesn't need to use the same display function, just write a newline
        if self.position.0 > 0
        {
            self.position.0 -= 1;
        }
    }

    fn cursor_down(&mut self)
    {
        if self.position.0 < 42 // How low can the cursor go?
        {
            self.position.0 += 1;
        }
    }

    fn cursor_left(&mut self)
    {
        if self.position.1 > 0
        {
            self.position.1 -= 1;
        }
    }

    fn cursor_right(&mut self)
    {
        if self.position.1 > 0
        {
            self.position.1 += 42; // How far right can I go? Should make some constants or something
        } // A constant called screen size, x*y
    }

    fn cursor_row(&self) -> i32
    {
        self.position.0
    }

    fn cursor_column(&self) -> i32
    {
        self.position.1
    }
}

struct Sequence
{
    sequence: Vec<Pattern>
}

impl Sequence
{
    fn new() -> Sequence
    {
        Sequence { sequence: Vec::new() }
    }
    
    fn get_pattern_number(&self, location: i32) -> i32
    {
        location //Not implemented
    }

    fn length(&self) -> i32
    {
        self.sequence.len() as i32 // No implicit conversions, so sad
    }
}

struct Globe // All the data we need passed between functions, only should borrow this (pass the reference)
{
    seq: Sequence,
    project_name: String,
    author: String,
    samples: Vec<AudioSample>,
    splash_text: String,
    title: String,
    cursor: SeqCursor,
    track_begin: i32,
    track_end: i32,
    mode: i8,
    // audio_buffer: audio::buf::Interleaved
}

impl Globe
{
    fn new() -> Globe
    {
        Globe
        {
        // Do I need to define vectors as empty??
            project_name: String::from("0w0"), // Defaults are here for no reason
            author: String::from("Furro"),
            seq: Sequence::new(),
            samples: Vec::new(),
            splash_text: get_quote(),
            title: get_program_name(),
            cursor: SeqCursor::new(),
            track_begin: 0,
            track_end: 16,
            mode: MODE_MENU,
            // audio_buffer: buf = audio::buf::Interleaved::<f32>::new()
        }
    }
    fn move_down(&mut self)
    {
        if self.track_begin > 0
        {
            self.track_begin -= 1;
            self.track_end -= 1;
        }
    }

    fn move_up(&mut self)
    {
        if self.track_end < self.seq.length()
        {
            self.track_begin += 1;
            self.track_end += 1;
        }
    }

    fn track_begin(&self) -> i32
    {
        self.track_begin
    }

    fn track_end(&self) -> i32
    {
        self.track_end
    }
    // implement getters
    fn name(&self) -> &String
    {
        &self.project_name
    }

    fn author(&self) -> &String
    {
        &self.author
    }

    fn splash(&self) -> &String
    {
        &self.splash_text
    }

    fn title(&self) -> &String
    {
        &self.title
    }    

    fn cursor_row(&self) -> i32
    {
        self.cursor.cursor_row()
    }

    fn cursor_column(&self) -> i32
    {
        self.cursor.cursor_column()
    }
}

// Figure out vectors
// Read this: https://doc.rust-lang.org/std/time/index.html

fn main() -> io::Result<()> {
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let config = device.default_output_config().unwrap();
    println!("Default output config: {config:?}");
    run::<f32>(&device, &config.into());
    // This code should be moved into a structure to keep main nice.
    // Play (probably very loud) noise
    // Technically just fills buffers with random noise, but I figure that the buffers are being played?
    startup_noises();
    // Initialize global data structure
    let firmament = Globe::new();
    // Define variables that we will need throughout the program
    let sequence: Vec<Step> = Vec::new();
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    loop {
        stdin.read_line(&mut buffer)?; // Oh no, how can we clean the input?
        // I was trying to use pop, but that didn't work, using trim does
        // We could likely trim once if we cared, but this program is lightweight alrady, I'll fix it later.
        if buffer.trim() == "quit" // Okay, I have dereferenced the buffer since I wan't to compare the data, not the pointer.
        {
            break;
        }
        else if buffer.trim() == "help" // Do I really need to trim the buffer every time
        {
            help();
        }
        else if buffer.trim() == "load"
        {
            load();
        }
        else if buffer.trim() == "sequence"
        {
            sequence_loop(&firmament, &stdin);
        }
        else if buffer.trim() == "samples"
        {
            edit_samples(&stdin);
        }
        else if buffer.trim() == "hi"
        {
            say_hi();
        }
        else if buffer.trim() == "mix"
        {
            println!("Did you really think I've implemented mixing yet?");
        }
        else if buffer.trim() == "master"
        {
            master_edit();
        }
        else if buffer.trim() == "export"
        {
            // Export project as audio or whatever.
        }
        // Should command be from a global place? Probably not since context matters for commands.
        // That could create a bad UX though...
        // println!("{}", buffer); // This was here for debugging
        buffer.clear();
    }
    Ok(()) // This is just a return for the operating system saying that we exited successfully (it returns nothing)
}

fn help()
{
    println!("1. quit\n2. load\n3. sequence\n4. samples");
    println!("If you need help with bugs/bad UI try alt-f4...");
}

fn say_hi()
{
    println!("hi");
}

fn sequence_loop(firm: &Globe, stdin: &io::Stdin)
{
    let mut buffer = String::new();
    // Add/record notes into vector
    // Allow playback
    // I could make a macro that generates these loops, I should. Maybe one day.
    // commands:
    // play, stop (can we use f1 and f2?)
    // display an ascii tracker and use arrow keys to move
    // Extra (and easier for me) algorithmic input, ` to access commands then type in Notes, length, scale, pattern, etc. Use overloading if it exists in this language.`
    //loop {
        //if buffer.trim() == "back"
        //{
        //    stdin.read_line(&mut buffer);
        //    break;
        //}
    //}
    display_sequencer_screen(firm);
}

// Display should look like this for sequencer:

/*
(Help bar, off by default, not implemented)
ProjectName - Author -                                  EXPGSTracker
[1][2][3][4][5][6][7][8][9][8][7][6][5][4][3][2]...[1] |Pattern (1)
--------------------------------------------------------------------
TRACK1|TRACK2|TRACK3|TRACK4|TRACK5|TRACK6|TRACK7|TRACK8|Sample:
......|......|......|......|......|......|......|......|(samplename)
......|......|......|......|......|......|......|......|Autostep:
......|......|......|......|......|......|......|......|(number)
......|......|......|......|......|......|......|......|PPQ:
......|......|......|......|......|......|......|......|(PPQ)
......|......|......|......|......|......|......|......|BPM:
......|......|......|......|......|......|......|......|(BPM)
......|......|......|......|......|......|......|......|Scale/Mode:
......|......|......|......|......|......|......|......|(Not implemented)
......|......|......|......|......|......|......|......|
\/\/\/ (pgdown/pgup)
(Information on cursor selection)
CMD:_

So we have a header at top containing {project_name} {author} and "EXPGSTracker"
After that we have the pattern sequencer, in this example filled longer than display
which produces ... which shows there are hidden patterns in the sequence
The pattern number is to the right of this (pattern you are currently editing)
With all of these you should ideally be able to cursor to the thing you want to edit
Then type in a new value, but we also should allow shortcuts and commands to edit things.
Then we display tracks with the {TrackName} (6 characters)
Below that we have {Data} (6 characters, ...... means empty)
Arrows indicating if further tracks are not displayed on screens
Arrows up would appear above if scrolled down
To the right we have information about current selections
Sample:
{Currently Selected Sample}
Autostep:
{Autostep length}
PPQ:
{PulsePerQuarter}
BPM:
{BPM}
CMD appears when ` is pressed and can be cancelled with esc
This will be implemented with a cartesian co-ordinate system
so that the program knows where the cursor is and highlights that area
row 0 is title information and such
row 1 is sequencer which is stored as a vector
row 2-{pattern length} is tracker
column 9 (or 1+{track#}) is tracked as a seperate array
So we have:
-Array with project metadata
-Vector with sequencer information
-variable with current track
-Array with pattern data
-Array with project data
-We just have to keep track of when to swap cursor between these things
-Use integer to keep track of which one is selected, and if moved off (right, down, left, up)
-Then we automatically switch that variable.

How information is displayed in boxes:
A#1C01
^  ^
|  Channel
Note
And:
123C01
^  ^
|  Channel
Signal (for control signals like pitch change/whatever)

formatting info: https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences
\033[7 (selected), if you want to you can also use color and bold and other fancy things.
*/

fn display_sequencer_screen(firm: &Globe) // The display needs to know
{
    // If help, display help
    print!("{}[2J", 27 as char); //clears screen, from https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed
    println!("{} - {} - \'{}\' ^w^ {}", firm.name(), firm.author(), firm.splash(), firm.title()); //Global variables would be nice, so would classes, but I'm stubborn, let's use some global variables
    println!("{} |Pattern: {}", sequence_to_string(&firm.seq), 1); // Do I need ()? I don't think so
    println!("--------------------------------------------------------------------");
    println!("TRACK1|TRACK2|TRACK3|TRACK4|TRACK5|TRACK6|TRACK7|TRACK8|Sample:");
    // if not at start then display ^^^, don't let cursor move here? Just move up (How to access project metadata then?)
    // support pgup and pgdown (Jump x tracks down)
    if firm.track_begin() != 0
    {
        println!("^^^^");
    }
    for i in firm.track_begin()..firm.track_end()
    {
        for y in 0..8 // could go 0 to track number, but that may get too wide
        {
            // Track 1, 2, 3, 4, 5, 6, 7, 8
            print!("{}", get_data(y, i)); // This will change
        }
        // print sample, autostep, ppq, etc. on appropriate lines
        // 
        println!("{}", get_text_after(i));
    }
    // for loop up until size of terminal, do we have a terminal size?
    // if more lines to display then print \/\/\/
    // Don't allow cursor to move here, just shift down
    if firm.track_end() != firm.seq.length() // -1?
    {
        println!("\\/\\/\\/") // how you print "\/\/\/"
    }
}

fn get_text_after(row: i32) -> String
{
    return match row
    {
        0 => "Sample".to_string(),
        1 => "(sample_name)".to_string(),
        2 => "Autostep".to_string(),
        3 => "(step_size)".to_string(),
        4 => "PPQ".to_string(),
        5 => "(ppq)".to_string(),
        6 => "BPM".to_string(),
        7 => "(tempo)".to_string(),
        8 => "Scale/mode".to_string(),
        9 => "(scale)".to_string(),
        _ => "".to_string()
    };
}

fn get_data(track: i32, column: i32) -> String
{
    return "......|".to_string();
}

fn load()
{
    // Load wavy files into memory
    // Or projects depending on file extension
}

fn edit_samples(stdin: &io::Stdin)
{
    // We are not tokenizing, not yet at least. This should make the software easier to use, but also slower to use.
    // Maybe just run tokenized command if more than one word is given.
    let mut buffer = String::new();
    loop {
        stdin.read_line(&mut buffer);
        if buffer.trim() == "quit"
        {
            break;
        }
        else if buffer.trim() == "truncate" // Wrong, we need to check arguments
        {
            // Run truncate command
        }
        else if buffer.trim() == "repitch"
        {
            // run repitch command
        }
        else if buffer.trim() == "time-stretch" // This one may be hard to spell for people
        {
            // Run time-stretch
        }
        else if buffer.trim() == "param-eq"
        {
            // Run EQ
        }
        buffer.clear();
    }
    // Do digital signal processing to the samples loaded into memory
    // Make an undo function if you care about usability

}

/*
https://doc.rust-lang.org/rust-by-example/flow_control/match.html
Code to match 8 bit integer to corresponding MIDI note

midi 0 = A
Repeats every 12 notes
A, A#, B, C, C#, D, D#, E, F, F#, G, G#
So modulo 12 to find octave then use the remainder with this match
(so we can store notes as numbers, but display as notes)
string = match pitch
{
    0 => "A",
    1 => "A#",
    2 => "B",
    3 => "C",
    4 => "C#",
    5 => "D",
    6 => "D#",
    7 => "E",
    8 => "F",
    9 => "F#",
    10 => "G",
    11 => "G#",
};

Would be smart to have another version that displays as flat
*/

fn get_quote() -> String // Returns random quote
{
    return "non-human userbase".to_string();
}

fn get_program_name() -> String // Returns some variation of EXPGSTracker
{
    return "EZGQRTracker".to_string();
}

fn sequence_to_string(seq: &Sequence) -> String
{
    let mut sequence_displayed: String = String::new();
    let mut temp_string: String = String::new();
    let length_seq: i32 = seq.length();
    for i in 0..16 // So add another variable which is shift, only allow shift if greater than whatever, that would be cursor code
    {
        if i < length_seq
        {
            temp_string.push_str("[");
            temp_string.push_str(&seq.get_pattern_number(i).to_string());
            temp_string.push_str("]");
            // Append pattern number like [1] and blank if not in range
            // Also need to know where we are viewing the string from
            sequence_displayed.push_str(&temp_string);
            temp_string.clear();
        }
        else {
            sequence_displayed.push_str("[.]"); // THREE things, this is for visual clarity
        }
    }
    // 17
    if length_seq < 17 // Most likely case, hopefully
    {
        sequence_displayed.push_str("      "); // SIX spaces
    }
    else if length_seq == 17 // A special case
    {
        sequence_displayed.push_str(&temp_string);
    }
    else {
        sequence_displayed.push_str("   ");
    }
    // Add a ... if there are more than 18, otherwise display last two normally
    // 18
    return sequence_displayed;
}

fn command(stdin: io::Stdin)
{
    let mut buffer: String = String::new();
    print!("CMD: ");
    stdin.read_line(&mut buffer); // Do we need a ?
    // tokenize the command
    // let command: Vec<String> buffer.split(" "); // This command is wrong
    println!("Commands not implemented");
}

fn check_input()
{
    // IDK, look here: https://github.com/crossterm-rs/crossterm/blob/master/examples/key-display.rs
}

fn get_note_as_string()
{
    // Return note/octave or hex code
}

fn useless(variable: i32)
{
    println!("{}", variable);
}

fn master_edit()
{
    // Choose between brickwall, clear, etc...
}

fn load_file()
{
    // Check file extension so we know what file we are working with
    // Implement later
}

fn load_wav()
{
    // Load *.wav file as data (32 bit float of course)
}

// At this point I wish I knew how to seperate functions into seperate files

fn startup_noises()
{
    // Filepath to startup noises
    // Play that wavy file
    // BTW, WAV files are a subtype of the RIFF file format or whatever,
    // Which means they have a dumb header,
    // It really isn't important, just skip it.
    // Though you do want to know the bit depth

    // Until we figure out reading files, we can generate NOISE, so fun
    // It would be fun (easy) to make a stupid synth engine instead of using samples
    // But that would be worse software
    let mut buf = audio::buf::Dynamic::<f32>::new(); // Woah, we make an audio buffer

    buf.resize_channels(2);
    buf.resize_frames(2048);

    // Just so you know, this is example code for the audio crate.

    let mut rng= rand::rng();
    rng.fill(&mut buf[0]);
    rng.fill(&mut buf[1]);

    // The more I look at this audio crate, the more I realize the README is really old
    // Can I update this?
}

// From example code

pub fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
where
    T: SizedSample + FromSample<f32>,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // Produce a sinusoid of maximum amplitude.
    let mut sample_clock = 0f32;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {err}");

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_value)
        },
        err_fn,
        None,
    )?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: Sample + FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let value: T = T::from_sample(next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}

// End example code

async fn input_handler() -> io::Result<()>
{
    // Nevermind, I'll just read documentation
    // Based on example code from crossterm
    execute!(
         EnableFocusChange,
         EnableMouseCapture
    )?;

    loop {
        // `poll()` waits for an `Event` for a given time period
        if poll(Duration::from_millis(500))? {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match read()? {
                Event::FocusGained => println!("FocusGained"),
                Event::FocusLost => println!("FocusLost"),
                Event::Key(event) => println!("{:?}", event),
                Event::Mouse(event) => println!("{:?}", event),
                Event::Resize(width, height) => println!("New size {}x{}", width, height), // Make display work with resize
            }
        } else {
            // Timeout expired and no `Event` is available
        }
    }

    execute!(
         DisableFocusChange,
         DisableMouseCapture
    )?;

    Ok(())
}

fn connect_key_event_to_function(event : Eventually, global: &mut Globe) // Why &mut? I just need to borrow it
{
    // This is for the entire thing
    if Globe.mode == MODE_MENU
    {
        match event
        {
            Event::Key(event) => // I hate this code, but it allows me to match Key events and Mouse events in the same function
            {
                match event.code
                {
                    KeyCode::Up => global.move_up(),
                    KeyCode::Down => global.move_down(),
                    _ => {}
                }
            }
            Event::Mouse(event) =>
            {
                match event.kind
                {
                    MouseEventKind::ScrollUp => global.move_up(),
                    MouseEventKind::ScrollDown => global.move_down(),
                    _ => {}
                }
            }
            _ => {}
        }
    }
    else if Globe.mode == MODE_SEQUENCER
    {
        match event
        {
            Event::Key(event) =>
            {
                match event.code
                {
                    KeyCode::Up => global.cursor.cursor_up(),
                    KeyCode::Down => global.cursor.cursor_down(),
                    KeyCode::Left => global.cursor.cursor_left(),
                    KeyCode::Right => global.cursor.cursor_right(),
                    _ => {}
                }
            }
            event::Mouse(event) =>
            {
                match event.kind
                {
                    MouseEventKind::ScrollUp => global.move_up(),
                    MouseEventKind::ScrollDown => global.move_down(),
                    _ => {}
                }
            }
            _ => {}
        }
    }
    else if Globe.mode == MODE_TRACKER
    {
        // Similar to sequencer, but different functions
    }
    else if Globe.mode == MODE_SAMPLE_EDIT
    {
        // Sample editing functions
    }
}