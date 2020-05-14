

use crate::epg;
use rayon::prelude::*;
use std::error::Error;
use std::fmt;

#[allow(dead_code)]
pub enum OptionType {
    Call,
    Put,
}

#[derive(Debug)]
struct PricerError(String);

impl fmt::Display for PricerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is a pricing error: {}", self.0)
    }
}

impl Error for PricerError {}

#[allow(dead_code)]
pub struct MCEuroOptPricer {
    strike : f64,
    spot : f64,
    risk_free_rate : f64,
    volatility : f64,
    time_to_expiry : f64,
    porc : OptionType,
    num_time_steps : u64,
    num_scenarios : u64,
    run_parallel : bool,
    init_seed : u64,
    quantity : u64,
    epg : epg::EquityPriceGenerator
}
#[allow(dead_code)]
impl MCEuroOptPricer {
    pub fn new(strike : f64,spot : f64,risk_free_rate : f64,volatility : f64,time_to_expiry : f64,porc : OptionType,num_time_steps : u64,num_scenarios : u64,
                run_parallel : bool,init_seed : u64,quantity : u64) -> Self {

        Self {
            strike,
            spot,
            risk_free_rate,
            volatility,
            time_to_expiry,
            porc,
            num_time_steps,
            num_scenarios,
            run_parallel,
            init_seed,
            quantity,
            epg : epg::EquityPriceGenerator::new(spot, num_time_steps, time_to_expiry, risk_free_rate, volatility)
        }
    }
    fn disc_factor(&self) -> f64 {
        (-self.risk_free_rate * self.time_to_expiry).exp()
    }
    pub fn compute_price(&self) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
        match self.run_parallel {
            true => {Ok(self.compute_price_parallel()?)},
            false => {Ok(self.compute_price_non_parallel()?)}
        }
    }

    fn payoff (&self, porc:&OptionType, terminal_price:&f64,strike:&f64) -> f64 {
        match porc {
            OptionType::Call => (terminal_price - strike).max(0.0),
            OptionType::Put => (strike - terminal_price).max(0.0)
        }
    }

    fn get_one_discounted_payoff(&self) -> Result<f64, Box<dyn std::error::Error + Send>> {
        //let path = self.epg.get_path()?;
        let path = match self.epg.get_path(){
            Ok(p) => p,
            Err(error) => panic!("Problem generating path: {:?}", error)
        };
        let terminal_price = path.last().unwrap();
        let payoff:f64 = self.payoff(&self.porc, &terminal_price,&self.strike);
        Ok(self.disc_factor() * payoff)
        
    }

    fn compute_price_parallel(&self) -> Result<f64, Box<dyn Error + Send + Sync>>  {

        let discounted_payoffs = (0..self.num_scenarios).into_par_iter().map( 
            |_| {           
                    match self.get_one_discounted_payoff() {
                        Ok(payoff) => {Ok(payoff)},
                        Err(e) => Err(e)
                    }
                }
        ).collect::<Result<Vec<_>, _>>();

        match discounted_payoffs {
            Ok(v) => Ok(self.quantity as f64 * (1.0/self.num_scenarios as f64) * v.into_iter().sum::<f64>()),
            Err(_e) => Err(format!("There was an error in calculating Parallel Discounted Payoffs.{}",_e).into())
        }
        
    }

    fn compute_price_non_parallel(&self) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {

        let discounted_payoffs = (0..self.num_scenarios).into_iter().map( 
            |_| {           
                self.get_one_discounted_payoff()
                }
        ).collect::<Result<Vec<_>, _>>();
            
        match discounted_payoffs {
            Ok(v) => Ok(self.quantity as f64 * (1.0/self.num_scenarios as f64) * v.into_iter().sum::<f64>()),
            Err(_e) => Err(From::from("There was an error in calculating Non-Parallel Discounted Payoffs."))
        }
               
    }

}