mod waves;
// use waves::Wave;
mod serialize;
// use serialize::duty_cycle;
mod wave_forms;
mod note;
mod standard_notes;

fn main() {
    // let s = pulse(0.0, 0.0, 0.0);
    // println!("{}", duty_cycle(0.0));
    println!("{:?}", standard_notes::AVAILABLE_NOTES);
}
