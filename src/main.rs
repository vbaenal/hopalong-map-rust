mod hopalong;

use hopalong::Hopalong;
use fltk::{app::*, frame::*, window::*, button::*, input::*, image::*};

fn main() {
    window();
}

fn window() {
    
    let app = App::default();
    let mut wind = Window::default()
        .with_size(810, 520)
        .center_screen()
        .with_label("Hopalong Map");

    let mut img_frame = Frame::new(300,20,500,500,"");

    let alpha_frame = Frame::default()
        .with_size(80,20)
        .with_pos(10,10)
        .with_label("Alpha");
    let alpha_input = FloatInput::default()
        .with_size(80,20)
        .right_of(&alpha_frame, 10);

    let beta_frame = Frame::default()
        .with_size(80,20)
        .below_of(&alpha_frame, 10)
        .with_label("Beta");
    let beta_input = FloatInput::default()
        .with_size(80,20)
        .right_of(&beta_frame, 10);

    let delta_frame = Frame::default()
        .with_size(80,20)
        .below_of(&beta_frame, 10)
        .with_label("Delta");
    let delta_input = FloatInput::default()
        .with_size(80,20)
        .right_of(&delta_frame, 10);

    let iters_frame = Frame::default()
        .with_size(80,20)
        .below_of(&delta_frame, 20)
        .with_label("Iterations");
    let iters_input = IntInput::default()
        .with_size(80,20)
        .right_of(&iters_frame, 10);

    let n_points_frame = Frame::default()
        .with_size(80,20)
        .below_of(&iters_frame, 10)
        .with_label("N Points");
    let n_points_input = IntInput::default()
        .with_size(80,20)
        .right_of(&n_points_frame, 10);

    let mut but_calc = Button::default()
        .with_size(180,40)
        .below_of(&n_points_frame, 10)
        .with_label("Get Hopalong");

    alpha_input.set_value("2.0");
    beta_input.set_value("-4.0");
    delta_input.set_value("5.0");
    iters_input.set_value("10000");
    n_points_input.set_value("50");

    let mut alpha: f64 = alpha_input.value().parse().unwrap();
    let mut beta: f64 = beta_input.value().parse().unwrap();
    let mut delta: f64 = delta_input.value().parse().unwrap();
    let mut iterations: usize = iters_input.value().parse().unwrap();
    let mut n_points: usize = n_points_input.value().parse().unwrap();

    let raster = get_hopalong(alpha, beta, delta, iterations, n_points);
    let image = RgbImage::new(&raster, 500, 500, 3);
    img_frame.set_image(&image);
 
    wind.make_resizable(true);
    wind.end();
    wind.show();

    but_calc.set_callback(Box::new (move || {
        alpha = alpha_input.value().parse().unwrap();
        beta = beta_input.value().parse().unwrap();
        delta = delta_input.value().parse().unwrap();
        iterations = iters_input.value().parse().unwrap();
        n_points = n_points_input.value().parse().unwrap();

        let raster = get_hopalong(alpha, beta, delta, iterations, n_points);
        let image = RgbImage::new(&raster, 500, 500, 3);
        img_frame.set_image(&image);
    }));

    app.run().unwrap();
}

fn get_hopalong(alpha: f64, beta: f64, delta: f64, iterations: usize, n_points: usize) -> Vec<u8> {

    let colors = vec!(
        (255, 225, 255),
        (255, 0, 0),
        (0, 255, 0),
        (0, 0, 255),
        (255, 255, 0),
        (255, 0, 255),
        (0, 255, 255),
        (230, 25, 75),
        (170, 255, 195),
        (230, 190, 255)
    );

    let (width, height) = (500, 500);

    let mut raster = Vec::with_capacity(width*height*3);

    for _ in 0..raster.capacity() {
        raster.push(0);
    }

    let hopa: Hopalong = Hopalong {alpha, beta, delta};

    let (xss, yss): (Vec<Vec<f64>>, Vec<Vec<f64>>) = hopa.random([-32.0f64, 32.0f64], [-32.0f64, 32.0f64], iterations, n_points);    

    let mut x_max=f64::MIN;
    let mut x_min=f64::MAX;
    let mut y_max=f64::MIN;
    let mut y_min=f64::MAX;

    for (xs, ys) in xss.iter().zip(yss.iter()) {
        for (x, y) in xs.iter().zip(ys.iter()) {
            x_max = x_max.max(*x);
            x_min = x_min.min(*x);
            y_max = y_max.max(*y);
            y_min = y_min.min(*y);
        }
    }

    let x_width = (x_max-x_min)*1.001;
    let y_width = (y_max-y_min)*1.001;

    for (xs, ys) in xss.iter().zip(yss.iter()) {
        for (i, (x, y)) in (xs.iter().zip(ys.iter())).enumerate() {
            let px: usize = ((width as f64)*(x-x_min)/x_width) as usize;
            let py: usize = ((height as f64)*(y-y_min)/y_width) as usize;
            let color = colors[i%9];
            raster[(px+py*width)*3] = color.0;   //R
            raster[(px+py*width)*3+1] = color.1; //G
            raster[(px+py*width)*3+2] = color.2; //B
        }
    }

    return raster;
}
