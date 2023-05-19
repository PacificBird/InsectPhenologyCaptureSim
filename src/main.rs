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

#[allow(dead_code)]
enum ProbDist<'a> {
    CDF(&'a dyn Fn(f64) -> f64),
    PDF(&'a dyn Fn(f64) -> f64),
}

fn main() {
    // let pop_range = 1_000..=100_000;
    // let delta_pop = 100;
    // let detection_range = 0.0025..=0.050;
    // let delta_detection = 0.0005;
    let degree_day_range = 0..=1000;

    let test_0 = simulate(
        100000,
        0.00025,
        degree_day_range.clone(),
        ProbDist::PDF(&jones_wiman_2012),
    );
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
    emergence: ProbDist,
) -> Vec<DataPoint> {
    let mut pop_inactive = pop_0 as f64;
    // pop_inactive *= 1.55;
    let mut pop_active = 0;
    let mut pop_captured = 0;
    let mut avg_age: f64 = 0.0;
    let mut eggs: u32 = 0;
    let mut mating_pop: u32 = 0;
    let mut mating_now: u32 = 0;
    let mut pop_active_last = 0;

    deg_day_range
        .into_iter()
        .map(|x| {
            let activated = ((pop_inactive
                * match emergence {
                    ProbDist::CDF(cdf) => diff(x as f64, &*cdf),
                    ProbDist::PDF(pdf) => pdf(x as f64),
                })
                / 100.0)
                .max(0.0);
            pop_inactive -= activated;
            pop_active += activated as u32;
            // mating_pop += (activated / 2.0) as u32;

            avg_age += 1f64;
            avg_age *= if pop_active != 0 {
                (pop_active_last as f64 / pop_active as f64)
            } else {
                0.0
            };

            let mut rng = thread_rng();
            let (captured, died): (u32, u32) =
                (0..pop_active).fold((0u32, 0u32), |(cap, ded), _| {
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

            mating_now = ((pop_active as f64 / 2.0)
                * integrate::quad5(curried_normal(40.0, 8.0), avg_age - 0.5, avg_age + 0.5))
            .round() as u32;
            if x % 5 == 0 {
                println!(
                    "{x}, activated: {}, age: {avg_age}, proprtion: {}",
                    activated.round(),
                    (1f64 - (activated.round() / pop_active as f64))
                );
            }
            // mating_pop = mating_pop.checked_sub(mating_now).unwrap_or(0);
            eggs += (15.0 * mating_now as f64) as u32;

            // println!("{},{},{},{}", pop_active, captured, died, avg_age);
            pop_active -= captured;
            pop_captured += captured;
            pop_active = pop_active.checked_sub(died).unwrap_or(0);

            pop_active_last = pop_active;
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

#[allow(dead_code)]
fn example_cdf(x: f64) -> f64 {
    100.0 / (1.0 + E.powf(-0.05 * (x - 200.0)))
}

fn jones_wiman_2012(x: f64) -> f64 {
    let gamma = 1.0737;
    let delta = 1.2349;
    let lambda = 577.2;
    let zeta = 69.0;
    let z = (x - zeta) / lambda;

    (100.0
        * (delta / (lambda * (2.0 * PI).sqrt() * z * (1.0 - z)))
        * f64::exp(-0.5 * (gamma + (delta * (z / (1.0 - z)).ln())).powi(2)))
    .max(0.0)
}

fn normal(mean: f64, stdev: f64, x: f64) -> f64 {
    (1.0 / (stdev * (2.0 * PI).sqrt())) * E.powf(-0.5 * ((x - mean) / stdev).powi(2))
}

fn curried_normal(mean: f64, stdev: f64) -> impl Fn(f64) -> f64 {
    move |x| normal(mean, stdev, x)
}
