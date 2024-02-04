use clap::Parser;
use rand::Rng;
use serde::{Deserialize, Serialize};

use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Args {
    /// The number of players to play the game
    #[arg(short, long)]
    players: usize,
    /// The number of civilizations for each player to choose from
    #[arg(short, long)]
    picks: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Civ {
    name: String,
}

fn choose_civs(
    civs: &mut Vec<Civ>,
    players: usize,
    picks: usize,
) -> Result<Vec<Vec<Civ>>, Box<dyn std::error::Error>> {
    if players * picks > civs.len() {
        return Err("Not enough civs to choose from".into());
    }
    let mut result = vec![vec![]; players];
    for _ in 0..picks {
        for player in 0..players {
            let index = rand::thread_rng().gen_range(0..civs.len());
            let civ = civs.remove(index);
            result[player].push(civ);
        }
    }
    Ok(result)
}

fn get_civs(path: PathBuf) -> Result<Vec<Civ>, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let civs: Vec<Civ> = serde_json::from_reader(file)?;
    Ok(civs)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_choose_civs() {
        let mut civs = vec![
            Civ {
                name: "America".to_string(),
            },
            Civ {
                name: "Arabia".to_string(),
            },
            Civ {
                name: "Australia".to_string(),
            },
        ];
        let players = 3;
        let picks = 1;
        let result = choose_civs(&mut civs, players, picks).unwrap();
        assert_eq!(result.len(), players);
        assert_eq!(result[0].len(), picks);
    }

    #[test]
    fn test_get_civs() {
        let path = PathBuf::from("civilizations.json");
        let result = get_civs(path).unwrap();
        assert_eq!(result.len(), 109);
    }
}
