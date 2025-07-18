mod waves;
// use waves::Wave;
mod serialize;
// use serialize::duty_cycle;
mod wave_forms;
mod note;

fn main() {
    // let s = pulse(0.0, 0.0, 0.0);
    // println!("{}", duty_cycle(0.0));
    println!("{:?}", note::AVAILABLE_NOTES);
}
