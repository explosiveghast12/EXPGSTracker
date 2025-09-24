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

// Reuirements
// Variables: mutable and unmutable - done (unmutable by default)
// loops - done
// functions: transfer ownership or borrow refernce - done
// vec data structure - done
// match keyword - apparently like switch, I don't have a use for this, so figure something out.

const SAMPLE_BITRATE: i32 = 22 * 1024;
const SAMPLE_LENGTH: i32 = 10 * SAMPLE_BITRATE;
//Wouldn't it be funny if longer samples were just played as shorter samples played after another?
//That would allow for some fun granulization


//Comments from previous thing, remove
//Global variables
//Apparently these would be considered unsafe if mutable, so you are supposed to wrap them in synchronization.
// So where should I put this? I could make a class, but I want to do functional programming
// I guess I should put this in the main function, and then pass it into my other functions

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