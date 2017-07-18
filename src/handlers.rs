use piston::input::Input;

use errors::*;
use events::*;

pub fn input(wevent: &Input) -> Option<Fact> {
    match *wevent {
        Input::Press(_) => unimplemented!(),
        Input::Release(_) => unimplemented!(),
        Input::Move(_) => unimplemented!(),
        Input::Focus(_) => unimplemented!(),
        Input::Text(_) => unimplemented!(),
        Input::Cursor(_) => unimplemented!(),
        _ => error!("I don't know how to handle this event: {:?}", wevent),
    }
    unimplemented!();
}

pub fn change(c: Change) -> Option<Fact> {
    match c {
        Change::Insertion => None,
    }
}
