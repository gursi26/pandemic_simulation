# pandemic_simulation

A simple pandemic simulation with customizable parameters written in Rust, rendered with [Raylib](https://www.raylib.com). 

https://user-images.githubusercontent.com/75204369/156892046-7c64ba58-c295-4c62-9b74-b0ea915d30fb.mp4

Customisable parameters: 
- INFECTION_RADIUS - Radius around infected person where normal person is susceptible to infection
- INFECTION_RATE - Probability that normal person will get infected when within infection radius
- NUMBALLS - Number of people
- MAXSPEED - Maximum possible speed that people move around at (speeds and start locations are random)
- BASE_RECOVERY_TIME - Base time for recovery
- RECOVERY_TIME_RANGE - `recovery_time = BASE_RECOVERY_TIME + rand(-RECOVERY_TIME_RANGE, RECOVERY_TIME_RANGE)`, accounts for natural variation in recovery times.
- FATALITY_RATE - Percentage of people that would die after `recovery_time` has elapsed. Remaining will recover.

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
