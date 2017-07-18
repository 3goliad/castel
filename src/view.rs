use piston::window::Size;
use graphics::*;
use gfx_graphics::GfxGraphics;
use gfx::{Resources, CommandBuffer};

use errors::*;

// a full screen of information laid out using standard units
pub struct Screen {
    dimensions: [usize; 2],
}

impl Screen {
    pub fn new(size: Size) -> Self {
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
