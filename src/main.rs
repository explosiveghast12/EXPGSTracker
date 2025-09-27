use std::{collections::btree_map::Range, io};
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
// https://crates.io/crates/crossterm
//

// Reuirements
// Variables: mutable and unmutable - done (unmutable by default)
// loops - done
// functions: transfer ownership or borrow refernce - done
// vec data structure - done
// match keyword - pretty much done, just need to put the code in place

const SAMPLE_BITRATE: i32 = 22 * 1024;
const SAMPLE_LENGTH: i32 = 10 * SAMPLE_BITRATE;
const VERSION: i32 = 1; // This does limit us to 2^32 versions we can release, so sad
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
    row: i32, // Row, # columns varies based on row
    // You can reset column number each time, but why would you?
    // Or track based on character, then change character based on row.
    // That sounds better
    column: i32 // What maniac has 2^32 rows on their terminal?
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
            row: 0,
            column: 0
        }
    }

    fn cursor_up()
    {
        // Where are we now, where are we going?
        // Do we even have the help screen?
        // The command function doesn't need to use the same display function, just write a newline
    }

    fn cursor_down()
    {

    }

    fn cursor_left()
    {

    }

    fn cursor_right()
    {

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
    samples: Vec<Sample>,
    splash_text: String,
    title: String
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
            title: get_program_name()
        }
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
}

// Figure out vectors
// Read this: https://doc.rust-lang.org/std/time/index.html

fn main() -> io::Result<()> {
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
            sequence_loop(&firmament, &stdin);
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
    for i in 0..16
    {
        for _y in 0..8
        {
            // Track 1, 2, 3, 4, 5, 6, 7, 8
            print!("......|"); // This will change
        }
        // print sample, autostep, ppq, etc. on appropriate lines
        println!();
    }
    // for loop up until size of terminal, do we have a terminal size?
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
            sequence_displayed.push_str("   "); // THREE spaces
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