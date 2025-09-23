use std::io;
//In which I try and make a better tracker program than OpenMPT
//Good luck
// I have been working on this for ~30min 2:00pm 9/23/2025, add to time card

const SAMPLE_BITRATE: i32 = 22 * 1024;
const SAMPLE_LENGTH: i32 = 10 * SAMPLE_BITRATE;
//Wouldn't it be funny if longer samples were just played as shorter samples played after another?
//That would allow for some fun granulization

struct Sample {
    audio: [i8; SAMPLE_LENGTH as usize], //This seems really hacky, I hope it doesn't cause problems
    pitch: i8,
    reverse: bool
}

struct Note
{
    pitch: i8, // We only need 255 notes, unless we start doing weird stuff
    sample: Sample
}

// Figure out vectors
// Read this: https://doc.rust-lang.org/std/time/index.html

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    loop {
        stdin.read_line(&mut buffer)?; // Oh no, how can we clean the input?
        // I was trying to use pop, but that didn't work, using trim does
        if buffer.trim() == "quit" // Okay, I have dereferenced the buffer since I wan't to compare the data, not the pointer.
        {
            break;
        }
        println!("{}", buffer);
        buffer.clear();
    }
    Ok(()) // This is a weird thing, I don't really understand it. LOOK IT UP.
}

fn say_hi()
{
    println!("hi");
}

fn sequence_loop()
{
    // Add/record notes into vector
    // Allow playback
}

fn load_sample()
{
    // Load wavy files into memory
}

fn edit_samples()
{
    // Do digital signal processing to the samples loaded into memory
    // Make an undo function if you care about usability
}