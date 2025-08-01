mod audio;
mod waves;
mod serialize;
mod tone;

fn main() {
    // let s = pulse(0.0, 0.0, 0.0);
    // println!("{}", duty_cycle(0.0));
    println!("{:?}", tone::standard_notes::AVAILABLE_NOTES);
}
