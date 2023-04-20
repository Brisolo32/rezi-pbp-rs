use std::{path, fs, io::Write};

use reqwest::Client;
use serde::Deserialize;

// Structs to recieve the response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    hits: Vec<Hit>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hit {
    link: String,
    title: String,
}

// Structs to serialize the JSON to results.json
#[derive(serde::Serialize, Debug)]
struct Response {
    response: Vec<Game>,
}

#[derive(serde::Serialize, Debug)]
struct Game {
    title: String,
    urls: Vec<String>,
}

// Struct for the query serialization
#[derive(serde::Serialize, Debug)]
struct RequestBody {
	q: String,
	limit: i32
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args: Vec<String> = std::env::args().collect();

	let query = format!(r#"{{ "q": {:?}, "limit": 35 }} "#, &args[1]);
	println!("{:?}", query);
	let destination_folder = &args[2];

	let folder_path = path::Path::new(destination_folder);
	let file_path = folder_path.join("results.json");

	let mut file = fs::File::create(file_path).expect("Failed to create file");

	let client = Client::new();

	let res = client
		.post("https://search.rezi.one/indexes/rezi/search")
		.bearer_auth("e2a1974678b37386fef69bb3638a1fb36263b78a8be244c04795ada0fa250d3d")
		.header("Content-Type", "application/json")
		.body(query)
		.send()
		.await?;

	let response_text = res.text().await?;

	let deserialized_res = serde_json::from_str::<Root>(&response_text).expect("JSON was not formatted!");
	
	
	let mut games_vec: Vec<Game> = vec![];
	let mut i = 0;

	for games in deserialized_res.hits {
		if i < 20 {
			let mut urls = vec![];

			urls.push(games.link);
			let title = games.title;

			games_vec.push(Game { title, urls });
			i += 1;
		}
	}

	let response = Response { response: games_vec };
	let json = serde_json::to_vec_pretty(&response).expect("JSON serialization failed");
	file.write_all(&json).expect("Failed to write file");

	Ok(())
}
