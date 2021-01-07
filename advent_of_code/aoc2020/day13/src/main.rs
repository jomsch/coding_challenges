mod part_one; 

fn main() {
    let raw_input = include_str!("../input.txt");
    part_one::main(raw_input);
}


struct Bus {
    bus_id: u32,
    delay: u32,
}

fn parse_input(input: &str) -> Vec<Bus> {
    let mut delay = 0;
    input.lines()
        .skip(1).next().unwrap()
        .split(',')
        .fold(Vec::new(), |acc, bus_id| {
            match 
            delay+=1
        })
}

