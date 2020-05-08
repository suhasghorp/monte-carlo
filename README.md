# monte-carlo
Rust Monte-Carlo Option Pricer


~/rust-projects/monte-carlo$ cargo build --release

   Compiling monte-carlo v0.1.0
   
    Finished release [optimized + debuginfo] target(s) in 2.50s
    
~/rust-projects/monte-carlo$ cargo run --release

    Finished release [optimized + debuginfo] target(s) in 0.03s
    
     Running `target/release/monte-carlo`
     
<b>Non-parallel Price is 28.48201998899803, took 2.292222318s</b>

<b>Number of logical cores is 4</b>

<b>Parallel Price is 28.488221293384285, took 613.905618ms</b>

<b>Parallel Price is 28.488221293384285, took 613.905618ms</b>


