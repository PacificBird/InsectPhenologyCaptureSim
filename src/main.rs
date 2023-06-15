use std::io::Write;
use InsectPhenologyCaptureSim::{
    egg_coefficient, fitting::fit_pop_growth, simulate, JW_EMERGENCES,
};

fn main() {
    println!("fitted vals: {:#?}", fit_pop_growth());

    let mating_delay = 0.0;
    let test_0 = simulate(
        100_000,
        0.00025,
        0..=2200,
        JW_EMERGENCES.to_vec(),
        mating_delay,
        egg_coefficient(mating_delay),
    );
    let test_0_headers = test_0.get(1).unwrap().csv_headers();
    let test_0_csv = test_0
        .into_iter()
        .enumerate()
        .map(|(idx, x)| format!("{},{}\n", idx, x.to_string()))
        .reduce(|acc, row| format!("{}{}", acc, row))
        .unwrap();
    let test_0_csv = format!("{}\n{}", test_0_headers, test_0_csv);
    let mut file_0 = std::fs::File::create("test_0.csv").unwrap();
    writeln!(&mut file_0, "{}", test_0_csv).expect("Couldn't write test_0.csv");
}
