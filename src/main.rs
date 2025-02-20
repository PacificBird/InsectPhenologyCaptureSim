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
    //     true,
    // );
    // let test_0_csv = test_0.to_csv_string();
    // let mut file_0 = std::fs::File::create("test_0.csv").unwrap();
    // writeln!(&mut file_0, "{}", test_0_csv).expect("Couldn't write test_0.csv");

    // let test_1 = multisim(
    //     MultiParam::Range(1_000.0..=10_000.0, 20),
    //     MultiParam::Range(0.005..=0.101, 20),
    //     0..=2200,
    //     JW_EMERGENCES,
    //     MultiParam::Range(0.0..=80.0, 4),
    //     egg_coefficient,
    //     false,
    // );
    // println!("before csv string");
    // let test_1_csv = test_1.to_csv_string();
    // println!("after csv string");
    // let mut file_1 = std::fs::File::create("test_1.csv").unwrap();
    // writeln!(&mut file_1, "{}", test_1_csv).expect("Couldn't write test_0.csv");

    let simdata = multisim(
        MultiParam::Constant(1_000.0),
        MultiParam::Range(0.0005..=0.1, 1000),
        0..=2200,
        JW_EMERGENCES,
        MultiParam::Constant(0.0),
        egg_coefficient,
        false,
    );
    let simdata_csv = simdata.to_csv_string();
    println!("after csv string");
    let mut simdata_file = std::fs::File::create("simdata.csv").unwrap();
    writeln!(&mut simdata_file, "{}", simdata_csv).expect("Couldn't write simdata.csv");

    // let test_3 = multisim(
    //     MultiParam::Constant(1_000.0),
    //     MultiParam::Range(0.001..=1.0, 800),
    //     0..=2200,
    //     JW_EMERGENCES,
    //     MultiParam::Constant(0.0),
    //     egg_coefficient,
    //     true,
    // );
    // let test_3_csv = test_3.to_csv_string();
    // println!("before file creation");
    // let mut file_3 = std::fs::File::create("test_3.csv").unwrap();
    // println!("after file creation");
    // writeln!(&mut file_3, "{}", test_3_csv).expect("Couldn't write test_3.csv");
    // println!("after writing");
}
