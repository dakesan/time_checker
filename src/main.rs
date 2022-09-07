use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct TimeData {
    abbreviation: String,
    client_ip: String,
    datetime: String,
    day_of_week: i32,
    day_of_year: i32,
    dst: bool,
    dst_from: Option<String>,
    dst_offset: i32,
    dst_until: Option<String>,
    raw_offset: i32,
    timezone: String,
    unixtime: i32,
    utc_datetime: String,
    utc_offset: String,
    week_number: i32,
}

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let local: DateTime<Local> = Local::now();
    
    let body = reqwest::get("http://worldtimeapi.org/api/timezone/Asia/Tokyo");
    let json_value = body.await?.json::<TimeData>().await?;
    let date_online = DateTime::parse_from_rfc3339(&json_value.datetime).unwrap().with_timezone(&local.timezone());
    
    let diff = date_online - local;

    println!("Online time: {:?}", date_online);
    println!("Local time : {:?}", local);
    println!();
    if diff.num_milliseconds() > 0 {
        println!("機械時間が {:.3}ms 遅れてるっぽいです🐢", diff);
    }
    println!("機械時間が {:.3}ms 進んでるっぽいです🐇", -diff);
    Ok(())
}