use super::{simulate, DataPoint, JW_EMERGENCES};

pub fn fit_pop_growth() -> [f64; 3] {
    let mut coefficient_0 = 15.0;
    let mut rm_0 = 1.0;
    while f64::abs(1.0 - (rm_0 / 0.05)) > 0.001 {
        let data = simulate(
            100_000,
            0.05,
            0..=2200,
            JW_EMERGENCES.to_vec(),
            0.0,
            coefficient_0,
        );
        rm_0 = calc_inherent_growth(100_000.0, get_last_egg_total(&data, 2), 55.2 * 2.0);
        println!(
            "no delay rm: {rm_0}, coefficient: {coefficient_0}, % deviation: {}",
            f64::abs(1.0 - (rm_0 / 0.05))
        );
        coefficient_0 *= 1.0 / (rm_0 / 0.05);
    }
    let mut coefficient_1 = 9.7;
    let mut rm_1 = 1.0;
    while f64::abs(1.0 - (rm_1 / 0.0418)) > 0.001 {
        let data = simulate(
            100_000,
            0.05,
            0..=2200,
            JW_EMERGENCES.to_vec(),
            40.0,
            coefficient_1,
        );
        rm_1 = calc_inherent_growth(100_000.0, get_last_egg_total(&data, 2), 57.3 * 2.0);
        println!(
            "40 delay rm: {rm_1}, coefficient: {coefficient_1}, % deviation: {}",
            f64::abs(1.0 - (rm_1 / 0.0418))
        );
        coefficient_1 *= 1.0 / (rm_1 / 0.0418);
    }
    let mut coefficient_2 = 8.6;
    let mut rm_2 = 1.0;
    while f64::abs(1.0 - (rm_2 / 0.0384)) > 0.001 {
        let data = simulate(
            100_000,
            0.05,
            0..=2200,
            JW_EMERGENCES.to_vec(),
            50.0,
            coefficient_2,
        );
        rm_2 = calc_inherent_growth(100_000.0, get_last_egg_total(&data, 2), 66.5 * 2.0);
        println!(
            "50 delay rm: {rm_0}, coefficient: {coefficient_0}, % deviation: {}",
            f64::abs(1.0 - (rm_2 / 0.0384))
        );
        coefficient_2 *= 1.0 / (rm_2 / 0.0384);
    }
    [coefficient_0, coefficient_1, coefficient_2]
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
