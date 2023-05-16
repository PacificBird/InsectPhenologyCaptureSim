use compute::integrate;
use rand::prelude::*;
use std::{
    f64::consts::{E, PI},
    fmt,
    fmt::{Display, Formatter},
    io::Write,
};

struct DataPoint {
    pop_captured: u32,
    pop_0: f64,
    pop_active: u32,
    eggs: u32,
}
impl Display for DataPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{},{},{}",
            self.pop_captured, self.pop_0, self.pop_active, self.eggs
        )
    }
}

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
        .map(|(idx, x)| format!("{},{}\n", idx, x.to_string()))
        .reduce(|acc, row| format!("{}{}", acc, row))
        .unwrap();
    let test_0_csv = format!("dd,captured,population,active,eggs\n{}", test_0_csv);
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
) -> Vec<DataPoint> {
    let mut pop_inactive = pop_0 as f64;
    // pop_inactive *= 1.55;
    let mut pop_active = 0;
    let mut pop_captured = 0;
    let mut avg_age: f64 = 0.0;
    let mut eggs: u32 = 0;
    let mut mating_pop: u32 = 0;
    let mut mating_now: u32 = 0;

    deg_day_range
        .into_iter()
        .map(|x| {
            // !TODO: Make option for using a PDF directly, or continue using CDF, possibly using an enum?
            let activated = (pop_inactive * diff(x as f64, &cdf)) / 100.0;
            pop_inactive -= activated;
            pop_active += activated as u32;
            mating_pop += (activated / 2.0) as u32;

            avg_age += 1f64;
            avg_age *= 1f64 - (activated / pop_active as f64);

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

            mating_now = (mating_pop as f64
                * integrate::quad5(curried_normal(40.0, 2.0), avg_age - 0.5, avg_age + 0.5))
            .round() as u32;
            if x % 3 == 0 {
                println!(
                    "{x}, {activated}, {avg_age}, {}",
                    (1f64 - (activated / (pop_0 as f64 - pop_inactive)))
                );
            }
            mating_pop.checked_sub(mating_now).unwrap_or(0);
            eggs += (15.0 * mating_now as f64) as u32;

            // println!("{},{},{},{}", pop_active, captured, died, avg_age);
            pop_active -= captured;
            pop_captured += captured;
            pop_active = pop_active.checked_sub(died).unwrap_or(0);

            // avg_age *= ((pop_active != 0) as i32) as f64;

            DataPoint {
                pop_captured,
                pop_0: (pop_0 as f64 - pop_inactive),
                pop_active,
                eggs,
            }
        })
        .collect::<Vec<DataPoint>>()
}

fn example_cdf(x: f64) -> f64 {
    100.0 / (1.0 + E.powf(-0.05 * (x - 200.0)))
}

fn normal(mean: f64, stdev: f64, x: f64) -> f64 {
    (1.0 / (stdev * (2.0 * PI).sqrt())) * E.powf(-0.5 * ((x - mean) / stdev).powi(2))
}

fn curried_normal(mean: f64, stdev: f64) -> impl Fn(f64) -> f64 {
    move |x| normal(mean, stdev, x)
}
