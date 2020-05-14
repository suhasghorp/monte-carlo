#[allow(dead_code)]

use rand::prelude::*;
use rand_distr::StandardNormal;


#[allow(dead_code)]
pub struct EquityPriceGenerator {
    init_equity_price : f64,
    num_time_steps : u64,
    time_to_expiry : f64,
    drift : f64,
    volatility : f64,
    dt : f64
}
#[allow(dead_code)]
impl EquityPriceGenerator {

    pub fn new(init_equity_price : f64,num_time_steps : u64,time_to_expiry : f64,drift : f64,volatility : f64) -> Self {
        Self {
            init_equity_price,
            num_time_steps,
            time_to_expiry,
            drift,
            volatility,
            dt : time_to_expiry/num_time_steps as f64
        }
    }

    pub fn get_path(&self) -> Result<Vec<f64>, Box<dyn std::error::Error>>  {

        let mut v : Vec<f64> = Vec::new();
        let new_price = |previous_equity_price,norm| {
            let exp_arg1:f64 = (self.drift - ((self.volatility * self.volatility) / 2.0)) * self.dt;
            let exp_arg2:f64 = self.volatility * norm * self.dt.sqrt();
            previous_equity_price * (exp_arg1 + exp_arg2).exp()
        };

        v.push(self.init_equity_price);
        let mut equity_price : f64 = self.init_equity_price;
        let rng = thread_rng();
        let rnd: Vec<f64> = StandardNormal.sample_iter(rng).take(self.num_time_steps as usize).collect();
        
        for i in 0..self.num_time_steps as usize {
            equity_price = new_price(equity_price,rnd[i]);
            v.push(equity_price);
        };

        if v.is_empty() {
            Err("Error in generating path".into())
        } else {
            Ok(v)
        }   
        
    }

}