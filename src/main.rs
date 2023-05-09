use rand::prelude::*;
use std::io::Write;

const E: f64 = std::f64::consts::E;

fn main() {
    // let pop_range = 1_000..=100_000;
    // let delta_pop = 100;
    // let detection_range = 0.0025..=0.050;
    // let delta_detection = 0.0005;
    let degree_day_range = 0..=400;

    let test_0 = simulate(1000000, 0.00025, degree_day_range.clone(), &example_cdf);
    let test_0_csv = test_0
        .into_iter()
        .enumerate()
        .map(|(idx, x)| format!("{},{},{},{},{}\n", idx, x.0, x.1, x.2, x.3))
        .reduce(|acc, row| format!("{}{}", acc, row))
        .unwrap();
    let test_0_csv = format!("dd,captured,population,active,age\n{}", test_0_csv);
    let mut file_0 = std::fs::File::create("test_0.csv").unwrap();
    writeln!(&mut file_0, "{}", test_0_csv).expect("Couldn't write test_0.csv");
}

fn diff(x: f64, cdf: impl Fn(f64) -> f64) -> f64 {
    let h = 0.001;
    (cdf(x + h) - cdf(x)) / h
}

fn simulate(
    pop_0: u32,
    prob_detection: f64,
    deg_day_range: std::ops::RangeInclusive<u32>,
    cdf: impl Fn(f64) -> f64,
) -> Vec<(u32, f64, u32, u32)> {
    let mut pop_inactive = pop_0 as f64;
    let mut pop_active = 0;
    let mut pop_captured = 0;
    let mut avg_age: f64 = 0.0;
    let mut pop_active_last: u32 = 0;

    deg_day_range
        .into_iter()
        .map(|x| {
            let activated = ((pop_inactive * diff(x as f64, &cdf)) / 100.0).round();
            pop_inactive -= activated;
            pop_active += activated as u32;

            avg_age += 1f64.min(if pop_active == 0 {
                0f64
            } else {
                (pop_active_last as f64) / (pop_active as f64)
            });

            let pop_active_before = pop_active;

            let mut rng = thread_rng();
            let (captured, died): (u32, u32) =
                (0..=pop_active).fold((0u32, 0u32), |(cap, ded), _| {
                    if rng.gen::<f64>() <= prob_detection {
                        (cap + 1, ded)
                    } else if rng.gen::<f64>()
                        >= E.powf(0.059 * (1f64 - E.powf(0.044 * avg_age as f64)))
                    {
                        (cap, ded + 1)
                    } else {
                        (cap, ded)
                    }
                });
            // println!("{},{},{},{}", pop_active, captured, died, avg_age);
            pop_active -= captured;
            pop_captured += captured;
            pop_active = pop_active.checked_sub(died).unwrap_or(0);

            pop_active_last = pop_active;

            (
                pop_captured,
                pop_0 as f64 - pop_inactive,
                pop_active_before,
                died,
            )
        })
        .collect::<Vec<(u32, f64, u32, u32)>>()
}

fn example_cdf(x: f64) -> f64 {
    100.0 / (1.0 + E.powf(-0.05 * (x - 200.0)))
}
