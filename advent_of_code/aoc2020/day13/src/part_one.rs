type Timestamp = u32;
type BusId = u32;

fn parse_input(input: &str) -> (Timestamp, Vec<BusId>) {
    let mut lines = input.lines();
    let timestamp = lines.next().unwrap().parse::<u32>().unwrap();
    let bus_ids = lines.next()
        .unwrap()
        .split(',')
        .filter_map(|id| id.parse::<u32>().ok())
        .collect();

    (timestamp, bus_ids)
}

fn bus_departs(ts: Timestamp, bus_ids: &Vec<BusId>) -> Option<&BusId> {
    bus_ids.iter()
        .filter(|id| ts % **id == 0)
        .next()
}

pub fn main(input: &str) {
    let (timestamp, bus_ids) = parse_input(input);
    let mut inc_ts = timestamp;

    loop {
        match bus_departs(inc_ts, &bus_ids) {
            None => inc_ts += 1,
            Some(id) => {
                let minutes = inc_ts - timestamp;
                dbg!(minutes * id);
                break;
            }
        }
    }
}

