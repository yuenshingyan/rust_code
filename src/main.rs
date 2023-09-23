use serde::{Deserialize};
use reqwest;
use polars::prelude::*;

#[derive(Debug, Deserialize)]
struct Data {
    name: String,
    url: String,
    #[serde(rename = "sport-id")]
    sport_id: u32,
    #[serde(rename = "sport-url")]
    sport_url: String,
    #[serde(rename = "tournament-id")]
    tournament_id: u32,
}

#[derive(Debug, Deserialize)]
struct D {
    data: Vec<Data>,
    title: String,
    section_link: String,
    section_link_name: String,
}

#[derive(Debug, Deserialize)]
struct Response {
    s: u32,
    d: D,
    refresh: u32,
}

// {
//     "s": 1,
//     "d": {
//         "data": [
//             {
//                 "name": "Premier League",
//                 "url": "\/football\/england\/premier-league\/",
//                 "sport-id": 1,
//                 "sport-url": "football",
//                 "tournament-id": 1
//             },
//         ],
//         "title": "Top Events",
//         "section_link": "\/events\/",
//         "section_link_name": "All events"
//     },
//     "refresh": 20
// }

// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("sec-ch-ua", "\"Chromium\";v=\"116\", \"Not)A;Brand\";v=\"24\", \"Google Chrome\";v=\"116\"".parse().unwrap());
    headers.insert("X-XSRF-TOKEN", "eyJpdiI6Im5CZnkzbGdDV2dCKzJtT0l6SXFXSEE9PSIsInZhbHVlIjoiTHlSZGFkZUVCMzNLdk5oT0xKOHA0RHNFNk9kNldFTUtqR2gxUytHVml6MEljcmY2aGNjK0czNTl1VVFha0JXTHJkdlJHbWptWkFCMjJsTTRzTFJPNFVHbXp4U0kxYlNEOW11Zi9yVmFkZUtXKzJtOTVtWi94VU1mY0YwNXpPN3kiLCJtYWMiOiI2NTMzZjEzYmY2MjI1OTlmYmJlYTA0Y2E1MGY5OTExYTRiMTMxYzk0MjJiMTcyZmE2N2Y5YTA3NmM0MjY0MDg1IiwidGFnIjoiIn0=".parse().unwrap());
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert("Accept", "application/json, text/plain, */*".parse().unwrap());
    headers.insert("Referer", "https://www.oddsportal.com/football/england/premier-league/results/".parse().unwrap());
    headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"macOS\"".parse().unwrap());

    let client = reqwest::Client::new();
    let response: Response = client.get("https://www.oddsportal.com/ajax-all-events/topEvents")
    .headers(headers)
    .send()
    .await?
    .json()
    .await?;

    println!("{:?}", response.d.data);

    // Collect fields across structs in vector.
    let names: Vec<String> = response.d.data.iter().map(|p| p.name.clone()).collect();
    let urls: Vec<String> = response.d.data.iter().map(|p| p.url.clone()).collect();
    let sport_ids: Vec<u32> = response.d.data.iter().map(|p| p.sport_id).collect();
    let sport_urls: Vec<String> = response.d.data.iter().map(|p| p.sport_url.clone()).collect();
    let tournament_ids: Vec<u32> = response.d.data.iter().map(|p| p.tournament_id).collect();

    // Convert fields into Polars' series.
    let names_series = Series::new("names", names);
    let urls_series = Series::new("urls", urls);
    let sport_ids_series = Series::new("sport_ids", sport_ids);
    let sport_urls_series = Series::new("sport_urls", sport_urls);
    let tournament_ids_series = Series::new("tournament_ids", tournament_ids);

    // Make dataframe from pl series.
    let df = DataFrame::new(vec![names_series, urls_series, sport_ids_series, sport_urls_series, tournament_ids_series]);

    println!("{:?}", df);

    Ok(())
}
