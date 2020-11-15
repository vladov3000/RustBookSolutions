use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Stats {
    mean: f32,
    median: i32,
    mode: i32,
}

pub fn get_stats(arr: &mut [i32]) -> Option<Stats> {
    if arr.is_empty() {
        return None;
    }

    let mut sum: f32 = 0.0;
    let mut counts = HashMap::new();
    let mut max_count = -1;
    let mut mode = -1;

    for i in arr.iter() {
        sum += *i as f32;
        let count = counts.entry(*i).or_insert(1);
        *count += 1;
        if mode == -1 || *count > max_count {
            max_count = *count;
            mode = *i;
        }
    }
    let mean = sum / arr.len() as f32;

    arr.sort();
    let mut median = arr[arr.len() / 2];
    if arr.len() % 2 == 0 {
        median += arr[arr.len() / 2 - 1];
        median /= 2;
    }

    let res = Stats { mean: mean, median: median, mode: mode };
    Some(res)
}
