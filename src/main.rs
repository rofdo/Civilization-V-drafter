use clap::Parser;
use log::info;
use rand::Rng;
use serde::{Deserialize, Serialize};

use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Args {
    /// The number of players to play the game
    #[arg(short, long, default_value = "6")]
    users: usize,
    /// The number of civilizations for each player to choose from
    #[arg(short, long, default_value = "5")]
    picks: usize,
    /// A list of civilizations to ban
    #[arg(long, default_value = "[]", num_args = 1..)]
    banned_civs: Vec<String>,
    /// A list of leaders to ban
    #[arg(long, default_value = "[]", num_args = 1..)]
    banned_leaders: Vec<String>,
    /// A list of biases to ban
    #[arg(long, default_value = "[]", num_args = 1..)]
    banned_biases: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Civ {
    name: String,
    leader: String,
    bias: Vec<String>,
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
    env_logger::init();

    let path = PathBuf::from("civilizations.json");
    let args = Args::parse();
    let civs: Vec<Civ> = get_civs(path).unwrap();
    let civs: Vec<Civ> = civs
        .into_iter()
        .filter(|civ| {
            !args.banned_civs.contains(&civ.name)
                || args.banned_leaders.contains(&civ.leader.to_lowercase())
                || args.banned_biases.iter().any(|bias| civ.bias.contains(&bias.to_lowercase()) )
        })
        .collect();
    info!("total civs: {}", civs.len());

    let result = choose_civs(&mut civs.clone(), args.users, args.picks).unwrap_or_else(|err| {
        log::error!("{}", err);
        std::process::exit(1);
    });
    for (i, player) in result.iter().enumerate() {
        let civs = player
            .iter()
            .map(|civ| civ.name.clone())
            .collect::<Vec<String>>()
            .join(", ");
        println!("Player {}: {}", i + 1, civs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_choose_civs() {
        let mut civs = vec![
            Civ {
                name: "America".to_string(),
                leader: "Teddy".to_string(),
                bias: vec!["None".to_string()],
            },
            Civ {
                name: "Arabia".to_string(),
                leader: "Saladin".to_string(),
                bias: vec!["None".to_string()],
            },
            Civ {
                name: "Aztec".to_string(),
                leader: "Montezuma".to_string(),
                bias: vec!["None".to_string()],
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
