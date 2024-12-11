#![feature(array_windows, let_chains, array_chunks)]

use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap, HashSet},
};

pub mod errors;
pub mod parsers;

pub fn day1(mut list1: Vec<u32>, mut list2: Vec<u32>) -> (u64, u64) {
    list1.sort();
    list2.sort();

    let difference = list1
        .iter()
        .zip(&list2)
        .map(|(x, y)| x.abs_diff(*y) as u64)
        .sum();

    let mut score = 0;

    let mut list1_iter = list1.iter();
    let mut list2_iter = list2.iter();

    let list1_entry = list1_iter.next();
    let list2_entry = list2_iter.next();

    let Some(mut list1_val) = list1_entry else {
        return (difference, score);
    };
    let Some(mut list2_val) = list2_entry else {
        return (difference, score);
    };

    loop {
        match list1_val.cmp(list2_val) {
            Ordering::Less => {
                if let Some(next_list1_val) = list1_iter.next() {
                    list1_val = next_list1_val;
                } else {
                    return (difference, score);
                }
            }
            Ordering::Equal => {
                let mut list1_counter = 1;
                let mut list2_counter = 1;
                let value = list1_val;
                let list1_entry = 'a: {
                    for list1_val in list1_iter.by_ref() {
                        if list1_val == value {
                            list1_counter += 1;
                        } else {
                            break 'a Some(list1_val);
                        }
                    }
                    None
                };
                let list2_entry = 'a: {
                    for list2_val in list2_iter.by_ref() {
                        if list2_val == value {
                            list2_counter += 1;
                        } else {
                            break 'a Some(list2_val);
                        }
                    }
                    None
                };
                score += (list1_counter * list2_counter * value) as u64;
                if let Some(next_list1_val) = list1_entry
                    && let Some(next_list2_val) = list2_entry
                {
                    list1_val = next_list1_val;
                    list2_val = next_list2_val;
                } else {
                    return (difference, score);
                }
            }
            Ordering::Greater => {
                if let Some(next_list2_val) = list2_iter.next() {
                    list2_val = next_list2_val;
                } else {
                    return (difference, score);
                }
            }
        }
    }
}

pub fn valid_report_diff(diff: i8) -> bool {
    (1..=3).contains(&diff)
}

pub enum ReportResult {
    NoMismatch,       // (val --> next_val is safe)
    OneMismatch,      // A mismatch occurred, but no other mismatches occur in the window
    MultipleMismatch, // An unresolvable mismatch occurred
}

// Assumes (prev_val --> val) is safe
pub fn process_padded_window_increasing(
    prev_val: i8,
    val: i8,
    next_val: i8,
    next_next_val: i8,
) -> ReportResult {
    if !valid_report_diff(next_val - val) {
        if valid_report_diff(next_next_val - val)
            || (valid_report_diff(next_val - prev_val)
                && valid_report_diff(next_next_val - next_val))
        {
            ReportResult::OneMismatch
        } else {
            ReportResult::MultipleMismatch
        }
    } else {
        ReportResult::NoMismatch
    }
}

// Assumes (prev_val --> val) is safe
pub fn process_padded_window_decreasing(
    prev_val: i8,
    val: i8,
    next_val: i8,
    next_next_val: i8,
) -> ReportResult {
    if !valid_report_diff(val - next_val) {
        if valid_report_diff(val - next_next_val)
            || (valid_report_diff(prev_val - next_val)
                && valid_report_diff(next_val - next_next_val))
        {
            ReportResult::OneMismatch
        } else {
            ReportResult::MultipleMismatch
        }
    } else {
        ReportResult::NoMismatch
    }
}

pub fn check_report_safety_increasing(report: &[i8]) -> bool {
    for [val, next_val] in report.array_windows() {
        if !valid_report_diff(next_val - val) {
            return false;
        }
    }
    true
}

pub fn check_report_safety_decreasing(report: &[i8]) -> bool {
    for [val, next_val] in report.array_windows() {
        if !valid_report_diff(val - next_val) {
            return false;
        }
    }
    true
}

// Assumes report.len() >= 4
pub fn check_report_safety_increasing_one_mismatch(report: &[i8]) -> bool {
    if !valid_report_diff(report[1] - report[0]) {
        if valid_report_diff(report[2] - report[1]) || valid_report_diff(report[2] - report[0]) {
            return check_report_safety_increasing(&report[2..]);
        } else {
            return false;
        }
    }
    for (i, [prev_val, val, next_val, next_next_val]) in report.array_windows().enumerate() {
        match process_padded_window_increasing(*prev_val, *val, *next_val, *next_next_val) {
            ReportResult::NoMismatch => continue,
            ReportResult::OneMismatch => return check_report_safety_increasing(&report[i + 3..]),
            ReportResult::MultipleMismatch => return false,
        }
    }
    true
}

pub fn check_report_safety_decreasing_one_mismatch(report: &[i8]) -> bool {
    if !valid_report_diff(report[0] - report[1]) {
        if valid_report_diff(report[1] - report[2]) || valid_report_diff(report[0] - report[2]) {
            return check_report_safety_decreasing(&report[2..]);
        } else {
            return false;
        }
    }
    for (i, [prev_val, val, next_val, next_next_val]) in report.array_windows().enumerate() {
        match process_padded_window_decreasing(*prev_val, *val, *next_val, *next_next_val) {
            ReportResult::NoMismatch => continue,
            ReportResult::OneMismatch => return check_report_safety_decreasing(&report[i + 3..]),
            ReportResult::MultipleMismatch => return false,
        }
    }
    true
}

pub fn check_report_safety(report: &[i8]) -> bool {
    if report.len() < 2 {
        return true;
    }
    if report[0] < report[1] {
        check_report_safety_increasing(report)
    } else {
        check_report_safety_decreasing(report)
    }
}

pub fn check_report_safety_one_mismatch(report: &[i8]) -> bool {
    match report.len() {
        0..=2 => true,
        3 => valid_report_diff(report[0] - report[1]) || valid_report_diff(report[1] - report[2]),
        _ => {
            let num_positive = report[0..4]
                .array_windows::<2>()
                .filter(|[x, y]| x < y)
                .count();
            if num_positive < 2 {
                check_report_safety_decreasing_one_mismatch(report)
            } else {
                check_report_safety_increasing_one_mismatch(report)
            }
        }
    }
}

pub fn day2(reports: Vec<Vec<i8>>) -> (usize, usize) {
    let count_no_mismatch = reports.iter().filter(|x| check_report_safety(x)).count();
    let count_one_mismatch = reports
        .iter()
        .filter(|x| check_report_safety_one_mismatch(x))
        .count();
    (count_no_mismatch, count_one_mismatch)
}

//          11  111  1111
// 12345678901  234  5678
// mul(###,###) do() n't()

pub fn day3(input: &[u8], conditionals_on: bool) -> u32 {
    let mut sum = 0;
    let mut val1 = 0;
    let mut val2 = 0;
    let mut flag_on = true;

    let mut state = 0;
    for &char in input {
        state = match (state, char, flag_on) {
            (_, b'm', true) => 1,
            (_, b'd', _) => 12,
            (1, b'u', true) => 2,
            (2, b'l', true) => 3,
            (3, b'(', true) => 4,
            (4, c, true) => {
                if let Some(d) = (c as char).to_digit(10) {
                    val1 = d;
                    5
                } else {
                    0
                }
            }
            (5..=7, b',', true) => 8,
            (s @ 5..=6, c, true) => {
                if let Some(d) = (c as char).to_digit(10) {
                    val1 = val1 * 10 + d;
                    s + 1
                } else {
                    0
                }
            }
            (8, c, true) => {
                if let Some(d) = (c as char).to_digit(10) {
                    val2 = d;
                    9
                } else {
                    0
                }
            }
            (9..=11, b')', true) => {
                sum += val1 * val2;
                0
            }
            (s @ 9..=10, c, true) => {
                if let Some(d) = (c as char).to_digit(10) {
                    val2 = val2 * 10 + d;
                    s + 1
                } else {
                    0
                }
            }
            (12, b'o', _) => 13,
            (13, b'(', _) => 14,
            (14, b')', _) => {
                flag_on = true;
                0
            }
            (13, b'n', _) => 15,
            (15, b'\'', _) => 16,
            (16, b't', _) => 17,
            (17, b'(', _) => 18,
            (18, b')', _) => {
                flag_on = false | !conditionals_on;
                0
            }
            _ => 0,
        };
    }

    sum
}

const NEIGHBOR_OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

/// Return whether XMAS is present in grid, starting at (i, j) and in direction
/// (offset_i, offset_j). Assumes we have already verified grid[i][j] = b'X' and
/// that grid is padded.
fn check_for_word(grid: &[Vec<u8>], i: usize, j: usize, offset_i: i32, offset_j: i32) -> bool {
    let mut i = i as i32 + offset_i;
    let mut j = j as i32 + offset_j;
    for c in b"MAS" {
        if grid[i as usize][j as usize] == *c {
            i += offset_i;
            j += offset_j;
        } else {
            return false;
        }
    }
    true
}

/// Return the number of times XMAS appears in the grid starting at (i, j).
/// Assumes that grid is padded.
fn count_words_at_spot(grid: &[Vec<u8>], i: usize, j: usize) -> u64 {
    let mut count = 0;
    if grid[i][j] == b'X' {
        for (offset_i, offset_j) in NEIGHBOR_OFFSETS {
            if check_for_word(grid, i, j, offset_i, offset_j) {
                count += 1;
            }
        }
    }
    count
}

/// Assumes that grid is padded.
fn count_words(grid: &[Vec<u8>]) -> u64 {
    let mut count = 0;
    let rows = grid.len();
    let cols = grid[0].len();
    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            count += count_words_at_spot(grid, i, j);
        }
    }
    count
}

/// Assumes that grid is padded.
fn check_for_cross(grid: &[Vec<u8>], i: usize, j: usize) -> bool {
    grid[i][j] == b'A'
        && (grid[i - 1][j - 1] == b'M' && grid[i + 1][j + 1] == b'S'
            || grid[i - 1][j - 1] == b'S' && grid[i + 1][j + 1] == b'M')
        && (grid[i - 1][j + 1] == b'M' && grid[i + 1][j - 1] == b'S'
            || grid[i - 1][j + 1] == b'S' && grid[i + 1][j - 1] == b'M')
}

/// Assumes that grid is padded.
fn count_crosses(grid: &[Vec<u8>]) -> u64 {
    let mut count = 0;
    let rows = grid.len();
    let cols = grid[0].len();
    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            if check_for_cross(grid, i, j) {
                count += 1;
            }
        }
    }
    count
}

pub fn day4(mut grid: Vec<Vec<u8>>) -> (u64, u64) {
    let cols = grid[0].len();
    for row in grid.iter_mut() {
        row.splice(0..0, [b'.']);
        row.extend_from_slice(b".");
    }
    grid.splice(0..0, [vec![b'.'; cols + 2]]);
    grid.extend([vec![b'.'; cols + 2]]);

    (count_words(&grid), count_crosses(&grid))
}

struct PageSorter<'a> {
    rules_matrix: &'a [[bool; 100]; 100],
    start_pages: BTreeSet<usize>,
    pages_remaining: HashSet<usize>,
}

impl<'a> PageSorter<'a> {
    fn new(rules_matrix: &'a [[bool; 100]; 100], update: &[usize]) -> Self {
        Self {
            rules_matrix,
            start_pages: BTreeSet::new(),
            pages_remaining: HashSet::from_iter(update.iter().copied()),
        }
    }
}

impl Iterator for PageSorter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(page) = self.start_pages.pop_first() {
            self.pages_remaining.remove(&page);
            Some(page)
        } else {
            let pages = self.pages_remaining.iter().copied().collect::<Vec<_>>();
            for page in &pages {
                let is_start_node = pages.iter().all(|from| !self.rules_matrix[*from][*page]);
                if is_start_node {
                    self.pages_remaining.remove(page);
                    self.start_pages.insert(*page);
                }
            }
            self.start_pages.pop_first()
        }
    }
}

pub fn day5(rules: Vec<(usize, usize)>, updates: Vec<Vec<usize>>) -> (usize, usize) {
    let mut correct_middle = 0;
    let mut sorted_middle = 0;
    let mut rules_matrix = [[false; 100]; 100];
    for (from, to) in rules {
        rules_matrix[from][to] = true;
    }
    'a: for update in updates {
        for (i, from) in update.iter().enumerate() {
            for to in update.iter().skip(i + 1) {
                // Check if it violates the rule
                if rules_matrix[*to][*from] {
                    sorted_middle += PageSorter::new(&rules_matrix, &update)
                        .nth(update.len() / 2)
                        .unwrap();

                    continue 'a;
                }
            }
        }
        correct_middle += update[update.len() / 2];
    }

    (correct_middle, sorted_middle)
}

#[derive(Clone)]
pub enum LabEntry {
    OutOfBounds,
    Vacant,
    Starting,
    Visited,
    Obstacle {
        hits: [bool; 4],
        round_updated: usize,
    },
}

#[allow(clippy::too_many_arguments)]
pub fn simulate_obstacle(
    grid: &mut [Vec<LabEntry>],
    mut i: i32,
    mut j: i32,
    mut vi: i32,
    mut vj: i32,
    mut dir: usize,
    simul_count: &mut usize,
) -> bool {
    let obj_i = (i + vi) as usize;
    let obj_j = (j + vj) as usize;
    let original = std::mem::replace(&mut grid[obj_i][obj_j], LabEntry::Obstacle {
        hits: [false; 4],
        round_updated: *simul_count,
    });

    let out = loop {
        match grid[(i + vi) as usize][(j + vj) as usize] {
            LabEntry::OutOfBounds => {
                break false;
            }
            LabEntry::Vacant | LabEntry::Visited | LabEntry::Starting => {
                i += vi;
                j += vj;
            }
            LabEntry::Obstacle {
                ref mut hits,
                ref mut round_updated,
            } => {
                if round_updated == simul_count {
                    if std::mem::replace(&mut hits[dir], true) {
                        break true;
                    }
                } else {
                    *hits = [false; 4];
                    hits[dir] = true;
                    *round_updated = *simul_count;
                }

                (vi, vj) = (vj, -vi);
                dir = (dir + 1) % 4;
            }
        }
    };
    grid[obj_i][obj_j] = original;
    *simul_count += 1;
    out
}

pub fn day6(mut grid: Vec<Vec<LabEntry>>, mut i: i32, mut j: i32) -> (u64, u64) {
    let cols = grid[0].len();
    for row in grid.iter_mut() {
        row.splice(0..0, [LabEntry::OutOfBounds]);
        row.extend_from_slice(&[LabEntry::OutOfBounds]);
    }
    grid.splice(0..0, [vec![LabEntry::OutOfBounds; cols + 2]]);
    grid.extend([vec![LabEntry::OutOfBounds; cols + 2]]);
    i += 1;
    j += 1;

    let mut visited_counter = 1;
    let mut cycle_counter = 0;
    let mut vi = -1;
    let mut vj = 0;
    let mut dir = 0;
    let mut simul_count = 0;
    loop {
        match grid[(i + vi) as usize][(j + vj) as usize] {
            LabEntry::OutOfBounds => break,
            LabEntry::Vacant => {
                if simulate_obstacle(&mut grid, i, j, vi, vj, dir, &mut simul_count) {
                    cycle_counter += 1;
                }
                i += vi;
                j += vj;
                grid[i as usize][j as usize] = LabEntry::Visited;
                visited_counter += 1;
            }
            LabEntry::Visited | LabEntry::Starting => {
                i += vi;
                j += vj;
            }
            LabEntry::Obstacle { .. } => {
                (vi, vj) = (vj, -vi);
                dir = (dir + 1) % 4;
            }
        }
    }
    (visited_counter, cycle_counter)
}

#[derive(PartialEq, Eq)]
enum EqSolvable {
    No,
    WithArith,
    WithConcat,
}

pub fn exact_quot(x: u64, y: u64) -> Option<u64> {
    if y == 0 {
        None
    } else if x % y == 0 {
        Some(x / y)
    } else {
        None
    }
}

pub fn checked_trunc(x: u64, y: u64) -> Option<u64> {
    let y_string = y.to_string();
    if x.to_string().ends_with(&y_string) {
        Some((x - y) / 10u64.pow(y_string.len() as u32))
    } else {
        None
    }
}

fn is_solvable(eq: &[u64], current_index: usize, total: u64) -> EqSolvable {
    if current_index == 0 {
        if eq[0] == total {
            EqSolvable::WithArith
        } else {
            EqSolvable::No
        }
    } else if let Some(quot) = exact_quot(total, eq[current_index])
        && let eq_solvable = is_solvable(eq, current_index - 1, quot)
        && eq_solvable != EqSolvable::No
    {
        eq_solvable
    } else if let Some(diff) = total.checked_sub(eq[current_index]) {
        let diff_eq_solvable = is_solvable(eq, current_index - 1, diff);
        if diff_eq_solvable != EqSolvable::No {
            diff_eq_solvable
        } else if let Some(trunc) = checked_trunc(total, eq[current_index])
            && is_solvable(eq, current_index - 1, trunc) != EqSolvable::No
        {
            EqSolvable::WithConcat
        } else {
            EqSolvable::No
        }
    } else {
        EqSolvable::No
    }
}

pub fn day7(eqs: Vec<(u64, Vec<u64>)>) -> (u64, u64) {
    eqs.into_iter()
        .map(|(total, eq)| (total, is_solvable(&eq, eq.len() - 1, total)))
        .fold(
            (0, 0),
            |(with_arith, with_concat), (total, result)| match result {
                EqSolvable::No => (with_arith, with_concat),
                EqSolvable::WithArith => (with_arith + total, with_concat + total),
                EqSolvable::WithConcat => (with_arith, with_concat + total),
            },
        )
}

fn count_antinodes_no_resonance(
    antennas: &HashMap<u8, Vec<(i32, i32)>>,
    rows: i32,
    cols: i32,
) -> usize {
    let mut antinode_positions = HashSet::new();
    for antennas in antennas.values() {
        for (num, (i1, j1)) in antennas.iter().enumerate() {
            for (i2, j2) in &antennas[num + 1..] {
                let i_a1 = 2 * i1 - i2;
                let j_a1 = 2 * j1 - j2;
                let i_a2 = 2 * i2 - i1;
                let j_a2 = 2 * j2 - j1;
                if (0..rows).contains(&i_a1) && (0..cols).contains(&j_a1) {
                    antinode_positions.insert((i_a1, j_a1));
                }
                if (0..rows).contains(&i_a2) && (0..cols).contains(&j_a2) {
                    antinode_positions.insert((i_a2, j_a2));
                }
            }
        }
    }
    antinode_positions.len()
}

fn count_antinodes_with_resonance(
    antennas: &HashMap<u8, Vec<(i32, i32)>>,
    rows: i32,
    cols: i32,
) -> usize {
    let mut antinode_positions = HashSet::new();
    for antennas in antennas.values() {
        for (num, (i1, j1)) in antennas.iter().enumerate() {
            if antennas.len() > 1 {
                antinode_positions.insert((*i1, *j1));
            }
            for (i2, j2) in &antennas[num + 1..] {
                let mut vi = i2 - i1;
                let mut vj = j2 - j1;
                let mut new_i = i2 + vi;
                let mut new_j = j2 + vj;
                while (0..rows).contains(&new_i) && (0..cols).contains(&new_j) {
                    antinode_positions.insert((new_i, new_j));
                    new_i += vi;
                    new_j += vj;
                }
                vi *= -1;
                vj *= -1;
                new_i = i1 + vi;
                new_j = j1 + vj;
                while (0..rows).contains(&new_i) && (0..cols).contains(&new_j) {
                    antinode_positions.insert((new_i, new_j));
                    new_i += vi;
                    new_j += vj;
                }
            }
        }
    }
    antinode_positions.len()
}

pub fn day8(antennas: &HashMap<u8, Vec<(i32, i32)>>, rows: i32, cols: i32) -> (usize, usize) {
    (
        count_antinodes_no_resonance(antennas, rows, cols),
        count_antinodes_with_resonance(antennas, rows, cols),
    )
}

pub fn increase_checksum(checksum: u64, id: u64, pos: u64, num_files: u8) -> u64 {
    if num_files > 0 {
        checksum + id * (pos * num_files as u64 + (num_files * (num_files - 1) / 2) as u64)
    } else {
        checksum
    }
}

struct FilesTracker {
    files_by_len: [Vec<usize>; 10],
    pub processed: HashSet<u64>,
}

impl FilesTracker {
    fn new(layout: &[u8]) -> Self {
        let mut spaces: [Vec<usize>; 10] = vec![Vec::new(); 10].try_into().unwrap();
        for (id, file_len) in layout.iter().step_by(2).enumerate() {
            spaces[*file_len as usize].push(id);
        }
        FilesTracker {
            files_by_len: spaces,
            processed: HashSet::new(),
        }
    }

    fn pop_from_files_by_len(&mut self, max_size: u8) -> Option<(u8, u64)> {
        let out = self.files_by_len[0..=max_size as usize]
            .iter()
            .enumerate()
            .filter_map(|(size, ids)| ids.last().map(|&id| (id, size)))
            .max()
            .map(|(id, size)| (size as u8, id as u64));
        if let Some((size, _)) = out {
            self.files_by_len[size as usize].pop();
        }
        out
    }

    fn pop_file(&mut self, max_size: u8) -> Option<(u8, u64)> {
        loop {
            if let Some((size, id)) = self.pop_from_files_by_len(max_size) {
                if !self.processed.contains(&id) {
                    self.processed.insert(id);
                    return Some((size, id));
                }
            } else {
                return None;
            }
        }
    }
}

fn checksum_breaking(layout: &[u8]) -> u64 {
    let mut iter = layout.iter().copied();
    let mut id_front = 0;
    let mut id_back = ((iter.len() - 1) / 2) as u64;
    // This will not panic since layout has odd length, and hence len >= 1
    let mut num_files_back = iter.next_back().unwrap();
    let mut pos = 0;
    let mut checksum = 0;
    // The iterator now has even length, so we can process in pairs
    while let (Some(num_files_front), Some(mut num_free_front)) = (iter.next(), iter.next()) {
        // Process file block
        checksum = increase_checksum(checksum, id_front, pos, num_files_front);
        pos += num_files_front as u64;
        id_front += 1;

        // Process free block
        while num_free_front > num_files_back {
            num_free_front -= num_files_back;
            checksum = increase_checksum(checksum, id_back, pos, num_files_back);
            pos += num_files_back as u64;
            id_back -= 1;
            if iter.len() >= 2 {
                iter.next_back();
                num_files_back = iter.next_back().unwrap();
            } else {
                // Iterator is empty, num_files_back is exhausted
                return checksum;
            }
        }
        if num_free_front > 0 {
            checksum = increase_checksum(checksum, id_back, pos, num_free_front);
            pos += num_free_front as u64;
            num_files_back -= num_free_front;
        }
    }
    if num_files_back > 0 {
        checksum = increase_checksum(checksum, id_back, pos, num_files_back);
    }
    checksum
}

fn checksum_nonbreaking(layout: &[u8]) -> u64 {
    let mut tracker = FilesTracker::new(layout);
    let mut pos = 0;
    let mut id_front = 0;
    let mut checksum = 0;
    let mut in_file_block = true;
    for mut layout_num in layout.iter().copied() {
        if in_file_block {
            in_file_block = false;
            if !tracker.processed.contains(&(id_front as u64)) {
                tracker.processed.insert(id_front as u64);
                checksum = increase_checksum(checksum, id_front as u64, pos, layout_num);
            }
            pos += layout_num as u64;
            id_front += 1;
        } else {
            in_file_block = true;
            while layout_num > 0 {
                if let Some((num_files_back, id_back)) = tracker.pop_file(layout_num) {
                    checksum = increase_checksum(checksum, id_back, pos, num_files_back);
                    pos += num_files_back as u64;
                    layout_num -= num_files_back;
                } else {
                    pos += layout_num as u64;
                    break;
                }
            }
        }
    }
    checksum
}

pub fn day9(layout: Vec<u8>) -> (u64, u64) {
    assert!(layout.len() % 2 == 1);

    (checksum_breaking(&layout), checksum_nonbreaking(&layout))
}

pub fn get_score(
    terrain: &[Vec<u8>],
    visited: &mut [Vec<u64>],
    i: usize,
    j: usize,
    round: u64,
) -> u64 {
    if visited[i][j] == round {
        0
    } else if terrain[i][j] == 9 {
        visited[i][j] = round;
        1
    } else {
        let mut out = 0;
        let neighbors = [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)];
        for (ni, nj) in neighbors {
            if terrain[ni][nj] == terrain[i][j] + 1 {
                out += get_score(terrain, visited, ni, nj, round);
            }
        }
        visited[i][j] = round;
        out
    }
}

pub fn count_paths_up(
    terrain: &[Vec<u8>],
    num_paths_to_top: &mut [Vec<Option<u64>>],
    i: usize,
    j: usize,
) -> u64 {
    if let Some(paths) = num_paths_to_top[i][j] {
        paths
    } else if terrain[i][j] == 9 {
        num_paths_to_top[i][j] = Some(1);
        1
    } else {
        let mut paths = 0;
        let neighbors = [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)];
        for (ni, nj) in neighbors {
            if terrain[ni][nj] == terrain[i][j] + 1 {
                paths += count_paths_up(terrain, num_paths_to_top, ni, nj);
            }
        }
        num_paths_to_top[i][j] = Some(paths);
        paths
    }
}

#[allow(clippy::needless_range_loop)]
pub fn day10(mut terrain: Vec<Vec<u8>>) -> (u64, u64) {
    let cols = terrain[0].len();
    for row in terrain.iter_mut() {
        row.splice(0..0, [u8::MAX]);
        row.extend_from_slice(&[u8::MAX]);
    }
    terrain.splice(0..0, [vec![u8::MAX; cols + 2]]);
    terrain.extend([vec![u8::MAX; cols + 2]]);

    let rows = terrain.len();
    let cols = terrain[0].len();

    let mut round = 1;
    let mut visited = vec![vec![0; cols]; rows];

    let mut part1 = 0;
    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            if terrain[i][j] == 0 {
                part1 += get_score(&terrain, &mut visited, i, j, round);
                round += 1;
            }
        }
    }

    let mut num_paths_to_top = vec![vec![None; cols]; rows];

    num_paths_to_top[0] = vec![Some(0); cols];
    num_paths_to_top[rows - 1] = vec![Some(0); cols];
    for i in 1..rows - 1 {
        num_paths_to_top[i][0] = Some(0);
        num_paths_to_top[i][cols - 1] = Some(0);
    }

    let mut part2 = 0;
    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            if terrain[i][j] == 0 {
                part2 += count_paths_up(&terrain, &mut num_paths_to_top, i, j);
            }
        }
    }
    (part1, part2)
}
