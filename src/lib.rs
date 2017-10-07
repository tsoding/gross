extern crate sdl2;

use std::{error, result};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Rect, Point};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub enum Display {
    InWindow((i32, i32), (u32, u32)),
}

pub enum Picture {
    Blank,
    Rectangle(i32, i32, u32, u32),
    Line(i32, i32, i32, i32),
    Pictures(Vec<Picture>)
}

pub type Result<T> = result::Result<T, Box<error::Error>>;

fn render_picture(canvas: &mut Canvas<Window>, picture: Picture) -> Result<()> {
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    match picture {
        Picture::Rectangle(x, y, width, height) => {
            canvas.fill_rect(Rect::new(x, y, width, height))
        },

        Picture::Line(x1, y1, x2, y2) => {
            canvas.draw_line(Point::new(x1, y1),
                             Point::new(x2, y2))
        },

        _ => Ok({})
    }.map_err(|e| e.into())
}

pub fn simulate<S, R, U>(display: Display,
                         init_state: S,
                         render: R,
                         update: U) -> Result<S>
    where R: Fn(&S) -> Picture,
          U: Fn(S) -> S {

    let sdl2_context = sdl2::init()?;
    let video_subsystem = sdl2_context.video()?;

    let window = match display {
        Display::InWindow((x, y), (width, height)) =>
            video_subsystem
            .window("Gross Title", width, height)
            .position(x, y)
            .opengl()
            .build()?
    };

    let mut canvas = window.into_canvas().build()?;
    let mut event_pump = sdl2_context.event_pump()?;

    let mut state = init_state;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        render_picture(&mut canvas, render(&state))?;

        canvas.present();

        state = update(state);
    }

    Ok(state)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
