mod parser;

use crate::parser::Cli;
use crate::parser::Commands;
use crate::parser::Day;
use crate::parser::Year;
use aoc24 as _;
use aoc25 as _;
use clap::Parser;
use colored::Colorize;
use human_repr::HumanDuration;
use spinners::Spinner;
use std::path::PathBuf;
use utils::solution::Solver;

fn collect_solvers(year: Option<Year>, day: Option<Day>) -> Vec<&'static Solver> {
    let mut solvers: Vec<&'static Solver> = inventory::iter::<Solver>()
        .filter(|s| match year {
            Some(y) => y.value() == s.year,
            None => true,
        })
        .filter(|s| match day {
            Some(d) => d.value() == s.day,
            None => true,
        })
        .collect();

    solvers.sort_by_key(|s| (s.year, s.day));
    solvers
}

fn expected_input_path_for_solver(solver: &Solver) -> PathBuf {
    let package_path = format!("aoc{}", solver.year % 2000);
    PathBuf::from(format!(
        "./{package_path}/resources/input/{:02}.txt",
        solver.day
    ))
}

fn solver_display_id(solver: &Solver) -> String {
    format!("{} {}", solver.year, solver.day)
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { year, day } => {
            let solvers = collect_solvers(year, day);

            if solvers.is_empty() {
                anyhow::bail!("No matching solvers found for filters.");
            }

            println!("{}: {}", "Matched solvers".bold().green(), solvers.len());

            let mut succeeded = 0usize;
            let mut failed = 0usize;
            let mut skipped = 0usize;
            let mut missing_inputs: Vec<(u16, u8, PathBuf)> = Vec::new();

            let mut total_solve_duration = std::time::Duration::from_secs(0);
            // Iterate by reference so `solvers` remains available for the final summary.
            for solver_ref in &solvers {
                let solver = *solver_ref; // &'static Solver
                let input_path = expected_input_path_for_solver(solver);

                if !input_path.exists() {
                    missing_inputs.push((solver.year, solver.day, input_path));
                    skipped += 1;
                    continue;
                }

                println!("{}: {}", "Year".bold().green(), solver.year);
                println!("{}: {}", "Day".bold().green(), solver.day);
                println!("{}: {}", "Input".bold().green(), input_path.display());
                println!();

                let input = match std::fs::read_to_string(&input_path) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("Failed to read input file: {}", e);
                        failed += 1;
                        continue;
                    }
                };

                let mut spinner = Spinner::new(spinners::Spinners::Dots9, "Solving ...".into());

                let start_time = std::time::Instant::now();
                let result = (solver.solver)(&input);
                let duration = start_time.elapsed();
                total_solve_duration += duration;
                let elapsed = duration.human_duration();

                let success_symbol = "✔".green().to_string();
                let failure_symbol = "✘".red().to_string();

                match result {
                    Ok(solution) => {
                        spinner.stop_and_persist(
                            &success_symbol,
                            format!("Solution found! Elapsed time: {}.", elapsed),
                        );
                        println!();
                        println!("{}\n{}", "Part one:".green().bold(), solution.part_one);
                        println!("{}\n{}", "Part two:".green().bold(), solution.part_two);
                        succeeded += 1;
                    }
                    Err(err) => {
                        spinner.stop_and_persist(
                            &failure_symbol,
                            "An error occurred during solution.".into(),
                        );
                        eprintln!(
                            "Solver error for year {} day {}: {}",
                            solver.year, solver.day, err
                        );
                        failed += 1;
                    }
                }

                println!();
            }

            let total_elapsed = total_solve_duration.human_duration();

            println!("===== Summary =====");
            println!("Total matched: {}", solvers.len());
            println!("Succeeded: {}", succeeded);
            println!("Failed: {}", failed);
            println!("Skipped (missing input): {}", skipped);
            println!("Total solve time: {}", total_elapsed);

            if !missing_inputs.is_empty() {
                println!("\nMissing inputs report:");
                for (y, d, path) in missing_inputs {
                    println!("- Year {} Day {} -> {}", y, d, path.display());
                }
            }

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
        Commands::List { year, day } => {
            let solvers = collect_solvers(year, day);
            if solvers.is_empty() {
                anyhow::bail!("No matching solvers found for filters.");
            }
            println!("Matched solvers ({}):", solvers.len());
            for s in &solvers {
                println!("- {}", solver_display_id(s));
            }
            Ok(())
        }
    }
}
