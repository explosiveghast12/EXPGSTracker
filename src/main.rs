use std::io;
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

// Reuirements
// Variables: mutable and unmutable - done (unmutable by default)
// loops - done
// functions: transfer ownership or borrow refernce - done
// vec data structure - done
// match keyword - pretty much done, just need to put the code in place

const SAMPLE_BITRATE: i32 = 22 * 1024;
const SAMPLE_LENGTH: i32 = 10 * SAMPLE_BITRATE;
//Wouldn't it be funny if longer samples were just played as shorter samples played after another?
//That would allow for some fun granulization


//Comments from previous thing, remove
//Global variables
//Apparently these would be considered unsafe if mutable, so you are supposed to wrap them in synchronization.
// So where should I put this? I could make a class, but I want to do functional programming
// I guess I should put this in the main function, and then pass it into my other functions

static mut PROJECT_NAME: String = "default";
static mut AUTHOR: String = "anon";
static SPLASH: String = "The best music program";

struct Sample {
    audio: [i8; SAMPLE_LENGTH as usize], //This seems really hacky, I hope it doesn't cause problems
    pitch: i8,
    reverse: bool
}

struct Note
{
    pitch: i8, // We only need 255 notes, unless we start doing weird stuff
    sample: Sample,
    channel: i8
}

struct Step
{
    notes: Vec<Note>
}

// Figure out vectors
// Read this: https://doc.rust-lang.org/std/time/index.html

fn main() -> io::Result<()> {
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
        else if buffer.trim() == "help"
        {
            help();
        }
        else if buffer.trim() == "load"
        {
            load();
        }
        else if buffer.trim() == "sequence"
        {
            sequence_loop(&sequence);
        }
        else if buffer.trim() == "samples"
        {
            edit_samples();
        }
        println!("{}", buffer);
        buffer.clear();
    }
    Ok(()) // This is a weird thing, I don't really understand it. LOOK IT UP.
}

fn help()
{
    println!("1. quit\n2. load\n3. sequence\n4. samples");
}

fn say_hi()
{
    println!("hi");
}

fn sequence_loop(sequence: &Vec<Step>)
{
    // Add/record notes into vector
    // Allow playback
    // I could make a macro that generates these loops, I should. Maybe one day.
    // commands:
    // play, stop (can we use f1 and f2?)
    // display an ascii tracker and use arrow keys to move
    // Extra (and easier for me) algorithmic input, ` to access commands then type in Notes, length, scale, pattern, etc. Use overloading if it exists in this language.`
    loop {
        if buffer.trim() == "back"
        {
            stdin.read_line(&mut buffer)?; // Okay, so this probably won't work since buffer isn't defined here, two options, 1: make the buffer global, 2: reinitialize buffer
            break;
        }
    }
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

fn display_sequencer_screen()
{
    // If help, display help
    print!("\e[2J"); //Erase screen, figure out what escape sequences rust supports
    println!("{} - {} - \'{}\' ^w^ {}", PROJECT_NAME, ); //Global variables would be nice, so would classes, but I'm stubborn, let's use some global variables
    println!();
}

fn load()
{
    // Load wavy files into memory
    // Or projects depending on file extension
}

fn edit_samples()
{
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