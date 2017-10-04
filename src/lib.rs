extern crate sdl2;

use std::{error, result};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

pub enum Display {
    InWindow((i32, i32), (u32, u32)),
}

pub enum Picture {
    Blank,
    Rectangle(i32, i32, u32, u32)
}

pub type Result<T> = result::Result<T, Box<error::Error>>;

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

        let picture = render(&state);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        match picture {
            Picture::Rectangle(x, y, width, height) => {
                canvas.set_draw_color(Color::RGB(255, 0, 0));
                canvas.fill_rect(Rect::new(x, y, width, height))?
            },
            _ => {},
        }

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
