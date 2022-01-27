extern crate itertools;
use itertools::izip;

mod tools;
use tools::csv;
use tools::dataset::Dataset;
use tools::display;
use tools::math::{coef_determination};


fn cost_function(x: &Vec<f64>, y: &Vec<f64>, theta: &Vec<f64>) -> f64
{
    let m: usize = x.len();
    let mut err_cost: f64 = 0.0;
    let estimate_costs = model(x, theta);

    for (estimate_cost, real_cost) in izip!(estimate_costs, y) {
        err_cost = err_cost + (estimate_cost - real_cost).powi(2);
    }

    1.0 / ((2 * m) as f64) * err_cost
}


fn model(km: &Vec<f64>, theta: &Vec<f64>) -> Vec<f64>
{
    let mut estimate: Vec<f64> = Vec::with_capacity(km.len());

    for mileage in km {
        estimate.push(
            theta[0]  + (theta[1]* mileage)
        );
    }

    estimate
}


fn grad(x: &Vec<f64>, y: &Vec<f64>, theta: &Vec<f64>) -> Vec<f64>
{
    let m: usize = x.len();
    let estimate_costs = model(&x, &theta);
    let mut t0 = 0.0;
    let mut t1 = 0.0;

    for (estimate_cost, real_cost, km) in izip!(&estimate_costs, y, x){
        t0 = t0 + (estimate_cost - real_cost);
        t1 = t1 + ((estimate_cost - real_cost) * km);
    }

    vec![ 1.0 / (m as f64) * t0 ,
          1.0 / (m as f64) * t1 ]
}

fn gradient_descent(x: &Vec<f64>, y: &Vec<f64>, mut theta: Vec<f64>,
                        learning_rate: f64, n_iterations: i32) -> (Vec<f64>, Vec<f64>){
    let mut cost_history: Vec<f64> = Vec::with_capacity(y.len());

    for _i in 0..n_iterations {
        let mut tmp : Vec<f64> = Vec::with_capacity(2);

        for (grad, t) in izip!(grad(x, y, &theta), theta.clone()) {
            tmp.push(t - learning_rate * grad);
        }

        theta = vec![tmp[0], tmp[1]];
        cost_history.push(cost_function(x, y, &theta));
    }
    (theta, cost_history)
}

fn main()
{
    let data = Dataset::get_dataset();
    let theta = vec!(0.0, 0.0);
    let (x, y) = data.min_max_normalization();

    let (mut theta_final, cost_history) = gradient_descent(&x, &y, theta.clone(), 0.2, 1000);
    theta_final = data.denormalization(&theta_final);

    csv::update_theta(&theta_final);

    let prediction = model(&data.km, &theta_final);
    display::schemas(&data, &cost_history, &prediction);
    println!("{}", coef_determination(data.price, prediction));
}