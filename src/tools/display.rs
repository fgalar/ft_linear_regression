extern crate plotly;
use super::dataset::{ Dataset };
use plotly::{Scatter, Plot};
use plotly::common::{Mode};


pub fn schemas(data: &Dataset, cost_history: &Vec<f64>, prediction: &Vec<f64>)
{
    display_dataset(data, prediction);
    display_learning_rate(cost_history);
}


fn display_dataset(data: &Dataset, prediction: &Vec<f64>)
{
    let mut plot = Plot::new();

    let point = Scatter::new(data.km.clone(), data.price.clone())
        .name("Real")
        .mode(Mode::Markers);

    let line = Scatter::new(data.km.clone(), prediction.clone())
        .name("Linear Regression")
        .mode(Mode::Lines);

    plot.add_trace(point);
    plot.add_trace(line);

    plot.show();
}


fn display_learning_rate(cost_history: &Vec<f64>)
{
    let mut plot = Plot::new();

    let cost = Scatter::new(0..1000, cost_history.clone())
            .mode(Mode::Lines)
            .name("lines");

    plot.add_trace(cost);

    plot.show();
}