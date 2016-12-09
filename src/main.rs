extern crate sacn;
use sacn::DmxSource;

const UNIVERSE_SIZE: usize = 510;

struct Zone  { head: u8, body: u8, tail: u8, name: String }

fn main() {
    let zones: [Zone; 6] = [
        Zone { head: 3, body: 47, tail: 0, name: "10".to_string() },
        Zone { head: 2, body: 92, tail: 2, name: "11a".to_string() },
        Zone { head: 2, body: 92, tail: 2, name: "11b".to_string() },
        Zone { head: 2, body: 90, tail: 3, name: "12a".to_string() },
        Zone { head: 2, body: 91, tail: 3, name: "12b".to_string() },
        Zone { head: 2, body: 43, tail: 0, name: "13".to_string() }
    ];
    let mut out: Vec<u8> = vec![];
    for zone in zones.iter() {
        for i in 1..zone.head {
            out.push(0); // Red
            out.push(0); // Green
            out.push(0); // Blue
        }
        for i in 1..zone.body {
            out.push(0); // Red
            out.push(0); // Green
            out.push(0); // Blue
        }
        for i in 1..zone.tail {
            out.push(0); // Red
            out.push(0); // Green
            out.push(0); // Blue
        }
    }
    let mut universes = Vec::new();
    while out.len() > UNIVERSE_SIZE {
        let u = out.split_off(UNIVERSE_SIZE);
        universes.push(u);
    }
    universes.push(out);

    let dmx = DmxSource::new("Controller").unwrap();
    let mut universe: u16 = 1;
    while let Some(u) = universes.pop() {
        dmx.send(universe, &u);
        universe += 1;
    }
    dmx.terminate_stream(1);
}
