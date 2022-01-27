extern crate itertools;
use itertools::izip;

fn sum(values: &Vec<f64>) -> f64 {
    values.iter().sum()
}

fn mean(values: &Vec<f64>) -> f64 {
   sum(&values) / values.len() as f64
}

pub fn coef_determination(values: Vec<f64>, predictions: Vec<f64>) -> f64 {
   let mut u : Vec<f64> = Vec::with_capacity(values.len());
   let mut v : Vec<f64> = Vec::with_capacity(values.len());
   let mean = mean(&values);

   for (value, prediction) in izip!(values, predictions) {
       u.push((value - prediction).powi(2));
       v.push((value - mean).powi(2));
   }

   1 as f64 - sum(&u) / sum(&v)

}