# monte-carlo
Rust Monte-Carlo Option Pricer


~/rust-projects/monte-carlo$ cargo build --release

   Compiling monte-carlo v0.1.0
   
    Finished release [optimized + debuginfo] target(s) in 2.50s
    
~/rust-projects/monte-carlo$ cargo run --release

    Finished release [optimized + debuginfo] target(s) in 0.03s
    
     Running `target/release/monte-carlo`
     
<b>Non-parallel Option Price is 28.41012259095407, took 22.448422604s

Number of logical cores is 4

Number of Scenarios: 1,000,000

Parallel Option Price is 28.4264239304692, took 5.584623065s</b>




