mod model;

use model::Model;

fn run_case1<const N: usize, const I: usize>
    ( model: &mut Model<N, I>
    , case: &str
    ) {

    model.print_info();
    model.set_initial_condition(|x| {
        if x >= -2.0 && x <= 2.0 {
            5.0
        } else {
            0.0
        }});

    println!("Running model for case {case} CTCS:");
    model.run_ctcs();
    println!("Writing data file for t = 5");
    model.write_data(format!("case{case}_CTCS_5.txt"), 5.0);
    println!("Writing data file for t = 10");
    model.write_data(format!("case{case}_CTCS_10.txt"), 10.0);
    println!("Running model for case {case} FTBS:");
    model.run_ftbs();
    println!("Writing data file for t = 5");
    model.write_data(format!("case{case}_FTBS_5.txt"), 5.0);
    println!("Writing data file for t = 10");
    model.write_data(format!("case{case}_FTBS_10.txt"), 10.0);
    println!("");

}

fn run_case2<const N: usize, const I: usize>
    ( model: &mut Model<N, I>
    , case: &str
    ) {

    model.print_info();
    model.set_s();
    println!("Running model for case {case} CTCS:");
    model.run_ctcs();
    println!("Writing contour plot data file");
    model.write_data_contour(format!("case{case}_CTCS.txt"));
    println!("Running model for case {case} FTBS:");
    model.run_ftbs();
    println!("Writing contour plot data file");
    model.write_data_contour(format!("case{case}_FTBS.txt"));
    println!("");

}

fn main() {

    println!("Creating grid for case 1a:");
    let mut model1a: Model<400, 100> = Model::create(-50.0, 50.0, 100.0, 1.0);
    run_case1(&mut model1a, "1a");

    println!("Creating grid for case 1b:");
    let mut model1b: Model<400, 100> = Model::create(-50.0, 50.0, 100.0, 1.0);
    run_case1(&mut model1b, "1b");

    println!("Creating grid for case 1c:");
    let mut model1c: Model<200, 100> = Model::create(-50.0, 50.0, 100.0, 3.0);
    run_case1(&mut model1c, "1c");

    println!("Creating grid for case 1d:");
    let mut model1d: Model<400, 100> = Model::create(-50.0, 50.0, 100.0, -1.0);
    run_case1(&mut model1d, "1d");

    println!("Creating grid for case 2:");
    let mut model2:Model<40, 40> = Model::create(-20.0, 20.0, 20.0, 1.0);
    run_case2(&mut model2, "2");

}
