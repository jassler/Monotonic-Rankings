#[macro_use]
pub mod setup;

use fxhash::FxHashMap;
use lazy_static::lazy_static;
use rug::Integer;
use std::env;
use std::sync::Mutex;
use std::time::Instant;

// struct CustomHasher {
//     hash: u64,
// }

// impl Hasher for CustomHasher {
//     fn finish(&self) -> u64 {
//         self.hash
//     }

//     fn write(&mut self, bytes: &[u8]) {
//         // Your custom hashing algorithm here
//         self.hash = 0;
//         for b in bytes {
//             if *b != 0 {
//                 self.hash = (self.hash << 8) | *b as u64;
//             }
//         }
//     }
// }

// struct CustomBuildHasher;

// impl BuildHasher for CustomBuildHasher {
//     type Hasher = CustomHasher;

//     fn build_hasher(&self) -> CustomHasher {
//         CustomHasher { hash: 0 }
//     }
// }

const N: usize = 6;
const LEN: usize = 1 << N;

// macro_rules! update_counter {
//     ($counter:expr, $coalition:expr, $amount:expr) => {
//         for p in 0..N {
//             if $coalition & (1 << p) != 0 {
//                 $counter[$coalition ^ (1 << p)] += $amount;
//             }
//         }
//     };
// }

lazy_static! {
    static ref MEMO: Mutex<FxHashMap<Vec<i32>, Integer>> = Mutex::new(FxHashMap::default());
}

fn update_counter_vector(counter: &mut Vec<i8>, coalition: usize, amount: i8) {
    counter[coalition] += amount;
    // let mut p = 1;
    // while p <= coalition {
    //     if coalition & p != 0 {
    //         counter[coalition ^ p] += amount;
    //     }
    //     p <<= 1;
    // }

    for p in 0..N {
        if coalition & (1 << p) != 0 {
            counter[coalition ^ (1 << p)] += amount;
        }
    }
    // for p in 0..N {
    //     counter[coalition ^ (1 << p)] += amount * ((coalition & (1 << p)) >> p) as i8;
    // }
}

fn loop_available_coalitions(
    counter: &mut Vec<i8>,
    memo: &mut FxHashMap<Vec<i8>, Integer>,
) -> Integer {
    // let memo = MEMO.lock().unwrap();
    // if let Some(result) = memo.get(counter) {
    //     return result.clone();
    // }
    // drop(memo);

    if let Some(result) = memo.get(counter) {
        return result.clone();
    }

    // let zeros = counter.iter().filter(|x| **x == 0 as i8).count();
    // if zeros == 1 {
    //     let index = counter.iter().position(|x| *x == 0 as i8).unwrap();
    //     return match count_bits!(index) {
    //         0 => "1",
    //         1 => "1",
    //         2 => "2",
    //         3 => "48",
    //         4 => "1680384",
    //         5 => "14807804035657359360",
    //         6 => "141377911697227887117195970316200795630205476957716480",
    //         _ => panic!("Unknown value"),
    //     }
    //     .parse()
    //     .unwrap();
    // }

    let mut sum = Integer::new();
    for i in 0..LEN {
        if counter[i] == 0 {
            update_counter_vector(counter, i, -1);
            sum += loop_available_coalitions(counter, memo);
            update_counter_vector(counter, i, 1);
        }
    }
    let result = std::cmp::max(Integer::from(1), sum);
    // let mut memo = MEMO.lock().unwrap();
    memo.insert(counter.clone(), result.clone());
    result
}

fn list_counters(amount: usize) {
    let mut counter = init_counter!(amount);
    let len = counter.len();
    println!("{:?}", counter);
    print!("Removing {: <10} -> ", coalition_to_string(len - 1));
    update_counter_vector(&mut counter, len - 1, -1);
    println!("{:?}", counter);
    print!("Removing {: <10} -> ", coalition_to_string(len - 2));
    update_counter_vector(&mut counter, len - 2, -1);
    println!("{:?}", counter);
    print!("Removing {: <10} -> ", coalition_to_string(len - 3));
    update_counter_vector(&mut counter, len - 3, -1);
    println!("{:?}", counter);

    println!("Branching!");
    print!("Removing {: <10} -> ", coalition_to_string(len - 4));
    update_counter_vector(&mut counter, len - 4, -1);
    println!("{:?}", counter);

    update_counter_vector(&mut counter, len - 4, 1);
    print!("Removing {: <10} -> ", coalition_to_string(len - 5));
    update_counter_vector(&mut counter, len - 5, -1);
    println!("{:?}", counter);
}

fn coalition_to_string(mut coalition: usize) -> String {
    let mut s = "{".to_string();
    let mut i = '1';
    while coalition > 0 {
        if (coalition & 1) == 1 {
            if s.len() > 1 {
                s.push(',');
            }
            s.push(i);
        }
        i = (i as u8 + 1) as char;
        coalition >>= 1;
    }
    s.push('}');
    s
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let def_n = N.to_string();
        match args[1].as_str() {
            "-c" => list_counters(args.get(2).unwrap_or(&def_n).parse::<usize>().unwrap()),
            "-l" => println!("Listing monotonic power relations"),
            _ => println!(
                "Usage: {} [flags]
                With no flags, counts number of monotonic power relations for {} elements

                -h      Help
                -c <N>  Display counter, optional for N elements
                -l <N>  List power relations, optional for N elements",
                args[0], N
            ),
        }
        return;
    }

    let mut memo = FxHashMap::default();
    let mut counter = init_counter!(N);

    println!("Initial counter for {} elements ({} coalitions)", N, LEN);
    println!("{:?}", counter);
    update_counter_vector(&mut counter, LEN - 1, -1);
    update_counter_vector(&mut counter, LEN - 2, -1);
    update_counter_vector(&mut counter, LEN - 3, -1);

    let now = Instant::now();
    update_counter_vector(&mut counter, LEN - 4, -1);
    update_counter_vector(&mut counter, LEN - 5, -1);
    let result = loop_available_coalitions(&mut counter, &mut memo);
    update_counter_vector(&mut counter, LEN - 4, 1);

    let other = loop_available_coalitions(&mut counter, &mut memo);
    let elapsed = now.elapsed();
    println!("a: {}", result);
    println!("b: {}", other);
    let sum = (result * (N - 2)) + (other * (N - 2));
    println!("(a + b) = {}", sum * N * (N - 1));
    println!("Elapsed: {:?}", elapsed);
}
