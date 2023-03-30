use serde::{Deserialize, Serialize};
use std::{fs, io, path};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    response: ApiResponseContent,
}

#[derive(Serialize, Deserialize)]
struct ApiResponseContent {
    game_count: u32,
    games: Vec<ApiGame>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiGame {
    appid: u32,
    playtime_forever: u32,
    playtime_windows_forever: u32,
    playtime_mac_forever: u32,
    playtime_linux_forever: u32,
    rtime_last_played: u64,
}

#[derive(Serialize, Deserialize)]
struct ReturnedData {
    response: Vec<Game>,
}

impl ReturnedData {
    fn new(response: Vec<Game>) -> Self {
        ReturnedData { response }
    }
}

#[derive(Serialize, Deserialize)]
struct Game {
    appid: u32,
    playtime: u32,
}

impl Game {
    fn new(appid: u32, playtime: u32) -> Self {
        Game { appid, playtime }
    }
}

fn get_api_key() -> io::Result<String> {
    if !path::Path::new(".env").exists() {
        fs::File::create(".env")?;
        fs::write(".env", "STEAM_API_KEY=")?;
    }

    let contents = fs::read_to_string(".env")?;

    if !contents.starts_with("STEAM_API_KEY") {
        return Err(io::Error::new(io::ErrorKind::Other, "Invalid .env file."));
    }

    let split: Vec<&str> = contents.split_inclusive('=').collect();

    Ok(split[1].to_owned())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let api_key = get_api_key().expect("Failed to retrieve steam API key");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8789").await?;
    println!("Bound TCP listener. Ready for requests.");

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        println!("Incoming request from {}", addr);

        let api_key = api_key.clone();

        tokio::spawn(async move {
            let steam_id = socket.read_u64().await.unwrap();

            let api_url = format!("http://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?key={}&steamid={}&format=json", api_key, &steam_id);

            println!("Trying to retrieve data from steam api...");
            let api_response = reqwest::get(api_url).await.unwrap().text().await.unwrap();
            let deserialized: ApiResponse = serde_json::from_str(&api_response).unwrap();

            let mut games: Vec<Game> = vec![];

            for game in &deserialized.response.games {
                games.push(Game::new(game.appid, game.playtime_forever));
            }

            let serialized = serde_json::to_string_pretty(&ReturnedData::new(games)).unwrap();

            println!("Returning altered data...");
            println!("{}", serialized);
            socket.write_all(serialized.as_bytes()).await.unwrap();

            println!("Success! Closing connection.");
            socket.shutdown().await.unwrap();
        });
    }
}
