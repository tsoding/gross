extern crate gross;

use gross::*;

struct State {
    x: i32,
    y: i32
}

fn main() {
    use Picture::*;

    gross::simulate(gross::Display::InWindow((10, 10), (800, 600)),
                    State { x: 0, y: 0 },
                    |ref s| Pictures(vec![Rectangle(s.x, s.y, 100, 100),
                                          Line(s.x, s.y, 800, 600)]),
                    |mut s| {
                        s.x = (s.x + 1) % 800;
                        s
                    }).unwrap();
}
