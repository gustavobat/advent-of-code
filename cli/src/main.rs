mod parser;

use crate::parser::Cli;
use crate::parser::Commands;
use crate::parser::Day;
use crate::parser::Year;
use aoc24 as _;
use clap::Parser;
use colored::Colorize;
use spinners::Spinner;
use std::path::PathBuf;
use utils::solution::Solver;

fn make_input_path(year: Year, day: Day) -> PathBuf {
    let year = year.value();
    let day = day.value();
    let package_path = format!("aoc{}", year % 2000);
    PathBuf::from(format!("./{package_path}/resources/input/{day:02}.txt",))
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { year, day, input } => {
            let Some(solver) = inventory::iter::<Solver>()
                .find(|solver| solver.year == year.value() && solver.day == day.value())
            else {
                anyhow::bail!(
                    "No solver found for year {} day {}",
                    year.value(),
                    day.value()
                );
            };

            let input_path = input.unwrap_or(make_input_path(year, day));

            println!("{}: {}", "Year".bold().green(), year.value());
            println!("{}: {}", "Day".bold().green(), day.value());
            println!("{}: {}", "Input".bold().green(), input_path.display());
            println!();

            let input = std::fs::read_to_string(input_path)?;

            let mut spinner = Spinner::new(spinners::Spinners::Dots9, "Solving ...".into());
            let result = (solver.solver)(&input);

            let success_symbol = "✔".green().to_string();
            let failure_symbol = "✘".red().to_string();

            let solution = result.inspect_err(|_| {
                spinner
                    .stop_and_persist(&failure_symbol, "An error occurred during solution.".into());
            })?;
            spinner.stop_and_persist(&success_symbol, "Solution complete.".into());

            println!();
            println!("{}\n{}", "Part one:".green().bold(), solution.part_one);
            println!("{}\n{}", "Part two:".green().bold(), solution.part_two);

            Ok(())
        }
        Commands::GetInput { year, day } => {
            let year = year.value();
            let day = day.value();

            dotenvy::dotenv().ok();
            let session_token = std::env::var("AOC_SESSION").ok();
            let Some(session_token) = &session_token else {
                anyhow::bail!("Could not find AOC_SESSION in `.env`.");
            };
            println!("Found `AOC_SESSION` token!");

            println!("Downloading input for year {year}, day {day}...");
            let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
            let client = reqwest::blocking::Client::new();
            let response = client
                .get(&url)
                .header("cookie", format!("session={}", session_token))
                .send()?;
            if response.status() != reqwest::StatusCode::OK {
                anyhow::bail!(
                    "Failed to download input. Make sure your token is correct and up-to-date."
                );
            }
            let content = response.text()?;

            let package_path = format!("aoc{}", year % 2000);
            let input_dir = format!("./{package_path}/resources/input",);
            let input_file = format!("{input_dir}/{day:02}.txt");
            println!("Writing input to {input_file}");

            std::fs::create_dir_all(&input_dir)?;
            std::fs::write(&input_file, content)?;

            Ok(())
        }
    }
}
