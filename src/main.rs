use clap::Parser;
use rand::Rng;

#[derive(Debug, Parser)]
struct Args {
    /// The number of players to play the game
    #[arg(short, long)]
    players: usize,
    /// The number of civilizations for each player to choose from
    #[arg(short, long)]
    picks: usize,
}

type Civ = u32;

fn choose_civs(civs: &mut Vec<Civ>, players: usize, picks: usize) -> Vec<Vec<u32>> {
    let mut result = vec![vec![]; players];
    for _ in 0..picks {
        for player in 0..players {
            let index = rand::thread_rng().gen_range(0..civs.len());
            let civ = civs.remove(index);
            result[player].push(civ);
        }
    }
    result
}

fn main() {
    println!("Hello, world!");
}
