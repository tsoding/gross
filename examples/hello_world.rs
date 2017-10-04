extern crate gross;

struct State {
    x: i32,
    y: i32
}

fn main() {
    gross::simulate(gross::Display::FullScreen,
                    gross::Color::RGB(0, 0, 0),
                    30,
                    State { x: 0, y: 0 },
                    |ref s| gross::Picture::Blank,
                    |mut s| {
                        s.x = (s.x + 10) % 100;
                        s
                    })
}
