pub const A_FREQUENCY: f64 = 440_f64;
const LOWER_FREQUENCY_THRESHOLD: f64 = 20_f64;
const UPPER_FREQUENCY_THRESHOLD: f64 = 20_000_f64;
pub const LOWEST_NOTE: f64 = get_first_note(LOWER_FREQUENCY_THRESHOLD);
const SEMI_TONE_FACTOR: f64 = f64::from_bits(4607450216769616227);
pub const NUMBER_OF_AVAILABLE_NOTES: usize = get_size(UPPER_FREQUENCY_THRESHOLD);
pub const AVAILABLE_NOTES: [f64; NUMBER_OF_AVAILABLE_NOTES] = generate_array();

const fn get_first_note(lower_frequency_threshold: f64) -> f64 {
    let mut frequency: f64 = A_FREQUENCY;
    while frequency > lower_frequency_threshold {
        frequency /= SEMI_TONE_FACTOR;
    }
    return frequency * SEMI_TONE_FACTOR;
}

const fn get_size(upper_frequency_threshold: f64) -> usize {
    let mut frequency: f64 = LOWEST_NOTE;
    let mut counter = 0;
    while frequency < upper_frequency_threshold {
        frequency *= SEMI_TONE_FACTOR;
        counter += 1;
    }
    return counter;
}

const fn generate_array() -> [f64; NUMBER_OF_AVAILABLE_NOTES] {
    let mut array = [0_f64; NUMBER_OF_AVAILABLE_NOTES];
    array[0] = LOWEST_NOTE;
    let mut index: usize = 0;
    while index < NUMBER_OF_AVAILABLE_NOTES - 1 {
        index += 1;
        array[index] = array[index - 1] * SEMI_TONE_FACTOR;
    }
    return array;
}

// pub struct Key<const NUMBER_OF_AVAILABLE_NOTES: usize>(usize);
//
// impl Key {
//
// }
