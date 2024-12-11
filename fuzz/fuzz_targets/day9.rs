#![no_main]
#![feature(let_chains)]

use advent_of_code::day9;
use libfuzzer_sys::fuzz_target;

pub fn expand(layout: Vec<u8>) -> Vec<Option<u64>> {
    let mut out = vec![];
    let mut id = 0;
    let mut to_write = Some(id);
    for size in layout {
        out.extend_from_slice(vec![to_write; size as usize].as_ref());
        to_write = match to_write {
            Some(_) => None,
            None => {
                id += 1;
                Some(id)
            }
        }
    }
    out
}

pub fn compress(mut layout: Vec<Option<u64>>) -> Vec<Option<u64>> {
    let Some(last) = layout.last() else {
        return vec![];
    };

    for id in (0..=last.unwrap()).rev() {
        let start = layout.iter().position(|x| *x == Some(id)).unwrap();
        let end = layout.iter().rposition(|x| *x == Some(id)).unwrap();
        let len = end + 1 - start;

        if let Some(new_start) = layout.windows(len).position(|x| x == vec![None; len])
            && new_start < start
        {
            layout[new_start..new_start + len].copy_from_slice(&vec![Some(id); len]);
            layout[start..end + 1].copy_from_slice(&vec![None; len]);
        }
    }

    layout
}

pub fn checksum(layout: Vec<Option<u64>>) -> u64 {
    layout
        .into_iter()
        .enumerate()
        .filter_map(|(pos, id)| id.map(|id| pos as u64 * id))
        .sum()
}

fuzz_target!(|data: Vec<u8>| {
    if data.len() % 2 == 0 {
        return;
    }
    if data.iter().step_by(2).any(|x| *x == 0) {
        return;
    }
    if *data.iter().max().unwrap() >= 10 {
        return;
    }
    let actual = day9(data.clone());
    let expanded = expand(data);
    let compressed = compress(expanded);
    let checksum = checksum(compressed);
    assert_eq!(actual.1, checksum);
});
