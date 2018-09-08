#![no_std]
#![feature(alloc)]

extern crate keycodes_ascii;
extern crate alloc;

use keycodes_ascii::{KeyEvent};
use alloc::string::String;


#[derive(Debug, Clone)]
pub enum Event {
    InputEvent(KeyboardInputEvent),
    OutputEvent(PrintOutputEvent),
    ExitEvent,
}

impl Event {
    pub fn new_input_event(kev: KeyEvent) -> Event {
        Event::InputEvent(KeyboardInputEvent::new(kev))
    }

    pub fn new_output_event<S>(s: S, display: bool) -> Event where S: Into<String> {
        Event::OutputEvent(PrintOutputEvent::new(s.into(), display))
    }
}

/// use this to deliver input events (such as keyboard input) to the input_event_manager.
#[derive(Debug, Clone)]
pub struct KeyboardInputEvent {
    pub key_event: KeyEvent,
}

impl KeyboardInputEvent {
    pub fn new(kev: KeyEvent) -> KeyboardInputEvent {
        KeyboardInputEvent {
            key_event: kev,
        }
    }
}



/// use this to queue up a formatted string that should be printed to the input_event_manager. 
#[derive(Debug, Clone)]
pub struct PrintOutputEvent {
    pub text: String,
    // indicates whether or not the terminal/application should refresh its TextDisplay when it handles this output event
    pub display: bool,
}

impl PrintOutputEvent {
    pub fn new(s: String, display: bool) -> PrintOutputEvent {
        PrintOutputEvent {
            text: s,
            display: display
        }
    }
}