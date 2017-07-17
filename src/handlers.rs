use errors::*;
use events::*;
use store::Viewdoc;

use piston::window::Size;
use graphics::*;
use gfx_graphics::{Gfx2d, GfxGraphics};
use gfx::{Resources, CommandBuffer};
use piston::input::Input;

pub fn input(wevent: &Input) -> Fact {
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

pub fn change(c: Change) -> Fact {
    unimplemented!();
}

// a full screen of information laid out using standard units
pub struct Screen {
    dimensions: [usize; 2],
}

impl Screen {
    pub fn new(view: &Viewdoc, size: Size) -> Self {
        Self { dimensions: [size.width as usize, size.height as usize] }
    }

    pub fn render<R: Resources, C: CommandBuffer<R>>(
        &self,
        c: &Context,
        g: &mut GfxGraphics<R, C>,
    ) -> Box<FnOnce(Context, &mut GfxGraphics<R, C>) -> ()> {
        unimplemented!();
    }
}
