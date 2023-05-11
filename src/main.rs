use fxhash::FxHashMap;
use lazy_static::lazy_static;
use rug::Integer;
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


fn loop_available_coalitions(counter: &mut Vec<i8>, memo: &mut FxHashMap<Vec<i8>, Integer>) -> Integer {
    // let memo = MEMO.lock().unwrap();
    // if let Some(result) = memo.get(counter) {
    //     return result.clone();
    // }
    // drop(memo);
    
    if let Some(result) = memo.get(counter) {
        return result.clone();
    }

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

fn main() {
    let mut counter = match N {
        3 => vec![3, 1, 1, 0, 0, -1, -1, -1],
        4 => vec![4, 3, 3, 2, 3, 1, 1, 0, 3, 1, 1, 0, 0, -1, -1, -1],
        5 => vec![5, 4, 4, 3, 4, 3, 3, 2, 4, 3, 3, 2, 3, 1, 1, 0, 4, 3, 3, 2, 3, 1, 1, 0, 3, 1, 1, 0, 0, -1, -1, -1],
        6 => vec![6, 5, 5, 4, 5, 4, 4, 3, 5, 4, 4, 3, 4, 3, 3, 2, 5, 4, 4, 3, 4, 3, 3, 2, 4, 3, 3, 2, 3, 1, 1, 0, 5, 4, 4, 3, 4, 3, 3, 2, 4, 3, 3, 2, 3, 1, 1, 0, 4, 3, 3, 2, 3, 1, 1, 0, 3, 1, 1, 0, 0, -1, -1, -1],
        7 => vec![7, 6, 6, 5, 6, 5, 5, 4, 6, 5, 5, 4, 5, 4, 4, 3, 6, 5, 5, 4, 5, 4, 4, 3, 5, 4, 4, 3, 4, 3, 3, 2, 6, 5, 5, 4, 5, 4, 4, 3, 5, 4, 4, 3, 4, 3, 3, 2, 5, 4, 4, 3, 4, 3, 3, 2, 4, 3, 3, 2, 3, 1, 1, 0, 6, 5, 5, 4, 5, 4, 4, 3, 5, 4, 4, 3, 4, 3, 3, 2, 5, 4, 4, 3, 4, 3, 3, 2, 4, 3, 3, 2, 3, 1, 1, 0, 5, 4, 4, 3, 4, 3, 3, 2, 4, 3, 3, 2, 3, 1, 1, 0, 4, 3, 3, 2, 3, 1, 1, 0, 3, 1, 1, 0, 0, -1, -1, -1],
        _ => panic!("Choose a different number of n"),
    };
    
    let mut memo = FxHashMap::default();
    let now = Instant::now();
    let result = loop_available_coalitions(&mut counter, &mut memo);
    let elapsed = now.elapsed();
    println!("Result: {}", result * N * (N-1));
    println!("Elapsed: {:?}", elapsed);
}
