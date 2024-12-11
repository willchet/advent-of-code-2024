use advent_of_code::{
    day1, day2, day3, day4, day5, day6, day7, day8, day9, day10,
    parsers::{
        day1_parser, day2_parser, day3_parser, day4_parser, day5_parser, day6_parser, day7_parser,
        day8_parser, day9_parser, day10_parser,
    },
};
use anyhow::Result;
use clap::{Parser, Subcommand, command};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Day1 { file: Option<PathBuf> },
    Day2 { file: Option<PathBuf> },
    Day3 { file: Option<PathBuf> },
    Day4 { file: Option<PathBuf> },
    Day5 { file: Option<PathBuf> },
    Day6 { file: Option<PathBuf> },
    Day7 { file: Option<PathBuf> },
    Day8 { file: Option<PathBuf> },
    Day9 { file: Option<PathBuf> },
    Day10 { file: Option<PathBuf> },
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Day1 { file } => {
            let (list1, list2) = day1_parser(file)?;
            let (difference, score) = day1(list1, list2);
            println!("{difference}, {score}");
        }
        Commands::Day2 { file } => {
            let lists = day2_parser(file)?;
            let (count_no_mismatch, count_one_mismatch) = day2(lists);
            println!("{count_no_mismatch}, {count_one_mismatch}");
        }
        Commands::Day3 { file } => {
            let prgm = day3_parser(file)?;
            let val_no_cond = day3(&prgm, false);
            let val_cond = day3(&prgm, true);
            println!("{val_no_cond}, {val_cond}");
        }
        Commands::Day4 { file } => {
            let grid = day4_parser(file)?;
            let (word_count, cross_count) = day4(grid);
            println!("{word_count} {cross_count}");
        }
        Commands::Day5 { file } => {
            let (rules, updates) = day5_parser(file);
            let (correct_middle, sorted_middle) = day5(rules, updates);
            println!("{correct_middle} {sorted_middle}");
        }
        Commands::Day6 { file } => {
            let (grid, i, j) = day6_parser(file);
            let (visited_counter, cycle_counter) = day6(grid, i, j);
            println!("{visited_counter}, {cycle_counter}");
        }
        Commands::Day7 { file } => {
            let eqs = day7_parser(file);
            let (with_arith, with_concat) = day7(eqs);
            println!("{with_arith}, {with_concat}");
        }
        Commands::Day8 { file } => {
            let (antennas, rows, cols) = day8_parser(file);
            let (num_no_resonance, num_with_resonance) = day8(&antennas, rows, cols);
            println!("{num_no_resonance}, {num_with_resonance}");
        }
        Commands::Day9 { file } => {
            let layout = day9_parser(file);
            let (checksum_breaking, checksum_nonbreaking) = day9(layout);
            println!("{checksum_breaking}, {checksum_nonbreaking}");
        }
        Commands::Day10 { file } => {
            let terrain = day10_parser(file);
            let (score, num_paths) = day10(terrain);
            println!("{score}, {num_paths}");
        }
    }
    Ok(())

    // let (rules, updates) = day5_parser(Some("day5.txt".into()));
    // let (correct_middle, sorted_middle) = day5(rules, updates);
    // println!("{correct_middle} {sorted_middle}");
    // Ok(())
}
