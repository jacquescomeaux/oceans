mod model;

use model::Model;

fn main() {

    // case 1a
    println!("Creating grid for case 1a:");
    let mut model1a: Model<200, 100> = Model::create(-50.0, 50.0, 100.0, 1.0);
    model1a.print_info();
    model1a.set_initial_condition();
    println!("Running model for case 1a CTCS:");
    model1a.run_ctcs();
    println!("Writing data file for t = 5");
    model1a.write_data("case1a_CTCS_5.txt", 5.0);
    println!("Writing data file for t = 10");
    model1a.write_data("case1a_CTCS_10.txt", 10.0);
    println!("Running model for case 1a FTBS:");
    model1a.run_ftbs();
    println!("Writing data file for t = 5");
    model1a.write_data("case1a_FTBS_5.txt", 5.0);
    println!("Writing data file for t = 10");
    model1a.write_data("case1a_FTBS_10.txt", 10.0);
    println!("");

    // case 1b
    println!("Creating grid for case 1b:");
    let mut model1b: Model<400, 100> = Model::create(-50.0, 50.0, 100.0, 1.0);
    model1b.print_info();
    model1b.set_initial_condition();
    println!("Running model for case 1b CTCS:");
    model1b.run_ctcs();
    println!("Writing data file for t = 5");
    model1b.write_data("case1b_CTCS_5.txt", 5.0);
    println!("Writing data file for t = 10");
    model1b.write_data("case1b_CTCS_10.txt", 10.0);
    println!("Running model for case 1b FTBS:");
    model1b.run_ftbs();
    println!("Writing data file for t = 5");
    model1b.write_data("case1b_FTBS_5.txt", 5.0);
    println!("Writing data file for t = 10");
    model1b.write_data("case1b_FTBS_10.txt", 10.0);
    println!("");

    // case 1c
    println!("Creating grid for case 1c:");
    let mut model1c: Model<200, 100> = Model::create(-50.0, 50.0, 100.0, 3.0);
    model1c.print_info();
    model1c.set_initial_condition();
    println!("Running model for case 1c CTCS:");
    model1c.run_ctcs();
    println!("Writing data file for t = 5");
    model1c.write_data("case1c_CTCS_5.txt", 5.0);
    println!("Writing data file for t = 10");
    model1c.write_data("case1c_CTCS_10.txt", 10.0);
    println!("Running model for case 1c FTBS:");
    model1c.run_ftbs();
    println!("Writing data file for t = 5");
    model1c.write_data("case1c_FTBS_5.txt", 5.0);
    println!("Writing data file for t = 10");
    model1c.write_data("case1c_FTBS_10.txt", 10.0);
    println!("");

    // case 1d
    println!("Creating grid for case 1d:");
    let mut model1d: Model<400, 100> = Model::create(-50.0, 50.0, 100.0, -1.0);
    model1d.print_info();
    model1d.set_initial_condition();
    println!("Running model for case 1d CTCS:");
    model1d.run_ctcs();
    println!("Writing data file for t = 5");
    model1d.write_data("case1d_CTCS_5.txt", 5.0);
    println!("Writing data file for t = 10");
    model1d.write_data("case1d_CTCS_10.txt", 10.0);
    println!("Running model for case 1d FTBS:");
    model1d.run_ftbs();
    println!("Writing data file for t = 5");
    model1d.write_data("case1d_FTBS_5.txt", 5.0);
    println!("Writing data file for t = 10");
    model1d.write_data("case1d_FTBS_10.txt", 10.0);
    println!("");

    // case 2
    println!("Creating grid for case 2:");
    let mut model2:Model<40, 40> = Model::create(-20.0, 20.0, 20.0, 1.0);
    model2.print_info();
    model2.set_s();
    println!("Running model for case 2 CTCS:");
    model2.run_ctcs();
    println!("Writing contour plot data file");
    model2.write_data_contour("case2_CTCS.txt");
    println!("Running model for case 2 FTBS:");
    model2.run_ftbs();
    println!("Writing contour plot data file");
    model2.write_data_contour("case2_FTBS.txt");
    println!("");

}
