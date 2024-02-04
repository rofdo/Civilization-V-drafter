use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    /// The number of players to play the game
    #[arg(short, long)]
    players: usize,
    /// The number of civilizations for each player to choose from
    #[arg(short, long)]
    civs: usize,
}

fn main() {
    println!("Hello, world!");
}
