use std::io::Write;
use InsectPhenologyCaptureSim::{
    egg_coefficient,
    multisim::{multisim, MultiParam},
    simulate, JW_EMERGENCES,
};

fn main() {
    let mating_delay = 0.0;
    let test_0 = simulate(
        100_000,
        0.00025,
        0..=2200,
        JW_EMERGENCES,
        mating_delay,
        egg_coefficient(mating_delay),
    );
    let test_0_csv = test_0.to_csv_string();
    let mut file_0 = std::fs::File::create("test_0.csv").unwrap();
    writeln!(&mut file_0, "{}", test_0_csv).expect("Couldn't write test_0.csv");

    let test_1 = multisim(
        MultiParam::Range(1_000.0..=1_001_000.0, 20),
        // MultiParam::Constant(100_000.0),
        MultiParam::Range(0.001..=0.101, 20),
        // MultiParam::Constant(0.05),
        0..=2200,
        JW_EMERGENCES,
        MultiParam::Range(0.0..=80.0, 4),
        egg_coefficient,
    );
    println!("before csv string");
    let test_1_csv = test_1.to_csv_string();
    println!("after csv string");
    let mut file_1 = std::fs::File::create("test_1.csv").unwrap();
    writeln!(&mut file_1, "{}", test_1_csv).expect("Couldn't write test_0.csv");
}
