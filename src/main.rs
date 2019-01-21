use std::collections::HashMap;

use enigo::{Enigo, Key, KeyboardControllable};
use gilrs::{Button, Event, Gilrs};
use gilrs::ev::EventType::ButtonPressed;

fn main() {
    let mut mapping = HashMap::new();

    mapping.insert(Button::DPadRight, vec![Key::Tab]);
    mapping.insert(Button::DPadLeft, vec![Key::Shift, Key::Tab]);
    mapping.insert(Button::DPadDown, vec![Key::Tab]);
    mapping.insert(Button::DPadUp, vec![Key::Shift, Key::Tab]);

    mapping.insert(Button::North, vec![Key::Control, Key::Layout('v')]);
    mapping.insert(Button::East, vec![Key::Escape]);
    mapping.insert(Button::South, vec![Key::Return]);
    mapping.insert(Button::West, vec![Key::Space]);

    mapping.insert(Button::LeftTrigger, vec![Key::UpArrow]);
    mapping.insert(Button::RightTrigger, vec![Key::DownArrow]);

    mapping.insert(Button::RightTrigger2, vec![Key::Backspace]);
    mapping.insert(Button::LeftThumb, vec![Key::Control, Key::Layout('w')]);

    let mut enigo = Enigo::new();
    let mut gilrs = Gilrs::new().unwrap();

    for (_id, gamepad) in gilrs.gamepads() {
        println!("{}", gamepad.os_name());
    }

    loop {
        while let Some(Event { id: _, event, time: _ }) = gilrs.next_event() {
            if let ButtonPressed(button, _code) = event {
                if let Some(key_sequence) = mapping.get(&button) {
                    for &key in key_sequence {
                        enigo.key_down(key);
                    }
                    for &key in key_sequence {
                        enigo.key_up(key);
                    }
                }
            }
        }
    }
}

