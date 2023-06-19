use super::{simulate, DataPoint, JW_EMERGENCES};

#[derive(Debug)]
pub struct FittingData {
    delay: f64,
    target_rm: f64,
    gen_time: f64,
    seed_coefficient: f64,
}
impl FittingData {
    pub fn new(delay: f64, target_rm: f64, gen_time: f64, seed_coefficient: f64) -> Self {
        Self {
            delay,
            target_rm,
            gen_time,
            seed_coefficient,
        }
    }

    pub fn new_from_calendar_day(
        delay: f64,
        gen_time: f64,
        seed_coefficient: f64,
        dd_per_day: f64,
    ) -> Self {
        Self {
            delay,
            target_rm: f64::ln(seed_coefficient) / (dd_per_day * gen_time),
            gen_time: gen_time * dd_per_day,
            seed_coefficient,
        }
    }

    pub fn new_many<const N: usize>(
        delay: [f64; N],
        target_rm: [f64; N],
        gen_time: [f64; N],
        seed_coefficient: [f64; N],
    ) -> [Self; N] {
        (0..N)
            .into_iter()
            .map(|n| Self::new(delay[n], target_rm[n], gen_time[n], seed_coefficient[n]))
            .collect::<Vec<Self>>()
            .try_into()
            .expect("Fixed size map should have exactly N elements")
    }

    pub fn new_many_from_calendar_day<const N: usize>(
        delay: [f64; N],
        gen_time: [f64; N],
        seed_coefficient: [f64; N],
        dd_per_day: f64,
    ) -> [Self; N] {
        (0..N)
            .into_iter()
            .map(|n| {
                Self::new_from_calendar_day(delay[n], gen_time[n], seed_coefficient[n], dd_per_day)
            })
            .collect::<Vec<Self>>()
            .try_into()
            .expect("Fixed size map should have exactly N elements")
    }
}

pub fn fit_pop_growth<const N: usize>(fitparams: [FittingData; N]) -> [f64; N] {
    (0..N)
        .into_iter()
        .map(|n| {
            let mut coefficient = fitparams[n].seed_coefficient;
            let mut rm = 1.0;
            while f64::abs(1.0 - (rm / fitparams[n].target_rm)) > 0.001 {
                let data = simulate(
                    100_000,
                    0.05,
                    0..=2200,
                    JW_EMERGENCES.to_vec(),
                    fitparams[n].delay,
                    coefficient,
                );
                rm = calc_inherent_growth(
                    100_000.0,
                    get_last_egg_total(&data, 2),
                    fitparams[n].gen_time * 2.0,
                );
                coefficient *= 1.0 / (rm / fitparams[n].target_rm);
            }
            coefficient
        })
        .collect::<Vec<f64>>()
        .try_into()
        .expect("Fixed size map should have exactly N elements")
}

pub fn calc_inherent_growth(n0: f64, nn: f64, t: f64) -> f64 {
    f64::ln(nn / n0) / t
}

pub fn get_last_egg_total(data: &Vec<DataPoint>, gen: usize) -> f64 {
    data.get(2200)
        .expect("Couldn't get 2200th datapoint")
        .eggs_total
        .get(gen + 1)
        .expect("Couldn't get {gen} egg_total")
        .clone()
}
