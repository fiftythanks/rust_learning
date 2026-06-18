use std::collections::HashMap;

/// Given a list of integers, returns the mode.
fn get_median(list: &[i32]) -> Option<i32> {
    if list.is_empty() {
        return None;
    }

    let mut list_copy = list.to_vec();
    list_copy.sort_unstable();
    let middle_i = list_copy.len() / 2;
    Some(list_copy[middle_i])
}

/// Given a list of integers, returns the mode.
fn get_mode(list: &Vec<i32>) -> Option<i32> {
    if list.is_empty() {
        return None;
    }

    let mut map = HashMap::new();

    for int in list {
        let count = map.entry(*int).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

    let mut mode: Option<i32> = None;

    for (k, v) in map.iter() {
        match mode {
            None => {
                if *v > 1 {
                    mode = Some(*k);
                }
            }
            Some(val) => {
                if *v > map[&val] {
                    mode = Some(*k);
                }
            }
        }
    }

    mode
}

fn main() {
    let vec = vec![1, 3, 2, 1, 1, 3, 2, 2];
    println!(
        "Vector: {vec:?}\nMedian: {:?}\nMode: {:?}",
        get_median(&vec),
        get_mode(&vec)
    );
}
