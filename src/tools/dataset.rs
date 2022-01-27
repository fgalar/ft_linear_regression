pub struct Dataset {
    pub km: Vec<f64>,
    pub price: Vec<f64>,
    pub min: f64,
    pub max: f64,
}

impl Dataset {

    pub fn get_dataset() -> Dataset
    {
        let mut rdr = csv::Reader::from_path("ressources/data.csv").unwrap();

        let mut km : Vec<f64>= Vec::new();
        let mut price : Vec<f64> = Vec::new();

        for result in rdr.records() {
            let record = result.unwrap();
            km.push(record[0].parse::<f64>().unwrap());
            price.push(record[1].parse::<f64>().unwrap());
        }

        let (min, max) = get_min_max(&km, &price);

        Dataset { km: km, price: price, min: min, max: max }
    }
}


impl Dataset {

    pub fn min_max_normalization(&self) -> (Vec<f64>, Vec<f64>)
    {
        /*
            Computes the normalized version of a non-empty vector using the min max method
            Returns:
                Tuple(data_minMax_normalized, min_dataset, max_dataset)
        */
        let mut normalized_km: Vec<f64> = Vec::with_capacity(self.km.len());
        let mut normalized_price: Vec<f64> = Vec::with_capacity(self.price.len());

        for (km_value, price_value) in self.km.iter().zip(&self.price) {

            normalized_km.push((km_value - self.min) / (self.max - self.min));
            normalized_price.push((price_value - self.min) / (self.max - self.min));

        }

        (normalized_km, normalized_price)
    }

    pub fn denormalization(&self, thetas: &Vec<f64>) -> Vec<f64>
    {
        let mut tmp : Vec<f64> = Vec::with_capacity(2);

        tmp.push(thetas[0] * (self.max - self.min) + self.min);
        tmp.push(thetas[1]);

        tmp
    }

}


pub fn get_min_max( km: &Vec<f64>, price: &Vec<f64> ) -> (f64, f64)
{
    let km_min = km.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let km_max = km.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let price_min = price.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let price_max = price.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    let min = if km_min < price_min { km_min } else { price_min };
    let max = if km_max > price_max { km_max } else { price_max };

    (min, max)
}