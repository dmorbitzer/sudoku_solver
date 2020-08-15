enum Tiel {
    Empty,
    Number(i8)
}

struct Row (
    Tiel,
    Tiel,
    Tiel
);

impl Row {
    pub fn new() -> Row {
        Self(
            Tiel::Empty,
            Tiel::Empty,
            Tiel::Empty
        )
    }
}

struct Field (
    Row,
    Row,
    Row
);

impl Field {
    pub fn new() -> Field {
        Self(
            Row::new(),
            Row::new(),
            Row::new()
        )
    }
}

struct Game_Field (
    Field,
    Field,
    Field,

    Field,
    Field,
    Field,

    Field,
    Field,
    Field
);

impl Game_Field {
    pub fn new() ->Game_Field {
        Self(
            Field::new(),
            Field::new(),
            Field::new(),

            Field::new(),
            Field::new(),
            Field::new(),

            Field::new(),
            Field::new(),
            Field::new(),
        )
    }
}

fn main() {
    let mut game_field = Game_Field::new();
}
