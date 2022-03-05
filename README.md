# pandemic_simulation

A simple pandemic simulation with customizable parameters written in Rust. 

https://user-images.githubusercontent.com/75204369/156871852-c7133c73-31fc-400e-8739-307f45d052f4.mp4

Customisable parameters: 
- INFECTION_RADIUS - Radius around infected person where normal person is susceptible to infection
- INFECTION_RATE - Probability that normal person will get infected when within infection radius
- NUMBALLS - Number of people
- MAXSPEED - Maximum possible speed that people move around at (speeds and start locations are random)

Instructions: 
1. `git clone https://github.com/gursi26/pandemic_simulation.git`
2. `cd pandemic_simulation`
3. `cargo run`

Dependecies: 
- [Raylib](https://www.raylib.com)
