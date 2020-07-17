use std::collections::HashMap;
// use std::io::Read;

fn mean(numbers: &[i32]) -> Option<f32> {
    if numbers.is_empty() {
        return None;
    }

    let sum = numbers.iter().sum::<i32>() as f32;
    let length = numbers.len() as f32;
    Some(sum / length)
}

fn median(numbers: &[i32]) -> Option<i32> {
    if numbers.is_empty() {
        return None;
    }

    let length = numbers.len();
    let sorted_numbers = {
        let mut vec = numbers.to_vec();
        vec.sort();
        vec
    };

    if length % 2 == 0 {
        // Even number of elements
        let left_index: usize = ((length as f32 / 2.0) - 1.0) as usize;
        let right_index = (length as f32 / 2.0) as usize;
        let left = sorted_numbers[left_index] as f32;
        let right = sorted_numbers[right_index] as f32;
        let middle = (left + right / 2.0) as i32;
        Some(middle)
    } else {
        // Odd number of elements
        let middle_index = (length as f32 / 2.0).floor() as usize;
        let middle = sorted_numbers[middle_index];
        Some(middle)
    }
}

fn mode(numbers: &[i32]) -> Option<i32> {
    let mut counts: HashMap<i32, i32> = HashMap::new();

    for &number in numbers {
        counts
            .entry(number)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    counts
        .iter()
        .max_by_key(|(_, &count)| count)
        .map(|(&number, _)| number)
}

// fn pig_latin(input: &str) -> String {
// }

// fn employee_manager() {
// }

fn main() {
    let numbers = [9, 2, 5, 6, 5, 6, 7, 6, 10];
    println!("Numbers: {:?}", numbers);
    println!("Mean:    {:?}", mean(&numbers));
    println!("Median:  {:?}", median(&numbers));
    println!("Mode:    {:?}", mode(&numbers));

    // let words = ["first", "apple"];
    // println!("Words:     {:?}", words);
    // println!("Pig latin: {:?}", words.iter().map(|word| pig_latin(word)));

    // employee_manager();
}
