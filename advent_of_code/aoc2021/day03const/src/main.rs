const INPUT: &'static str = include_str!("../../day03/input.txt");
const INPUT_BYTES: &[u8] = INPUT.as_bytes();
const PARSED_INPUT: [(u8, [u8; 12]); 1000] = parse_input();

const PART_ONE: usize = part_one();
//const PART_TWO: usize = part_two();

fn main() {
    println!("Part one answer: {}", PART_ONE);
    //println!("Part two answer: {:?}", PART_TWO);
}

const fn parse_input() -> [(u8, [u8; 12]); 1000] {
    const NB: u8 = b'0';
    let mut list: [(u8, [u8; 12]); 1000] = [(1, [0; 12]); 1000];

    let input_bytes = INPUT_BYTES;

    let mut i = 0;
    loop {
        if i >= (INPUT.len() / 13) {
            break;
        }

        let mut j = 0;
        'inner: loop {
            if j >= 12 {
                break 'inner;
            }
            let idx = i * 13 + j;
            list[i].1[j] = input_bytes[idx] - NB;
            j += 1;
        }

        i += 1;
    }

    list
}

const fn common_bits(bits: &[(u8, [u8; 12]); 1000]) -> [u8; 12] {
    let mut bit_counter: [isize; 12] = [0; 12];

    let mut i = 0;
    loop {
        if i >= 1000 {
            break;
        }
        if bits[i].0 == 0 {
            continue;
        }
        let mut j = 0;
        'inner: loop {
            if j >= 12 {
                break 'inner;
            }
            if bits[i].1[j] == 1 {
                bit_counter[j] += 1;
            } else {
                bit_counter[j] += -1;
            }
            j += 1;
        }
        i += 1;
    }
    let mut common_bits = [0; 12];

    let mut i = 0;
    loop {
        if i >= 12 {
            break;
        }
        if bit_counter[i] >= 0 {
            common_bits[i] = 1;
        } else {
            common_bits[i] = 0;
        }
        i += 1;
    }

    common_bits
}

const fn part_one() -> usize {
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    let common_bits = common_bits(&PARSED_INPUT);

    let mut i = 0;
    loop {
        if i >= 12 {
            break;
        }
        if common_bits[i] == 1 {
            gamma_rate = (gamma_rate << 1) ^ 1;
            epsilon_rate = (epsilon_rate << 1) ^ 0;
        } else {
            gamma_rate = (gamma_rate << 1) ^ 0;
            epsilon_rate = (epsilon_rate << 1) ^ 1;
        }

        i += 1;
    }

    gamma_rate * epsilon_rate
}

const fn max_active(bits: &[(u8, [u8; 12]); 1000]) -> usize {
    let mut i = 0;
    let mut active = 0;
    loop {
        if i >= 1000 {
            break;
        }
        if bits[i].0 == 1 {
            active += 1;
        }
        i += 1;
    }
    active
}

const fn get_active(bits: &[(u8, [u8; 12]); 1000]) -> usize {
    let mut i = 0;
    loop {
        if i >= 1000 {
            break;
        }
        if bits[i].0 == 1 {
            return i;
        }
        i += 1;
    }
    0
}

// const fn part_two() -> usize {
//     let mut more_bits = parse_input();
//     let mut more_bits_idx = 0;
//     let mut fewer_bits = parse_input();
//     let mut fewer_bits_idx = 0;

//     let mut i = 0;
//     loop {
//         if i >= 12 || (more_bits_idx > 0 && fewer_bits_idx > 0) {
//             break;
//         }

//         if more_bits_idx == 0 {
//             let more_bits_common = common_bits(&more_bits);
//             let mut j = 0;
//             'inner: loop {
//                 if j >= 1000 {
//                     break 'inner;
//                 }
//                 if more_bits[j].0 == 1 && more_bits[i].1[i] != more_bits_common[i] {
//                     more_bits[j].0 = 0;
//                 }
//                 j += 1;
//             }
//         }
//         if max_active(&more_bits) == 1 {
//             more_bits_idx = get_active(&more_bits);
//         }

//         if fewer_bits_idx == 0 {
//             let fewer_bits_common = common_bits(&fewer_bits);
//             let mut j = 0;
//             'inner2: loop {
//                 if j >= 1000 {
//                     break 'inner2;
//                 }
//                 if fewer_bits[j].0 == 1 && fewer_bits[i].1[i] == fewer_bits_common[i] {
//                     fewer_bits[j].0 = 0;
//                 }
//                 j += 1;
//             }

//             if max_active(&fewer_bits) == 1 {
//                 fewer_bits_idx = get_active(&more_bits);
//             }
//         }

//         i += 1;
//     }

//     let mut oxygen_generator = 0;
//     let mut co2_scrubber = 0;

//     let mut i = 0;
//     loop {
//         if i >= 12 {
//             break;
//         }

//         let mbit = more_bits[more_bits_idx].1[i] as usize;
//         oxygen_generator = (oxygen_generator << 1) | mbit;

//         // let fbit = fewer_bits[fewer_bits_idx].1[i] as usize;
//         // co2_scrubber = (co2_scrubber << 1) | fbit;
//         i += 1;
//     }

//     oxygen_generator * co2_scrubber
// }
