use anyhow::{Context, Result, anyhow};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
    path::PathBuf,
};

use crate::LabEntry;

pub fn get_reader(path: Option<PathBuf>) -> Result<BufReader<Box<dyn std::io::Read>>> {
    Ok(if let Some(ref file_path) = path {
        BufReader::new(Box::new(
            File::open(file_path).context("Error opening input file")?,
        ))
    } else {
        BufReader::new(Box::new(std::io::stdin()))
    })
}

pub fn day1_parser(file: Option<PathBuf>) -> Result<(Vec<u32>, Vec<u32>)> {
    let mut list1 = vec![];
    let mut list2 = vec![];

    for line in get_reader(file)?.lines() {
        let vals = line?
            .split_ascii_whitespace()
            .map(str::parse::<u32>)
            .collect::<Result<Vec<_>, _>>()?;
        let [val1, val2] = vals
            .try_into()
            .map_err(|_| anyhow!("Row must have only two values"))?;
        list1.push(val1);
        list2.push(val2);
    }
    Ok((list1, list2))
}

pub fn day2_parser(file: Option<PathBuf>) -> Result<Vec<Vec<i8>>> {
    let mut reports = vec![];

    for line in get_reader(file)?.lines() {
        let report = line?
            .split_ascii_whitespace()
            .map(str::parse::<i8>)
            .collect::<Result<Vec<_>, _>>()?;
        reports.push(report);
    }
    Ok(reports)
}

pub fn day3_parser(file: Option<PathBuf>) -> Result<Vec<u8>> {
    let lines = get_reader(file)?
        .lines()
        .map(|x| x.map_err(|_| anyhow!("Failed to read line")))
        .collect::<Result<Vec<_>>>()?;
    Ok(lines.into_iter().flat_map(|x| x.into_bytes()).collect())
}

pub fn day4_parser(file: Option<PathBuf>) -> Result<Vec<Vec<u8>>> {
    let lines = get_reader(file)?
        .lines()
        .map(|x| x.map_err(|_| anyhow!("Failed to read line")))
        .collect::<Result<Vec<_>>>()?;
    Ok(lines.into_iter().map(|x| x.into_bytes()).collect())
}

pub fn day5_parser(file: Option<PathBuf>) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let mut lines_iter = get_reader(file).unwrap().lines().map(|x| x.unwrap());
    let rules = lines_iter
        .by_ref()
        .take_while(|line| !line.trim().is_empty())
        .map(|line| {
            (
                line[0..2].parse::<usize>().unwrap(),
                line[3..5].parse::<usize>().unwrap(),
            )
        })
        .collect();
    let updates = lines_iter
        .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();
    (rules, updates)
}

pub fn day6_parser(file: Option<PathBuf>) -> (Vec<Vec<LabEntry>>, i32, i32) {
    let mut guard_i = 0;
    let mut guard_j = 0;
    let grid = get_reader(file)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(i, x)| {
            x.unwrap()
                .as_bytes()
                .iter()
                .enumerate()
                .map(|(j, &x)| match x {
                    b'.' => LabEntry::Vacant,
                    b'#' => LabEntry::Obstacle {
                        hits: [false; 4],
                        round_updated: 0,
                    },
                    b'^' => {
                        guard_i = i;
                        guard_j = j;
                        LabEntry::Starting
                    }
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    (grid, guard_i as i32, guard_j as i32)
}

pub fn day7_parser(file: Option<PathBuf>) -> Vec<(u64, Vec<u64>)> {
    get_reader(file)
        .unwrap()
        .lines()
        .map(|x| {
            let line = x.unwrap();
            let [total, rest]: [String; 2] = line
                .split(": ")
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let total: u64 = total.parse().unwrap();
            let eq = rest.split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
            (total, eq)
        })
        .collect()
}

#[allow(clippy::type_complexity)]
pub fn day8_parser(file: Option<PathBuf>) -> (HashMap<u8, Vec<(i32, i32)>>, i32, i32) {
    let mut antennas: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
    let mut rows = 0;
    let mut cols = 0;
    for (i, line) in get_reader(file)
        .unwrap()
        .lines()
        .map(|x| x.unwrap().into_bytes())
        .enumerate()
    {
        rows = i + 1;
        cols = line.len();
        for (j, &frequency) in line.iter().enumerate() {
            if frequency != b'.' {
                antennas
                    .entry(frequency)
                    .or_default()
                    .push((i as i32, j as i32))
            }
        }
    }
    (antennas, rows as i32, cols as i32)
}

pub fn day9_parser(file: Option<PathBuf>) -> Vec<u8> {
    get_reader(file)
        .unwrap()
        .bytes()
        .filter_map(|x| (x.unwrap() as char).to_digit(10).map(|x| x as u8))
        .collect::<Vec<_>>()
}

pub fn day10_parser(file: Option<PathBuf>) -> Vec<Vec<u8>> {
    let lines = get_reader(file)
        .unwrap()
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();
    lines
        .into_iter()
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap() as u8).collect())
        .collect()
}
