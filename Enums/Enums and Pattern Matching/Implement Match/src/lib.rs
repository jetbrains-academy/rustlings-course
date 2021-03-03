pub enum Message {
    ChangeColor(i32, i32, i32),
    Echo(String),
    Move{x: u8, y: u8},
    Quit,
}

pub struct Point {
    pub x: u8,
    pub y: u8,
}

pub struct State {
    pub color: (u8, u8, u8),
    pub position: Point,
    pub quit: bool,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        match message {
            Message::ChangeColor(r, g, b) => self.change_color((r as u8, g as u8, b as u8)),
            Message::Echo(s) => self.echo(s),
            Message::Move{x, y} => self.move_position(Point{x, y}),
            Message::Quit => self.quit()
        }
    }
}


pub fn test_match_message_call() -> State {
    let mut state = State {
        quit: false,
        position: Point { x: 0, y: 0 },
        color: (0, 0, 0),
    };
    state.process(Message::ChangeColor(255, 0, 255));
    state.process(Message::Echo(String::from("hello world")));
    state.process(Message::Move{ x: 10, y: 15 });
    state.process(Message::Quit);

    return state
}
