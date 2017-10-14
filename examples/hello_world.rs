extern crate gross;

use gross::*;

struct State {
    x: f32,
    y: f32
}

fn main() {
    use Picture::*;

    gross::simulate(gross::Display::InWindow((10, 10), (800, 600)),
                    State { x: 0.0f32, y: 0.0f32 },
                    |ref s| Pictures(vec![Color(1.0f32, 0.0f32, 0.0f32,
                                                Box::new(Rectangle(s.x, s.y, 0.3f32, 0.3f32))),
                                          Color(0.0f32, 1.0f32, 0.0f32,
                                                Box::new(Line(s.x, s.y, 1.0f32, 1.0f32)))]),
                    |mut s| {
                        s.x = (s.x + 0.01f32) % 1.0f32;
                        s
                    }).unwrap();
}
