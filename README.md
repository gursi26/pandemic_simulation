# pandemic_simulation

A simple pandemic simulation with customizable parameters written in Rust, and rendered with [Raylib](https://www.raylib.com). 

https://user-images.githubusercontent.com/75204369/162750625-4df57e3d-0c02-4b5a-9c16-b510ff232b89.mp4

Customisable parameters: 
- `INFECTION_RADIUS` - Radius around infected person where normal person is susceptible to infection
- `INFECTION_RATE` - Probability that normal person will get infected when within infection radius
- `NUMBALLS` - Number of people
- `INITIAL_INFECTED_POPULATION` - Number of people infected at start of simualtion
- `MAXSPEED` - Maximum possible speed that people move around at (speeds and start locations are random)
- `BASE_RECOVERY_TIME` - Base time for recovery
- `RECOVERY_TIME_RANGE` - `recovery_time = BASE_RECOVERY_TIME + rand(-RECOVERY_TIME_RANGE, RECOVERY_TIME_RANGE)`, accounts for natural variation in recovery times.
- `FATALITY_RATE` - Percentage of people that would die after `recovery_time` has elapsed. Remaining will recover.

Instructions: 
1. `git clone https://github.com/gursi26/pandemic_simulation.git`
2. `cd pandemic_simulation`
3. `cargo run`

Dependecies: 
- [raylib-rs](https://crates.io/crates/raylib)

TODOs
- [ ] Use grid rendering system for better performance with higher number of objects
- [ ] Other, more optimised collision detection methods
- [ ] Include phenomena like
  - [ ] Social distancing
  - [ ] Lockdown with frequent trips to a central location (like a grocery store)
  - [ ] Separate communities with travel between them
  - [ ] Quarantine zones for infected patients
  - [ ] Asymptomatic cases
  - [X] Fatality rate
  - [X] Recovery rate
  - [ ] Virus mutating into a stronger variant (more contagious or more deadly)
