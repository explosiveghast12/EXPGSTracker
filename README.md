# EXPGSTracker

A silly name for silly software
Planned features are a minimalistic way to create sample based music, plugin support (VST/VST3/CLAP) may be added later.
I have no idea what the timeline is for this project, I would like to say that it will only take a year, but we will see.
I don't have a plan for any mixer/fx channels right now, but that could change.

## Instructions for Build and Use

Steps to build and/or run the software:

1. Make sure rust is installed
2. rustc main.rs or cargo build

Or:

1. Download executable (not uploaded yet, expect alpha 0.1 by 11/6/2025)
2. Launch executable

Instructions for using the software (Currently only secondary):

1. Use arrow keys to navigate
2. Space to play/pause
3. Bottom key row (z-m) to enter notes
4. 0 to enter stop note
5. f1 to access sequencer
6. f2 to access side panel
7. enter to enter command mode (not implemented fully)

## Development Environment 

To recreate the development environment, you need the following software and/or libraries with the specified versions:

* Install VSCode
* Install rust
* Install RustAnalyzer in VSCode
* Import standard crate

## Useful Websites to Learn More

I found these websites useful in developing this software:

* [Rust Programming Language Website](rust-lang.org)
* [Rust Audio Crate](https://docs.rs/audio/latest/audio/)

## Future Work

The following items I plan to fix, improve, and/or add to this project in the future:

* [x] Add basic features
* [ ] Create macro for user selections to make code easier to follow
* [x] Add input outside of reading from terminal (used crossterm)
* [ ] Update stable main code to have working features from secondary tracker
* [ ] Add MIDI support
* [ ] Add instrument selecter
