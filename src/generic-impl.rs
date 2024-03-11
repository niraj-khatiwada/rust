trait Game {
    fn name(&self) -> String;
}

enum BoardGame {
    Chess,
    Ludo,
}
impl Game for BoardGame {
    fn name(&self) -> String {
        match self {
            BoardGame::Chess => String::from("Chess"),
            BoardGame::Ludo => String::from("Ludo"),
        }
    }
}

enum VideoGame {
    Fortnite,
    CounterStrike,
}
impl Game for VideoGame {
    fn name(&self) -> String {
        match self {
            VideoGame::Fortnite => String::from("Fortnite"),
            VideoGame::CounterStrike => String::from("CounterStrike"),
        }
    }
}

struct WeekendGame<T: Game> {
    game: T,
}
// We usually do this when we want exclusive method for this generic structure of VideoGame
impl WeekendGame<VideoGame> {
    fn is_mouse_required(&self) -> bool {
        match self.game {
            VideoGame::Fortnite => true,
            VideoGame::CounterStrike => false,
        }
    }
}

// We usually do this when we want exclusive method for this generic structure of BoardGame
impl WeekendGame<BoardGame> {
    fn is_digital(&self) -> bool {
        match self.game {
            BoardGame::Chess => true,
            BoardGame::Ludo => false,
        }
    }
}

// Generic impl for above 2 WeekendGame<VideoGame> and WeekendGame<BoardGame>
// We usually do this when we want common method for generic structure
impl<T: Game> WeekendGame<T> {
    fn is_video_game(&self) -> bool {
        self.game.name() == "Fortnite" || self.game.name() == "CounterStrike"
    }
}

fn main() {
    let video_game: WeekendGame<VideoGame> = WeekendGame {
        game: VideoGame::Fortnite,
    };
    // Here is_mouse_required is only available for VideoGame
    println!(
        "Is mouse require for this video game? {:?}",
        video_game.is_mouse_required()
    );
    println!("Is video game? {:?}", video_game.is_video_game());

    let board_game: WeekendGame<BoardGame> = WeekendGame {
        game: BoardGame::Ludo,
    };
    // Here is_digital is only available for BoardGame
    println!("Is this board game digital? {:?}", board_game.is_digital());
    println!("Is video game?: {:?}", board_game.is_video_game())

    // Verdict: impl is used if we want custom methods for a generic str
}
