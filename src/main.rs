use std::io::Write;
use InsectPhenologyCaptureSim::{
    egg_coefficient,
    multisim::{multisim, MultiParam},
    simulate, JW_EMERGENCES,
};

fn main() {
    // let mating_delay = 0.0;
    // let test_0 = simulate(
    //     100_000,
    //     0.00025,
    //     0..=2200,
    //     JW_EMERGENCES,
    //     mating_delay,
    //     egg_coefficient(mating_delay),
    // );
    // let test_0_csv = test_0.to_csv_string();
    // let mut file_0 = std::fs::File::create("test_0.csv").unwrap();
    // writeln!(&mut file_0, "{}", test_0_csv).expect("Couldn't write test_0.csv");

    let test_1 = multisim(
        MultiParam::Range(5_000.0..=1_001_000.0, 30),
        MultiParam::Range(0.001..=0.101, 30),
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

    // let test_2 = multisim(
    //     MultiParam::Constant(1_000.0),
    //     MultiParam::Range(0.0001..=0.2, 400),
    //     0..=2200,
    //     JW_EMERGENCES,
    //     MultiParam::Constant(0.0),
    //     egg_coefficient,
    // );
    // let test_2_csv = test_2.to_csv_string();
    // let mut file_2 = std::fs::File::create("test_2.csv").unwrap();
    // writeln!(&mut file_2, "{}", test_2_csv).expect("Couldn't write test_2.csv");
}
