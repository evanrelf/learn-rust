use std::collections::HashMap;
// use std::io::Read;

fn mean(numbers: &Vec<i32>) -> Option<f32> {
    if numbers.is_empty() {
        return None;
    }
    let sum = numbers.iter().sum::<i32>() as f32;
    let len = numbers.len() as f32;
    Some(sum / len)
}

fn median(numbers: &Vec<i32>) -> Option<i32> {
    if numbers.is_empty() {
        return None;
    }

    let len = numbers.len();
    let mut numbers = numbers.clone();
    numbers.sort();

    if len % 2 == 0 {
        // Even number of elements
        let left_index = ((len as f32 / 2.0) - 1.0) as usize;
        let right_index = (len as f32 / 2.0) as usize;
        let left = numbers[left_index] as f32;
        let right = numbers[right_index] as f32;
        let middle = (left + right / 2.0) as i32;
        Some(middle)
    } else {
        // Odd number of elements
        let middle_index = (len as f32 / 2.0).floor() as usize;
        let middle = numbers[middle_index];
        Some(middle)
    }
}

fn mode(numbers: &Vec<i32>) -> Option<i32> {
    let mut counts: HashMap<i32, i32> = HashMap::new();

    for number in numbers {
        counts
            .entry(*number)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    counts
        .iter()
        .max_by_key(|(_, count)| *count)
        .map(|(number, _)| *number)
}

// fn pig_latin(input: &str) -> String {
//     if input.is_empty() {
//         return String::new();
//     }

//     let head = todo!();
//     let tail = todo!();

//     if "aeiou".contains(head) {
//         input.to_owned() + "-hay"
//     } else {
//         tail + "-" + head + "ay"
//     }
// }

// fn employee_manager() {
//     let mut employees: HashMap<String, String> = HashMap::new();

//     loop {
//         let mut input = String::new();

//         std::io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read stdin")

//         input = input.trim();
//     }
// }

fn main() {
    let numbers = vec![9, 2, 5, 6, 5, 6, 7, 6, 10];
    println!("Numbers: {:?}", numbers);
    println!("Mean:    {:?}", mean(&numbers));
    println!("Median:  {:?}", median(&numbers));
    println!("Mode:    {:?}", mode(&numbers));

    // let words = vec!["first", "apple"];
    // println!("Words:     {:?}", words);
    // println!("Pig latin: {:?}", words.iter().map(|word| pig_latin(word)));

    // employee_manager();
}
