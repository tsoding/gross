extern crate sdl2;
extern crate gl;

mod gll;
mod result;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use result::Result;
use gll::*;

pub enum Display {
    InWindow((i32, i32), (u32, u32)),
}

pub type Point = (f32, f32);

#[derive(Debug)]
pub enum Picture {
    Blank,
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

fn render_picture(picture: &Picture, points_vbo: u32, color_loc: i32) -> Result<()> {
    match *picture {
        Picture::Polygon(ref points) => {
            let data: Vec<f32> = points
                .iter()
                .flat_map(|&(x, y)| vec![x, y, 0.0f32])
                .collect();

            unsafe {
                gl::BindBuffer(gl::ARRAY_BUFFER, points_vbo);
                gl::BufferData(gl::ARRAY_BUFFER,
                               (std::mem::size_of::<f32>() * data.len()) as isize,
                               data.as_ptr() as *const _,
                               gl::STATIC_DRAW);
                gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
            }

            Ok({})
        },

        Picture::Line(x1, y1, x2, y2) => {
            let points = vec![x1, y1, 0.0f32,
                              x2, y2, 0.0f32];
            unsafe {
                gl::BindBuffer(gl::ARRAY_BUFFER, points_vbo);
                gl::BufferData(gl::ARRAY_BUFFER,
                               (std::mem::size_of::<f32>() * points.len()) as isize,
                               points.as_ptr() as *const _,
                               gl::STATIC_DRAW);
                gl::DrawArrays(gl::LINES, 0, 2);
            }
            Ok({})
        },

        Picture::Pictures(ref pictures) => {
            pictures
                .iter()
                .map(|picture| render_picture(picture, points_vbo, color_loc))
                .collect::<Result<Vec<_>>>()
                .map(|_| ())
        },

        // TODO(#12): Rethink the Color combinator implementation
        //
        // I think in Gloss it behaves a little bit different. We need to research that.
        Picture::Color(r, g, b, ref boxed_picture) => {
            unsafe {
                gl::Uniform3fv(color_loc, 1, vec![r, g, b].as_ptr())
            }
            render_picture(boxed_picture.as_ref(), points_vbo, color_loc)
        }

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

    let _context = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    let mut event_pump = sdl2_context.event_pump()?;

    let mut state = init_state;

    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    let vertex_shader = Shader::from_str(gl::VERTEX_SHADER, include_str!("shaders/vertex.glsl"))?;
    let frag_shader = Shader::from_str(gl::FRAGMENT_SHADER, include_str!("shaders/frag.glsl"))?;
    let circle_shader = Shader::from_str(gl::FRAGMENT_SHADER, include_str!("shaders/circle.glsl"))?;

    let program = Program::from_shaders(vec![&vertex_shader, &frag_shader])?;
    let circle_program = Program::from_shaders(vec![&vertex_shader, &circle_shader])?;

    let color_loc = unsafe {
        gl::GetUniformLocation(
            program.id,
            std::ffi::CString::new("color").unwrap().as_ptr())
    };

    let radius_loc = unsafe {
        gl::GetUniformLocation(
            circle_program.id,
            std::ffi::CString::new("radius").unwrap().as_ptr())
    };

    let mut points_vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut points_vbo);
    }

    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, points_vbo);
        gl::VertexAttribPointer(0, 3,
                                gl::FLOAT,
                                gl::FALSE,
                                0,
                                std::ptr::null());

        gl::EnableVertexAttribArray(0);
    }

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
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Viewport(0, 0, width as i32, height as i32);
            program.use_program();
            gl::BindVertexArray(vao);
        }

        render_picture(&render(&state), points_vbo, color_loc)?;

        {
            let points = vec![-1.0f32, -1.0f32, 0.0f32,
                              1.0f32, -1.0f32, 0.0f32,
                              1.0f32, 1.0f32, 0.0f32,
                              -1.0f32, 1.0f32, 0.0f32];

            circle_program.use_program();
            unsafe {
                gl::BindVertexArray(vao);
                gl::BindBuffer(gl::ARRAY_BUFFER, points_vbo);
                gl::BufferData(gl::ARRAY_BUFFER,
                               (std::mem::size_of::<f32>() * points.len()) as isize,
                               points.as_ptr() as *const _,
                               gl::STATIC_DRAW);
                gl::Uniform1f(radius_loc, 100.0f32);
                gl::DrawArrays(gl::TRIANGLE_FAN, 0, points.len() as i32);
            }
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
