use askama::{self, Template};
use axum::{extract, response};

use crate::web;

#[derive(Debug)]
struct Player {
    name: String,
    kills: u16,
    deaths: u16,
    assits: u16,
}

#[derive(Debug)]
struct Game {
    player: Player,
    enemy: Player,
    win: bool,
}

#[derive(askama::Template, Debug)]
#[template(path = "games.html")]
struct GamesTemplate {
    games: Vec<Game>,
}

pub async fn get(
    extract::State(pool): extract::State<sqlx::Pool<sqlx::Postgres>>,
) -> web::error::Result<response::Html<String>> {
    let games_template = GamesTemplate {
        games: vec![Game {
            player: Player {
                name: "Graves".to_string(),
                kills: 12,
                deaths: 2,
                assits: 5,
            },
            enemy: Player {
                name: "Master Yi".to_string(),
                kills: 2,
                deaths: 7,
                assits: 3,
            },
            win: true,
        }],
    };
    Ok(response::Html(games_template.render()?))
}
