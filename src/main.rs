use std::io;
//In which I try and make a better tracker program than OpenMPT
//Good luck

const SAMPLE_BITRATE: i32 = 22 * 1024;
const SAMPLE_LENGTH: i32 = 10 * SAMPLE_BITRATE;
//Wouldn't it be funny if longer samples were just played as shorter samples played after another?
//That would allow for some fun granulization

struct Sample {
    audio: [i8; SAMPLE_LENGTH as usize], //This seems really hacky, I hope it doesn't cause problems
    pitch: i32,
    reverse: bool
}

fn main() -> io::Result<()>
{
    let mut buffer = String::new();
    let stdin = io::stdin();
    //This is a terminal program, I will make a GUI later, if this is too cumbersome at least
    //So, get user input:
    loop {
        print!(">");
        stdin.read_line(&mut buffer)?;
        //Comparing input the lazy way, until I figure out some better interface
        // Add samples
        if buffer == "1"
        {
            say_hi();
        }
        // Edit samples
        if buffer == "2"
        {
            say_hi();
        }
        // Sequence samples
        if buffer == "3"
        {
            say_hi();
        }
        if buffer == "exit"
        {
            say_hi(); //Should we return something to the OS? Who knows.
        }
    }
}

fn say_hi()
{
    println!("hi");
}