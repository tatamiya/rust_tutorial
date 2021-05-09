use std::collections::HashMap;

fn main() {
    // Given a list of integers, use a vector
    // and return the mean (the average value),
    // median (when sorted, the value in the middle position),
    // and mode (the value that occurs most often;
    // a hash map will be helpful here) of the list.

    let v = vec![3, 4, 8, 8, 1];
    println!("input: {:?}", v);

    // mean
    let mut sum: i32 = 0;
    for i in &v {
        sum += i;
    }
    let mean: f32 = sum as f32 / v.len() as f32;
    println!("mean: {}", mean);

    // median
    let mut v_sort = v.clone();
    v_sort.sort();
    let median: f32;
    let length = v_sort.len();
    let middle_index = length / 2;
    if length % 2 == 0 {
        median = (v_sort[middle_index - 1] + v_sort[middle_index]) as f32 / 2.;
    } else {
        median = v_sort[middle_index] as f32;
    }
    println!("median: {}", median);

    // mode
    let mut mode: i32 = 0;
    let mut max_count: i32 = 0;
    let mut map = HashMap::new();
    for i in &v {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    for (num, count) in &map {
        if count > &max_count {
            mode = **num;
            max_count = *count;
        }
    }
    println!("mode: {}", mode);
}
