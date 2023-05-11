macro_rules! count_bits {
    ($i:expr) => {{
        let mut i = $i;
        i = i - ((i >> 1) & 0x55555555);
        i = (i & 0x33333333) + ((i >> 2) & 0x33333333);
        i = (i + (i >> 4)) & 0x0F0F0F0F;
        i = (i * 0x01010101) >> 24;
        i
    }};
}

macro_rules! init_counter {
    ($num_elements:expr) => {{
        let mut counter: Vec<i8> = vec![0; 1 << $num_elements];
        for i in 0..(1 << $num_elements) {
            counter[i as usize] = ($num_elements - count_bits!(i as usize))
                .try_into()
                .unwrap();
        }
        counter
    }};
}

// fn count_bits_fn(mut i: usize) -> usize {
//     i = i - ((i >> 1) & 0x55555555);
//     i = (i & 0x33333333) + ((i >> 2) & 0x33333333);
//     i = (i + (i >> 4)) & 0x0F0F0F0F;
//     (i * 0x01010101) >> 24
// }

// fn init_counter_fn(num_elements: usize) -> Vec<i8> {
//     let mut counter: Vec<i8> = vec![0; 1 << num_elements];
//     for i in 0..(1 << num_elements) {
//         counter[i] = (num_elements - count_bits_fn(i)).try_into().unwrap();
//     }
//     counter
// }
