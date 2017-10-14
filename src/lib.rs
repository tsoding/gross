extern crate sdl2;
extern crate gl;

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

pub type Point = (f32, f32);

#[derive(Debug)]
pub enum Picture {
    Blank,
    Rectangle(f32, f32, f32, f32),
    Line(f32, f32, f32, f32),
    Polygon(Vec<Point>),
    Circle(f32),
    Text(String),
    // TODO(#14): Design Picture::Bitmap interface and implement support for it
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
    Color(f32, f32, f32, Box<Picture>),
    Translate(f32, f32, Box<Picture>),
    Rotate(f32, Box<Picture>),
    Scale(f32, f32, Box<Picture>),
}

pub type Result<T> = result::Result<T, Box<error::Error>>;

fn render_picture(canvas: &mut Canvas<Window>, picture: &Picture) -> Result<()> {
    match *picture {
        Picture::Rectangle(x, y, width, height) => {
            canvas.fill_rect(rect::Rect::new(x as i32,
                                             y as i32,
                                             width as u32,
                                             height as u32)).map_err(|e| e.into())
        },

        Picture::Line(x1, y1, x2, y2) => {
            canvas.draw_line(rect::Point::new(x1 as i32, y1 as i32),
                             rect::Point::new(x2 as i32, y2 as i32)).map_err(|e| e.into())
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
            canvas.set_draw_color(Color::RGB((r * 255.0) as u8,
                                             (g * 255.0) as u8,
                                             (b * 255.0) as u8));
            let result = render_picture(canvas, boxed_picture.as_ref());
            canvas.set_draw_color(prev_color);
            result
        }

        // TODO(#15): Add Picture::Polygon support
        // TODO(#16): Add Picture::Circle support
        // TODO(#17): Add Picture::Text support
        // TODO(#18): Add Picture::Translate support
        // TODO(#19): Add Picture::Rotate support
        // TODO(#20): Add Picture::Scale support

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

    let context = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
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

        unsafe {
            let (width, height) = window.size();
            gl::ClearColor(1.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Viewport(0, 0, width as i32, height as i32);
        }

        window.gl_swap_window();

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
