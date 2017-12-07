extern crate rand;

use rand::Rng;
use std::collections::HashMap;

fn main() {
  let mut list: Vec<i32> = Vec::new();
  for _i in 0..101 {
    let num = rand::thread_rng().gen_range(1, 50);
    list.push(num);
  }

  vector_stats(&mut list);
}

fn vector_stats(vec: &mut Vec<i32>) {
  let count = vec.iter().count();
  let mid = count / 2;

  vec.sort();
  println!("median {}", &vec[mid - 1]);

  let mut map = HashMap::new();
  for v in vec.iter() {
    let vcount = map.entry(v).or_insert(0);
    *vcount += 1;
  }

  println!("Max: {:?}" , map.iter().fold((0, 0), |acc, (k, v)| {
    if *v > acc.1 {
      return (**k, *v);
    } else {
      return acc;
    }
  }));
}