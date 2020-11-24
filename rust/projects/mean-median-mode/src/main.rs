use std::collections::HashMap;

fn main() {
    let mut number_vector = vec![4, 783, 31, 4, 5, 25];
    let mean = mean(&number_vector);
    let median = median(&mut number_vector);
    let mode = mode(&number_vector);
    println!("mean is {}", mean);
    println!("median is {}", median);
    println!("mode is {}", mode);
}

fn mean(vec: &Vec<usize>) -> usize {
    let total = vec.iter().fold(0, |sum, x| sum + x);
    total / vec.len()
}

fn median(vec: &mut Vec<usize>) -> usize {
    vec.sort();
    println!("{:?}", vec);
    let length = vec.len();
    let half_vec = ((length / 2) as f32).floor() as usize;
    if length % 2 == 0 {
        (vec[half_vec] + vec[half_vec + 1]) / 2
    } else {
        vec[half_vec]
    }
}

fn mode(vec: &Vec<usize>) -> usize {
    let mut mode_map = HashMap::new();
    for val in vec {
        let count = mode_map.entry(val).or_insert(0);
        *count += 1;
    }
    let mut max_val = 0;
    let mut max_key = 0;

    for (k, v) in mode_map.iter() {
        if *v > max_val {
            max_key = **k;
            max_val = *v;
        }
    }
    max_key
}
