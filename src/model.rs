use std::io::Write;
use std::fs::File;

pub struct Model<const N: usize, const I: usize> {
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

    pub fn create(x_min: f64, x_max: f64, t_max: f64, c: f64) -> Self {
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

    pub fn print_info(&self) {
        println!("x range is [{}, {}]", self.x_min, self.x_max);
        println!("t range is [0, {}]", self.t_max);
        println!("Number of x points is {}", I);
        println!("Number of t points is {}", N);
        println!("dx is {}", self.dx);
        println!("dt is {}", self.dt);
        println!("C is {}", self.c);
        println!("r is {}", self.r);
    }

    pub fn set_initial_condition(&mut self) {
        for x in &mut self.u[0] {
            *x = 7.7;
        }
    }

    fn _clear_initial_condition(&mut self) {
        self.u[0].fill(0.0);
    }

    pub fn set_s(&mut self) {
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

    pub fn run_ctcs(&mut self) {

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

    pub fn run_ftbs(&mut self) {
        for n in 0..N-1 {
            for i in 1..I {
                self.u[n + 1][i] = (1.0 - self.r) * self.u[n][i]
                    + self.r * self.u[n][i - 1]
                    + self.dt * self.s[n][i]
            }
        }
    }

    pub fn write_data(&self, filename: &str, t: f64) {

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

    pub fn write_data_contour(&self, filename: &str) {

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
