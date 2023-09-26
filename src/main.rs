#![allow(unused)]
use rayon::prelude::*;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.age.partial_cmp(&other.age)
    }
}

fn quick_sort_par<T: PartialOrd + Send>(data: &mut [T]) {
    if data.len() <= 1 {
        return;
    }

    let pivot_index = partition(data);

    if pivot_index > 0 {
        let (left, right) = data.split_at_mut(pivot_index);
        rayon::join(|| quick_sort_par(left), || quick_sort_par(&mut right[1..]));
    } else {
        quick_sort_par(&mut data[1..]);
    }
}

fn partition<T: PartialOrd>(data: &mut [T]) -> usize {
    if data.is_empty() {
        return 0;
    }

    let pivot_index = data.len() / 2;
    data.swap(pivot_index, data.len() - 1);

    let mut i = 0;
    for j in 0..data.len() - 1 {
        if data[j] <= data[data.len() - 1] {
            data.swap(i, j);
            i += 1;
        }
    }

    data.swap(i, data.len() - 1);
    i
}

fn main() {
    let mut data = vec![5, 2, 9, 1, 5, 6];
    quick_sort_par(&mut data);
    println!("{:?}", data); // should print [1, 2, 5, 5, 6, 9]

    let mut people = vec![
        Person { name: String::from("Alice"), age: 30 },
        Person { name: String::from("Bob"), age: 20 },
        Person { name: String::from("Eve"), age: 25 },
    ];

    quick_sort_par(&mut people);

    for person in &people {
        println!("{:?}", person); // should print Bob, Eve, Alice
    }
}
