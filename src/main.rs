use reqwest;
use serde_json::Value;
use polars::prelude::*;

fn get_actual_value(value: Value) -> Option<String> {
    match value {
        Value::String(string_value) => Some(string_value),
        Value::Number(number_value) => Some(number_value.to_string()),
        Value::Bool(bool_value) => Some(bool_value.to_string()),
        Value::Null => Some("null".to_string()),
        _ => None,
    }
}

// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("authority", "www.oddsportal.com".parse().unwrap());
    headers.insert("accept", "application/json, text/plain, */*".parse().unwrap());
    headers.insert("accept-language", "en-US,en-GB;q=0.9,en;q=0.8".parse().unwrap());
    headers.insert("content-type", "application/json".parse().unwrap());
    headers.insert(reqwest::header::COOKIE, "op_cookie-test=ok; op_user_time=1694833059; op_user_cookie=5934834206; op_user_hash=9a17e2fc8d0b65105daae26aee2e47d1; _ga=GA1.1.2015866974.1694833061; op_user_time_zone=8; op_user_full_time_zone=75; OptanonAlertBoxClosed=2023-09-16T02:57:43.336Z; _sg_b_n=1694833063372; _hjSessionUser_3147261=eyJpZCI6ImU4ZjRiYzQwLWE4MTMtNTEzMC05Yjg0LTZhN2RhZjgzMDlkYSIsImNyZWF0ZWQiOjE2OTQ4MzMwNjA2MzMsImV4aXN0aW5nIjp0cnVlfQ==; eupubconsent-v2=CPyLPsAPyLPsAAcABBENDYCsAP_AAAAAAChQJGtf_X__b2_j-_7-f_t0eY1P9_7_v-0zjhfdF-8N2f_X_L8X52M5vF16pqoKuR4ku3bBIQVlHOHcDUmw6okVryPsbk2cr7NKJ7PkmlMbM2dYGH9_n93T-ZKY7___f__z_v-v___9____7-3f3__p__--2_e_V_89zfn9_____9vP___9v-_9_3gAAAAAAAAAAAAD4AAABQkAIAGgC8x0AQAGgAZgBlALzIQAQBlEoAIC8ykAQAGgAZgBlALz.f_gAAAAAAAAA; _hjIncludedInSessionSample_3147261=0; _hjSession_3147261=eyJpZCI6IjE4NjUyN2JkLWI1MTEtNDkzOC1hZTA3LTY1ZGY0ZTc5YmRjMSIsImNyZWF0ZWQiOjE2OTU0NTg2NjM5MTIsImluU2FtcGxlIjpmYWxzZX0=; _hjAbsoluteSessionInProgress=1; OptanonConsent=isGpcEnabled=0&datestamp=Sat+Sep+23+2023+16%3A44%3A23+GMT%2B0800+(Singapore+Standard+Time)&version=202210.1.0&isIABGlobal=false&hosts=&consentId=93886366-8b53-4b16-9566-9951a695145f&interactionCount=1&landingPath=NotLandingPage&groups=C0001%3A1%2CC0002%3A1%2CC0004%3A1%2CSTACK42%3A1&geolocation=SG%3B&AwaitingReconsent=false; _sg_b_p=%2Ffootball%2Fengland%2Fpremier-league-2022-2023%2Fresults%2F; _sg_b_v=5%3B39650%3B1695458664; _ga_5YY4JY41P1=GS1.1.1695458663.6.1.1695458827.60.0.0".parse().unwrap());
    headers.insert("referer", "https://www.oddsportal.com/football/england/premier-league-2021-2022/results/".parse().unwrap());
    headers.insert("sec-ch-ua", "\"Chromium\";v=\"116\", \"Not)A;Brand\";v=\"24\", \"Google Chrome\";v=\"116\"".parse().unwrap());
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"macOS\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "empty".parse().unwrap());
    headers.insert("sec-fetch-mode", "cors".parse().unwrap());
    headers.insert("sec-fetch-site", "same-origin".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert("x-requested-with", "XMLHttpRequest".parse().unwrap());

    let client = reqwest::Client::new();
    let response = client.get("https://www.oddsportal.com/ajax-sport-country-tournament-archive_/1/tdkpynmB/X0/1/0/?_=1695458829998")
    .headers(headers)
    .send()
    .await?
    .json::<Value>()
    .await?;

    // Access and work with the JSON data using serde_json::Value methods
    let json = &response["d"]["rows"].as_array().unwrap();
    let json_len: usize = json.len();

    let mut all_fields_res: Vec<Series> = vec![];
    let all_fields: [&str; 2] = ["home-name", "away-name"];
    for f in all_fields {
        let mut fields: Vec<String> = vec![];
        for i in 0..json_len {
            let json_val = get_actual_value(json[i][f].clone()).unwrap();
            let mut json_val = vec![json_val];
            fields.append(&mut json_val);
        }
        let mut fields: Vec<Series> = vec![Series::new(f, fields)];
        all_fields_res.append(&mut fields);
    }

    let mut all_home_odds: Vec<String> = vec![];
    let mut all_draw_odds: Vec<String> = vec![];
    let mut all_away_odds: Vec<String> = vec![];

    for i in 0..json_len {
        let all_odds: &Vec<Value> = json[i]["odds"].as_array().unwrap();

        let home_odd: &Value = &all_odds[0]["avgOdds"];
        let draw_odd: &Value = &all_odds[1]["avgOdds"];
        let away_odd: &Value = &all_odds[2]["avgOdds"];

        let home_odd: String = get_actual_value(home_odd.clone()).unwrap();
        let draw_odd: String = get_actual_value(draw_odd.clone()).unwrap();
        let away_odd: String = get_actual_value(away_odd.clone()).unwrap();

        let mut home_odd: Vec<String> = vec![home_odd];
        let mut draw_odd: Vec<String> = vec![draw_odd];
        let mut away_odd: Vec<String> = vec![away_odd];

        all_home_odds.append(&mut home_odd);
        all_draw_odds.append(&mut draw_odd);
        all_away_odds.append(&mut away_odd);   
    }

    let mut all_home_odds: Vec<Series> = vec![Series::new("home_odds", all_home_odds)];
    let mut all_draw_odds: Vec<Series> = vec![Series::new("draw_odds", all_draw_odds)];
    let mut all_away_odds: Vec<Series> = vec![Series::new("away_odds", all_away_odds)];

    all_fields_res.append(&mut all_home_odds);
    all_fields_res.append(&mut all_draw_odds);
    all_fields_res.append(&mut all_away_odds);

    let df = DataFrame::new(all_fields_res).unwrap();

    println!("{:?}", df);

    Ok(())
}
