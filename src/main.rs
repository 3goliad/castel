#![recursion_limit = "1024"]
extern crate clap;
extern crate stderrlog;
extern crate piston;
extern crate graphics;
extern crate gfx;
extern crate gfx_graphics;
extern crate gfx_device_gl;
extern crate sdl2_window;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

mod events;
mod handlers;
mod store;

mod errors {
    use std::fmt;
    use log;

    error_chain!{
        foreign_links {
            SetLogger(log::SetLoggerError);
        }
    }
}
use std::path::Path;

use clap::{Arg, App};
use sdl2_window::{Sdl2Window, OpenGL};
use gfx::traits::*;
use gfx::memory::Typed;
use gfx::format::{DepthStencil, Formatted, Srgba8};
use piston::window::{OpenGLWindow, Window, WindowSettings, Size};
use piston::event_loop::{Events, EventSettings, EventLoop};
use gfx_graphics::{Gfx2d, GlyphCache, TextureSettings};
use graphics::*;

use errors::*;
use store::Store;
use handlers::Screen;

quick_main!(run);

fn run() -> Result<()> {
    let matches = App::new("castel")
        .version("alpha")
        .arg(Arg::with_name("v").short("v").multiple(true))
        .get_matches();

    stderrlog::new()
        .timestamp(stderrlog::Timestamp::Second)
        .verbosity(matches.occurrences_of("v") as usize)
        .init()?;

    let mut data = Store::new();

    let opengl = OpenGL::V3_2;
    let samples = 4;
    let ref mut window: Sdl2Window = WindowSettings::new("castel", [200; 2])
        .opengl(opengl)
        .samples(4)
        .build()?;

    let (mut device, mut factory) = gfx_device_gl::create(|s| {
        window.get_proc_address(s) as *const std::os::raw::c_void
    });

    let mut glyph_cache = GlyphCache::new(
        Path::new("fonts/cmunbmr.ttf"),
        factory.clone(),
        TextureSettings::new(),
    ).expect("This error isn't good enough for error_chain");

    let draw_size = window.draw_size();
    let aa = samples as gfx::texture::NumSamples;
    let dim = (
        draw_size.width as u16,
        draw_size.height as u16,
        1,
        aa.into(),
    );
    let color_format = <Srgba8 as Formatted>::get_format();
    let depth_format = <DepthStencil as Formatted>::get_format();
    let (output_color, output_stencil) =
        gfx_device_gl::create_main_targets_raw(dim, color_format.0, depth_format.0);
    let output_color = Typed::new(output_color);
    let output_stencil = Typed::new(output_stencil);

    let mut encoder = factory.create_command_buffer().into();
    let mut g2d = Gfx2d::new(opengl, &mut factory);
    let mut events = Events::new(EventSettings::new().lazy(true));

    while let Some(e) = events.next(window) {
        use piston::input::Input::*;
        match e {
            Render(args) => {
                let screen = Screen::new(data.view(), window.draw_size());
                g2d.draw(
                    &mut encoder,
                    &output_color,
                    &output_stencil,
                    args.viewport(),
                    |c, g| {
                        clear([1.0; 4], g);
                        screen.render(&c, g);
                        text::Text::new_color([0.0, 0.5, 0.0, 1.0], 32).draw(
                            "Hello gfx_graphics!",
                            &mut glyph_cache,
                            &DrawState::default(),
                            c.transform.trans(
                                10.0,
                                100.0,
                            ),
                            g,
                        );
                    },
                );
                encoder.flush(&mut device);
            }
            AfterRender(_) => {
                device.cleanup();
            }
            Close(_) => {
                break;
            }
            _ => {
                data.apply(handlers::input(&e));
                while let Some(new_change) = data.changes.next() {
                    data.apply(handlers::change(new_change));
                }
            }

        }
    }
    Ok(())
}
