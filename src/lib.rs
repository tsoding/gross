pub enum Color {
    RGB(u8, u8, u8)
}

pub enum Display {
    FullScreen,
    InWindow(String, (i32, i32), (u32, u32))
}

pub enum Picture {
    Blank
}

pub fn simulate<S, R, U>(display: Display,
                         background_color: Color,
                         fps: u32,
                         init_state: S,
                         render: R,
                         update: U)
    where R: Fn(&S) -> Picture,
          U: Fn(S) -> S {

    let mut state = init_state;
    loop {
        let picture = render(&state);
        // ...actually render the state somehow...
        state = update(state);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
