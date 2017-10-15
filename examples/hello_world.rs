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
                                                Box::new(Picture::Polygon(vec![(s.x, s.y + 0.2f32),
                                                                               (s.x + 0.1f32, s.y),
                                                                               (s.x, s.y - 0.2f32),
                                                                               (s.x - 0.1f32, s.y)]))),
                                          Color(0.0f32, 1.0f32, 0.0f32,
                                                Box::new(Line(s.x, s.y, 1.0f32, 1.0f32))),
                                          // TODO: use GL coordinates iso screen coordinatse
                                          Circle(25.0)]),
                    |mut s| {
                        s.x = (s.x + 0.01f32) % 1.0f32;
                        s
                    }).unwrap();
}
