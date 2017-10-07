extern crate gross;

struct State {
    x: i32,
    y: i32
}

fn main() {
    gross::simulate(gross::Display::InWindow((10, 10), (800, 600)),
                    State { x: 0, y: 0 },
                    |ref s| gross::Picture::Pictures(vec![gross::Picture::Rectangle(s.x, s.y, 100, 100),
                                                          gross::Picture::Line(s.x, s.y, 800, 600)]),
                    |mut s| {
                        s.x = (s.x + 1) % 800;
                        s
                    }).unwrap();
}
