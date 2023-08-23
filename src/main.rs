use anyhow::Context;
use chrono::{DateTime, Utc};
use clap::Parser;
use policer::police;
use serde::Deserialize;
use std::io::{self, Read, Write};

#[derive(Debug, Deserialize)]
pub struct Duration {
    weeks: Option<i64>,
    days: Option<i64>,
    hours: Option<i64>,
    minutes: Option<i64>,
    seconds: Option<i64>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Policy to apply provided as array of durations in JSON format (e.g., '[{"days": 4}, {"hours": 2}]')
    #[arg(short, long)]
    policy: String,
}

impl From<&Duration> for chrono::Duration {
    fn from(value: &Duration) -> Self {
        chrono::Duration::seconds(value.seconds.unwrap_or(0))
            + chrono::Duration::minutes(value.minutes.unwrap_or(0))
            + chrono::Duration::hours(value.hours.unwrap_or(0))
            + chrono::Duration::days(value.days.unwrap_or(0))
            + chrono::Duration::weeks(value.weeks.unwrap_or(0))
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let policy: Vec<chrono::Duration> = serde_json::from_str::<Vec<Duration>>(&args.policy)?
        .iter()
        .map(|elem| elem.into())
        .collect();
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let list = serde_json::from_str::<Vec<String>>(&input)?
        .iter()
        .map(|elem| {
            let file_part = elem
                .split("/")
                .last()
                .ok_or(anyhow::anyhow!("could not find file part"))?;
            let date_part = file_part
                .split("_")
                .next()
                .ok_or(anyhow::anyhow!("could not find date part"))?;
            let date = date_part
                .parse::<DateTime<Utc>>()
                .context(format!("error parsing date: {}", date_part))?;

            Ok((date, elem.to_string()))
        })
        .collect::<anyhow::Result<Vec<(DateTime<Utc>, String)>>>()?;

    io::stdout().write_all(
        serde_json::to_string(
            &police(&Utc::now(), &policy, &list)
                .iter()
                .map(|&elem| elem.1.to_string())
                .collect::<Vec<String>>(),
        )
        .context("error policing entries")?
        .as_bytes(),
    )?;

    Ok(())
}
