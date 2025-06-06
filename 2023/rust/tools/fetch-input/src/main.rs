use std::{
    fs::{File, create_dir_all},
    io::Write,
    path::PathBuf,
};

use nom::{
    IResult, Parser, bytes::complete::tag,
    character::complete, sequence::preceded,
};
use reqwest::{blocking::Client, header::COOKIE};

#[derive(clap::Parser, Debug)]
#[clap(version)]
struct Args {
    /// Years may pass, but the pursuit of skill
    /// mastery continues.
    #[clap(short, long)]
    year: u32,
    /// Day is expected to be formatted as
    /// "day-01".
    #[clap(short, long)]
    day: String,
    /// Just a way to pass in the Justfile's
    /// directory.
    #[clap(long)]
    current_working_directory: PathBuf,
}

fn parse_day(input: &str) -> IResult<&str, u32> {
    parse_prefixed_u32("day-", input)
}

fn parse_prefixed_u32<'a>(
    prefix: &str,
    input: &'a str,
) -> IResult<&'a str, u32> {
    preceded(tag(prefix), complete::u32).parse(input)
}

fn main() -> anyhow::Result<()> {
    use anyhow::Context;
    use clap::{CommandFactory, Parser, error::ErrorKind};

    let session = std::env::var("SESSION")
        .expect("Should have a session token set");
    let args = Args::parse();

    let Ok((_, day)) = parse_day(&args.day) else {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::ValueValidation,
            format!(
                "Day `{}` must be formatted as `day-01`",
                args.day
            ),
        )
        .exit()
    };

    let url = format!(
        "https://adventofcode.com/{}/day/{}/input",
        args.year, day
    );

    println!("Getting input from `{url}`");
    println!("session={session}");

    let client = Client::new();
    let response = client
        .get(url)
        .header(COOKIE, format!("session={session}"))
        .send()
        .context("Failed to send request")?
        .text()
        .context("Failed to get response text")?;

    let input_data = response.as_bytes();

    let dir_path =
        args.current_working_directory.join(&args.day);
    create_dir_all(&dir_path).with_context(|| {
        format!(
            "Failed to create directory {}",
            dir_path.display()
        )
    })?;

    for filename in ["input1.txt", "input2.txt"] {
        let file_path = dir_path.join(filename);
        let mut file = File::create(&file_path)
            .with_context(|| {
                format!(
                    "Should create file {}",
                    file_path.display()
                )
            })?;
        file.write_all(input_data)
            .expect("Should write input file");

        println!("Wrote {}", file_path.display());
    }

    Ok(())
}
