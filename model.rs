use std::io::Write;
use std::fs::File;

struct Model<const N: usize, const I: usize> {
    x_min: f64,
    x_max: f64,
    t_max: f64,
    //t_min == 0
    dx: f64,
    dt: f64,
    c: f64,
    r: f64,
    u: [[f64; I]; N],
    s: [[f64; I]; N]
}

impl<const N: usize, const I: usize> Model<N, I> {

    fn create(x_min: f64, x_max: f64, t_max: f64, c: f64) -> Self {
        let dx = (x_max - x_min) / I as f64;
        let dt = t_max / N as f64;
        Self {
            x_min,
            x_max,
            t_max,
            dx,
            dt,
            c,
            r: (c * dt / dx),
            u: [[0.0; I]; N],
            s: [[0.0; I]; N]
        }
    }

    fn x_val(&self, i: usize) -> f64 {
        i as f64 * self.dx + self.x_min + self.dx / 2.0
    }

    fn t_val(&self, n: usize) -> f64 {
        n as f64 * self.dt + self.dt / 2.0
    }

    fn t_index(&self, t: f64) -> usize {
        (N as f64 * t / self.t_max).floor() as usize
    }

    fn print_info(&self) {
        println!("x range is [{}, {}]", self.x_min, self.x_max);
        println!("t range is [0, {}]", self.t_max);
        println!("Number of x points is {}", I);
        println!("Number of t points is {}", N);
        println!("dx is {}", self.dx);
        println!("dt is {}", self.dt);
        println!("C is {}", self.c);
        println!("r is {}", self.r);
    }

    fn set_initial_condition(&mut self) {
        for x in &mut self.u[0] {
            *x = 7.7;
        }
    }

    fn _clear_initial_condition(&mut self) {
        self.u[0].fill(0.0);
    }

    fn set_s(&mut self) {
        for n in 0..N {
            for i in 0..I {
                self.s[n][i] =
                    if self.x_val(i) >= 0.0
                        && self.x_val(i) <= 4.0
                            && self.t_val(n) <= 5.0
                            { 0.1 }
                    else { 0.0 };
            }
        }
    }

    fn _clear_s(&mut self) {
        self.s.fill([0.0; I]);
    }

    fn run_ctcs(&mut self) {

        //FTCS for the first step
        for i in 1..I-1 {
            self.u[1][i] = self.u[0][i]
                + (self.r / 2.0) * (self.u[0][i - 1] - self.u[0][i + 1])
                + self.dt * self.s[0][i];
        }

        //CTCS for subsequent steps
        for n in 1..N-1 {
            for i in 1..I-1 {
                self.u[n + 1][i] = self.u[n - 1][i]
                    + self.r * (self.u[n][i - 1] - self.u[n][i + 1])
                    + 2.0 * self.dt * self.s[n][i];
            }
        }

    }

    fn run_ftbs(&mut self) {
        for n in 0..N-1 {
            for i in 1..I {
                self.u[n + 1][i] = (1.0 - self.r) * self.u[n][i]
                    + self.r * self.u[n][i - 1]
                    + self.dt * self.s[n][i]
            }
        }
    }

    fn write_data(&self, filename: &str, t: f64) {

        let mut datafile = File::create(filename)
            .expect("Couldn't open file");

        for i in 0..I {
            writeln!(
                datafile,
                "{}, {}",
                self.x_val(i),
                self.u[self.t_index(t)][i])
                .expect("Couldn't write file");
        }

    }

    fn write_data_contour(&self, filename: &str) {

        let mut datafile = File::create(filename)
            .expect("Couldn't open file");

        for n in 0..N {
            for i in 0..I {
                writeln!(
                    datafile,
                    "{} {} {}",
                    self.t_val(n),
                    self.x_val(i),
                    self.u[n][i])
                    .expect("Couldn't write file");
            }
        }

    }

}

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
