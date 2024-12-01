fn shared() -> [Vec<i32>; 2] {
    let input = std::fs::read_to_string("inputs/day1.txt").unwrap();
    let mut left_vec: Vec<i32> = vec![];
    let mut right_vec: Vec<i32> = vec![];

    input.lines().for_each(|line| {
        let nums = line.split("   ");
        left_vec.push(nums.clone().nth(0).unwrap().parse::<i32>().unwrap());
        right_vec.push(nums.clone().nth(1).unwrap().parse::<i32>().unwrap());
    });

    return [ left_vec, right_vec ];
}

fn part_1() {
    let [mut left_vec, mut right_vec] = shared();
    let mut differences: Vec<i32> = vec![];
    left_vec.sort(); 
    right_vec.sort();

    for i in 0..left_vec.len() {
        let difference = left_vec[i] - right_vec[i];
        if difference < 0 {
            differences.push(difference * -1);
        } else {
            differences.push(difference);
        }
    }
    let total_difference: i32 = differences.iter().sum();
    println!("P1: Total Difference: {:?}", total_difference);
}

fn part_2() {
    let [left_vec, right_vec] = shared();
    let mut similarity = 0;
    for i in 0..left_vec.len() {
        let count = right_vec.iter().filter(|x| **x == left_vec[i]).count();
        let additional_similarity = count * (left_vec[i] as usize);
        similarity += additional_similarity;
    }
    println!("P2: Similarity: {:?}", similarity);
}

fn main() {
    part_1();
    part_2();
}