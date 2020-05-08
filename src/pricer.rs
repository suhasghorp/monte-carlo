

use crate::epg;
use std::process;
use rayon::prelude::*;

#[allow(dead_code)]
pub enum OptionType {
    Call,
    Put,
}

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
    pub fn compute_price(&self) -> f64 {
        match self.run_parallel {
            true => {self.compute_price_parallel()},
            false => {self.compute_price_non_parallel()}
        }
    }

    fn payoff (&self, porc:&OptionType, terminal_price:&f64,strike:&f64) -> f64 {
        match porc {
            OptionType::Call => (terminal_price - strike).max(0.0),
            OptionType::Put => (strike - terminal_price).max(0.0)
        }
    }

    fn get_one_discounted_payoff(&self) -> f64 {
        let path : Vec<f64> = self.epg.get_path();
        let terminal_price = path.last().unwrap_or_else(|| {eprintln!("Price path was not generated"); process::exit(-1);});
        let payoff:f64 = self.payoff(&self.porc, &terminal_price,&self.strike);
        self.disc_factor() * payoff
    }

    fn compute_price_parallel(&self) -> f64 {

        let discounted_payoffs = (0..self.num_scenarios).into_par_iter().map( 
            |_| {           
                self.get_one_discounted_payoff()
                }
        ).collect::<Vec<f64>>();

        self.quantity as f64 * (1.0/self.num_scenarios as f64) * discounted_payoffs.into_iter().sum::<f64>()
    }

    fn compute_price_non_parallel(&self) -> f64 {

        let discounted_payoffs = (0..self.num_scenarios).into_iter().map( 
            |_| {           
                self.get_one_discounted_payoff()
                }
        ).collect::<Vec<f64>>();

        self.quantity as f64 * (1.0/self.num_scenarios as f64) * discounted_payoffs.into_iter().sum::<f64>()
    }

}