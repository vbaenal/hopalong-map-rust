use rand::Rng;

static mut A: f64 = 0.;
static mut B: f64 = 0.;
static mut D: f64 = 0.;

pub fn hopalong(alpha:f64, beta:f64, delta:f64, mut n_points:usize, iterations:usize, xx:Option<Vec<f64>>, yy:Option<Vec<f64>>) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    unsafe {
        A = alpha;
        B = beta;
        D = delta;
    }
    
    let mut rng = rand::thread_rng();
    let mut xs: Vec<f64> = Vec::with_capacity(n_points);
    let mut ys: Vec<f64> = Vec::with_capacity(n_points);
    match xx {
        None => for _ in 0..n_points {
            xs.push(rng.gen_range(-10.0,10.0));
        },
        Some(p) => {
            xs = p;
            n_points = xs.capacity();
        } 
    }
    match yy {
        None => for _ in 0..n_points {
            ys.push(rng.gen_range(-10.0,10.0));
        },
        Some(p) => {
            ys = p;
            n_points = ys.capacity();
        } 
    }
    let mut xss: Vec<Vec<f64>> = Vec::with_capacity(iterations+1);
    let mut yss: Vec<Vec<f64>> = Vec::with_capacity(iterations+1);

    xss.insert(0, xs);
    yss.insert(0, ys);

    return iterate(xss, yss, iterations, n_points);
}

fn iterate(mut xss: Vec<Vec<f64>>, mut yss: Vec<Vec<f64>>, iterations: usize, n_points: usize) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    for i in 0..iterations {
        let mut xx: Vec<f64> = Vec::with_capacity(n_points);
        let mut yy: Vec<f64> = Vec::with_capacity(n_points);
        for j in 0..n_points {
            xx.insert(j, x_function(xss[i][j], yss[i][j]));
            yy.insert(j, y_function(xss[i][j], yss[i][j]));
        }
        xss.insert(i+1, xx);
        yss.insert(i+1, yy);
    }
    return (xss, yss);
}

fn x_function(x:f64, y:f64) -> f64 {
    let val: f64;
    unsafe {val = B*x-D;}
    return y-x.signum()*val.abs().sqrt();
}

fn y_function(x: f64, _y: f64) -> f64 {
    unsafe {return A-x;}
}