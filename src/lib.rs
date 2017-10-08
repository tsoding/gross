extern crate sdl2;

use std::{error, result};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

pub enum Display {
    InWindow((i32, i32), (u32, u32)),
}

pub type Point = (i32, i32);

// TODO(#13): use floats iso ints for Picture coordinates and sizes
#[derive(Debug)]
pub enum Picture {
    Blank,
    Rectangle(i32, i32, u32, u32),
    Line(i32, i32, i32, i32),
    Polygon(Vec<Point>),
    Circle(u32),
    Text(String),
    // TODO: Design Picture::Bitmap interface and implement support for it
    //
    // It would be good to have support for some bitmaps, but at the
    // moment I don't know how the signature of Picture::Bitmap should
    // look like.
    //
    // Gloss implements some kind of [BitmapData][BitmapData]. Maybe
    // we should implement something similar
    //
    // [BitmapData]: https://hackage.haskell.org/package/gloss-1.11.1.1/docs/Graphics-Gloss-Data-Bitmap.html#t:BitmapData
    Bitmap,

    Pictures(Vec<Picture>),
    Color(u8, u8, u8, Box<Picture>),
    Translate(i32, i32, Box<Picture>),
    Rotate(i32, Box<Picture>),
    Scale(i32, i32, Box<Picture>),
}

pub type Result<T> = result::Result<T, Box<error::Error>>;

fn render_picture(canvas: &mut Canvas<Window>, picture: &Picture) -> Result<()> {
    match *picture {
        Picture::Rectangle(x, y, width, height) => {
            canvas.fill_rect(rect::Rect::new(x, y, width, height)).map_err(|e| e.into())
        },

        Picture::Line(x1, y1, x2, y2) => {
            canvas.draw_line(rect::Point::new(x1, y1),
                             rect::Point::new(x2, y2)).map_err(|e| e.into())
        },

        Picture::Pictures(ref pictures) => {
            pictures
                .iter()
                .map(|picture| render_picture(canvas, picture))
                .collect::<Result<Vec<_>>>()
                .map(|_| ())
        },

        // TODO(#12): Rethink the Color combinator implementation
        //
        // I think in Gloss it behaves a little bit different. We need to research that.
        Picture::Color(r, g, b, ref boxed_picture) => {
            let prev_color = canvas.draw_color();
            canvas.set_draw_color(Color::RGB(r, g, b));
            let result = render_picture(canvas, boxed_picture.as_ref());
            canvas.set_draw_color(prev_color);
            result
        }

        // TODO: Add Picture::Polygon support
        // TODO: Add Picture::Circle support
        // TODO: Add Picture::Text support
        // TODO: Add Picture::Translate support
        // TODO: Add Picture::Rotate support
        // TODO: Add Picture::Scale support

        Picture::Blank => Ok({}),

        _ => panic!("Unsupported Picture element {:?}", picture)
    }
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

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        render_picture(&mut canvas, &render(&state))?;

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
