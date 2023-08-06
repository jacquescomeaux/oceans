struct Model<const N: usize, const I: usize> {
    _x_min: f64,
    _x_max: f64,
    _t_max: f64,
    //t_min == 0
    _dx: f64,
    _dt: f64,
    _c: f64,
    _r: f64,
    _u: [[f64; I]; N],
    _s: [[f64; I]; N],
}

impl<const N: usize, const I: usize> Model<N, I> {

    // fn _x_val(&self, int) -> f64 {}
    // fn _t_val(&self, int) -> f64 {}
    // fn _t_index(&self, t: f64) -> usize {}

    fn _print_info(&self) {}

    fn _set_initial_condition(&mut self) {}
    fn _clear_initial_condition(&mut self) {}

    fn _set_s(&mut self) {}
    fn _clear_s(&mut self) {}

    fn _run_ctcs(&mut self) {}
    fn _run_ftbs(&mut self) {}

    fn _write_data(&self, _: String, _: f64) {}
    fn _write_data_contour(&self, _: String) {}

}

const N: usize = 500;
const I: usize = 200;

fn main() {
    let _: Model<N, I> = Model {
        _x_min: 0.0,
        _x_max: 0.0,
        _t_max: 0.0,
        _dx: 0.0,
        _dt: 0.0,
        _c: 0.0,
        _r: 0.0,
        _u: [[0.0; I]; N],
        _s: [[0.0; I]; N],
    };
}
