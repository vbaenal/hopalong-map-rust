use rand::Rng;

pub struct Hopalong {
    pub alpha: f64,
    pub beta: f64,
    pub delta: f64,
}

impl Hopalong {
    pub fn random(
        &self,
        x_range: [f64; 2],
        y_range: [f64; 2],
        iterations: usize,
        n_points: usize,
    ) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
        let mut rng = rand::thread_rng();
        let mut xx: Vec<f64> = Vec::with_capacity(n_points);
        let mut yy: Vec<f64> = Vec::with_capacity(n_points);

        for _ in 0..n_points {
            xx.push(rng.gen_range(x_range[0], x_range[1]));
            yy.push(rng.gen_range(y_range[0], y_range[1]));
        }

        let mut xss: Vec<Vec<f64>> = Vec::with_capacity(iterations + 1);
        let mut yss: Vec<Vec<f64>> = Vec::with_capacity(iterations + 1);

        xss.push(xx);
        yss.push(yy);

        return self.iterate(xss, yss, iterations, n_points);
    }

    fn iterate(
        &self,
        mut xss: Vec<Vec<f64>>,
        mut yss: Vec<Vec<f64>>,
        iterations: usize,
        n_points: usize,
    ) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
        for i in 0..iterations {
            let mut xx: Vec<f64> = Vec::with_capacity(n_points);
            let mut yy: Vec<f64> = Vec::with_capacity(n_points);
            for j in 0..n_points {
                xx.push(self.x_function(&xss[i][j], &yss[i][j]));
                yy.push(self.y_function(&xss[i][j]));
            }
            xss.push(xx);
            yss.push(yy);
        }
        return (xss, yss);
    }

    fn x_function(&self, x: &f64, y: &f64) -> f64 {
        return y - x.signum() * ((self.beta * x - self.delta).abs().sqrt());
    }

    fn y_function(&self, x: &f64) -> f64 {
        return self.alpha - x;
    }
}
