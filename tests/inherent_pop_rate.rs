use InsectPhenologyCaptureSim::{
    egg_coefficient, fitting::get_last_egg_total, simulate, DataPoint, ProbDist, JW_EMERGENCES,
};

#[test]
fn pop_growth() {
    let no_delay = simulate(
        100_000,
        0.005,
        0..=2200,
        JW_EMERGENCES.to_vec(),
        0.0,
        egg_coefficient(0.0),
    );
    let fourty_delay = simulate(
        100_000,
        0.005,
        0..=2200,
        JW_EMERGENCES.to_vec(),
        40.0,
        egg_coefficient(40.0),
    );
    let fifty_delay = simulate(
        100_000,
        0.005,
        0..=2200,
        JW_EMERGENCES.to_vec(),
        50.0,
        egg_coefficient(50.0),
    );

    let zero_rm = 0.05;
    let fourty_rm = 0.0418;
    let fifty_rm = 0.0384;

    let zero_gentime = 55.2;
    let fourty_gentime = 57.3;
    let fifty_gentime = 66.5;

    let exp_growth = |rm: f64, t: f64, n0: f64| n0 * f64::exp(rm * t);

    let diff_0delay_0gen =
        exp_growth(zero_rm, zero_gentime, 100_000.0) - get_last_egg_total(&no_delay, 0);
    let diff_0delay_1gen =
        exp_growth(zero_rm, zero_gentime * 2.0, 100_000.0) - get_last_egg_total(&no_delay, 1);
    let diff_0delay_2gen =
        exp_growth(zero_rm, zero_gentime * 3.0, 100_000.0) - get_last_egg_total(&no_delay, 2);
    let diff_40delay_0gen =
        exp_growth(fourty_rm, fourty_gentime, 100_000.0) - get_last_egg_total(&fourty_delay, 0);
    let diff_40delay_1gen = exp_growth(fourty_rm, fourty_gentime * 2.0, 100_000.0)
        - get_last_egg_total(&fourty_delay, 1);
    let diff_40delay_2gen = exp_growth(fourty_rm, fourty_gentime * 3.0, 100_000.0)
        - get_last_egg_total(&fourty_delay, 2);
    let diff_50delay_0gen =
        exp_growth(fifty_rm, fifty_gentime, 100_000.0) - get_last_egg_total(&fifty_delay, 0);
    let diff_50delay_1gen =
        exp_growth(fifty_rm, fifty_gentime * 2.0, 100_000.0) - get_last_egg_total(&fifty_delay, 1);
    let diff_50delay_2gen =
        exp_growth(fifty_rm, fifty_gentime * 3.0, 100_000.0) - get_last_egg_total(&fifty_delay, 2);

    println!("--no delay--");
    println!(
        "gen 0: {}, gen 1: {}, gen 2: {}",
        get_last_egg_total(&no_delay, 0),
        get_last_egg_total(&no_delay, 1),
        get_last_egg_total(&no_delay, 2)
    );
    println!(
        "gen 0: {}, gen 1: {}, gen 2: {}",
        exp_growth(zero_rm, zero_gentime, 100_000.0),
        exp_growth(zero_rm, zero_gentime * 2.0, 100_000.0),
        exp_growth(zero_rm, zero_gentime * 3.0, 100_000.0)
    );
    println!("gen 0: {diff_0delay_0gen}, gen 1: {diff_0delay_1gen}, gen 2: {diff_0delay_2gen}");
    println!("--40 delay--");
    println!(
        "gen 0: {}, gen 1: {}, gen 2: {}",
        get_last_egg_total(&fourty_delay, 0),
        get_last_egg_total(&fourty_delay, 1),
        get_last_egg_total(&fourty_delay, 2)
    );
    println!(
        "gen 0: {}, gen 1: {}, gen 2: {}",
        exp_growth(fourty_rm, fourty_gentime, 100_000.0),
        exp_growth(fourty_rm, fourty_gentime * 2.0, 100_000.0),
        exp_growth(fourty_rm, fourty_gentime * 3.0, 100_000.0)
    );
    println!("gen 0: {diff_40delay_0gen}, gen 1: {diff_40delay_1gen}, gen 2: {diff_40delay_2gen}");
    println!("--50 delay--");
    println!(
        "gen 0: {}, gen 1: {}, gen 2: {}",
        get_last_egg_total(&fifty_delay, 0),
        get_last_egg_total(&fifty_delay, 1),
        get_last_egg_total(&fifty_delay, 2)
    );
    println!(
        "gen 0: {}, gen 1: {}, gen 2: {}",
        exp_growth(fifty_rm, fifty_gentime, 100_000.0),
        exp_growth(fifty_rm, fifty_gentime * 2.0, 100_000.0),
        exp_growth(fifty_rm, fifty_gentime * 3.0, 100_000.0)
    );
    println!("gen 0: {diff_50delay_0gen}, gen 1: {diff_50delay_1gen}, gen 2: {diff_50delay_2gen}");

    assert!(diff_0delay_0gen < 100.0);
    assert!(diff_0delay_1gen < 100.0);
    assert!(diff_0delay_2gen < 100.0);
    assert!(diff_40delay_0gen < 100.0);
    assert!(diff_40delay_1gen < 100.0);
    assert!(diff_40delay_2gen < 100.0);
    assert!(diff_50delay_0gen < 100.0);
    assert!(diff_50delay_1gen < 100.0);
    assert!(diff_50delay_2gen < 100.0);
}

fn generation_time() {
    let data = transpose_datapoints(simulate(
        100_000,
        0.005,
        0..=2200,
        vec![
            ProbDist::PDF(&jones_wiman_2012_0),
            ProbDist::PDF(&jones_wiman_2012_1),
            ProbDist::PDF(&jones_wiman_2012_2),
        ],
        0.0,
        1.0,
    ));
    let gen0_start = data
        .0
        .iter()
        .position(|x| *x.get(0).unwrap() > 1.0)
        .unwrap();
    let gen1_start = data
        .0
        .iter()
        .position(|x| *x.get(1).unwrap() > 1.0)
        .unwrap();
    let gen2_start = data
        .0
        .iter()
        .position(|x| *x.get(2).unwrap() > 1.0)
        .unwrap();
    println!("gen0: {gen0_start}, gen1: {gen1_start}, gen2: {gen2_start}");
    println!(
        "first diff: {}, second diff: {}",
        gen1_start - gen0_start,
        gen2_start - gen1_start
    );
}

fn transpose_datapoints(data: Vec<DataPoint>) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    // dbg!("{}", data.get(100).unwrap());
    data.into_iter()
        .fold((Vec::new(), Vec::new()), |mut acc, e| {
            acc.0.push(e.pop_active);
            acc.1.push(e.eggs);
            (acc.0, acc.1)
        })
}
