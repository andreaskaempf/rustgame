use bracket_lib::prelude::*;

// Game state
struct State {}

// Trait for game state
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello, Bracket Terminal!");
    }
}

fn main() -> BError {
    println!("Hello, world!");

    // Create Bracket Terminal context
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;  // pass errors to parent function

    // Start executing the game loop
    main_loop(context, State {})  // no semicolon, returns result
}
