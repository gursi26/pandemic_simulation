extern crate raylib;
use rand::Rng;
use raylib::prelude::*;
use std::time::Instant;

const TARGETFPS: u32 = 50;

// ball params
const RADIUS: f32 = 5.0;
const NUMBALLS: i32 = 1500;
const MAXSPEED: i32 = 3;

// colors 
const INFECTED_COLOR: Color = Color::RED;
const NORMAL_COLOR: Color = Color::GREEN;
const RECOVERED_COLOR: Color = Color::DARKBLUE;
const BG_COLOR: Color = Color::WHITE;

// infection params
const INFECTION_RADIUS: f32 = 2.5;
const INFECTION_RATE: f32 = 0.25;
const BASE_RECOVERY_TIME: i32 = 5; // time in seconds for recovery/death
const RECOVERY_TIME_RANGE: i32 = 2; // recovery_time = BASE_RECOVERY_TIME + rand(-RECOVERY_TIME_RANGE, RECOVERY_TIME_RANGE)

#[derive(Clone, Copy)]
struct Ball {
    pos: Vector2,
    speed: Vector2,
    time_infected: i32
}

impl Ball {
    fn new_ball(height: i32, width: i32) -> Ball {
        let posx = rand::thread_rng().gen_range(RADIUS as i32..(width - RADIUS as i32)) as f32;
        let posy = rand::thread_rng().gen_range(RADIUS as i32..(height - RADIUS as i32)) as f32;
        let mut speedx = rand::thread_rng().gen_range(-MAXSPEED..MAXSPEED) as f32;
        let mut speedy = rand::thread_rng().gen_range(-MAXSPEED..MAXSPEED) as f32;

        if speedx == 0.0 {speedx += 1.0;};
        if speedy == 0.0 {speedy += 1.0;};

        Ball {
            pos: Vector2::new(posx, posy),
            speed: Vector2::new(speedx, speedy),
            time_infected: 0
        }
    }

    fn populate(
        height: i32,
        width: i32,
        infected_arr: &mut Vec<Ball>,
        normal_arr: &mut Vec<Ball>
    ) {
        for _i in 0..(NUMBALLS - 1) {
            let newball = Ball::new_ball(height, width);
            normal_arr.push(newball);
        }
        let newball = Ball::new_ball(height, width);
        infected_arr.push(newball);
    }

    fn infection_check(
        infected_arr: &mut Vec<Ball>,
        normal_arr: &mut Vec<Ball>,
        recovered_arr: &mut Vec<Ball>,
        mut i: i32, 
        mut outer_loop_end: i32,
    ) -> (i32, i32) { 
        let mut j: i32 = 0;
        let mut end_idx: i32 = normal_arr.len() as i32;
        while j < end_idx {
            if collision::check_collision_circles(
                infected_arr[i as usize].pos,
                RADIUS + INFECTION_RADIUS,
                normal_arr[j as usize].pos,
                RADIUS,
            ) {
                let random = rand::thread_rng().gen_range(1..100) as f32 / 100.0;
                if random < INFECTION_RATE {
                    infected_arr.push(normal_arr.remove(j as usize));
                    j -= 1;
                    end_idx -= 1;
                }
            }
            j += 1;
        }
        let recovery_time: i32 = BASE_RECOVERY_TIME + rand::thread_rng().gen_range(-RECOVERY_TIME_RANGE..RECOVERY_TIME_RANGE);
        if infected_arr[i as usize].time_infected >= recovery_time * TARGETFPS as i32 {
            recovered_arr.push(infected_arr.remove(i as usize));
            i -= 1;
            outer_loop_end -= 1;
        } else {
            infected_arr[i as usize].time_infected += 1;
        }
        (i, outer_loop_end)
    }

    fn wall_collision(&mut self, width: i32, height: i32) {
        if (self.pos.x + RADIUS) > (width as f32) || (self.pos.x - RADIUS) < (0 as f32) {
            self.speed.x *= -1.0;
        }
        if (self.pos.y + RADIUS) > (height as f32) || (self.pos.y - RADIUS) < (0 as f32) {
            self.speed.y *= -1.0;
        }
    }
}

fn draw_stats(d: &mut RaylibDrawHandle, infected_len: usize, normal_len: usize, recovered_len: usize) {
        d.draw_rectangle(0, 0, 170, 110, Color::DARKBLUE);
        d.draw_rectangle(0, 0, 165, 105, Color::WHITE);
        let fps: String = d.get_fps().to_string();
        let num_infected: String = infected_len.to_string();
        let num_normal: String = normal_len.to_string();
        let num_recovered: String = recovered_len.to_string();

        let mut fps_string: String = "FPS : ".to_owned();
        let mut normal: String = "Normal : ".to_owned();
        let mut infected: String = "Infected: ".to_owned();
        let mut recovered: String = "Recovered: ".to_owned();
        normal.push_str(&num_normal);
        infected.push_str(&num_infected);
        recovered.push_str(&num_recovered);
        fps_string.push_str(&fps);

        d.draw_text(&fps_string, 5, 5, 20, Color::BLACK);
        d.draw_text(&normal, 5, 30, 20, NORMAL_COLOR);
        d.draw_text(&infected, 5, 55, 20, INFECTED_COLOR);
        d.draw_text(&recovered, 5, 80, 20, RECOVERED_COLOR);
}

fn main() {
    let (mut rl, thread) = init().size(1440, 900).fullscreen().title("Pandemic Simulation").build();
    //let (mut rl, thread) = init().size(1420, 827).resizable().title("Pandemic Simulation").build();

    let (mut width, mut height) = (rl.get_screen_width(), rl.get_screen_height());
    rl.set_target_fps(TARGETFPS);

    let mut infected_arr: Vec<Ball> = Vec::new();
    let mut normal_arr: Vec<Ball> = Vec::new();
    let mut recovered_arr: Vec<Ball> = Vec::new();
    Ball::populate(height, width, &mut infected_arr, &mut normal_arr);

    while !rl.window_should_close() {
        let start = Instant::now(); 

        (width, height) = (rl.get_screen_width(), rl.get_screen_height());
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(BG_COLOR);

        let mut i: i32 = 0; 
        let mut end_idx: i32 = infected_arr.len() as i32;
        while i < end_idx {
            d.draw_circle_v(infected_arr[i as usize].pos, RADIUS + 1.0, INFECTED_COLOR);
            d.draw_circle_v(infected_arr[i as usize].pos, RADIUS - 1.0, Color::WHITE);

            // updating positions
            infected_arr[i as usize].pos.x += infected_arr[i as usize].speed.x;
            infected_arr[i as usize].pos.y += infected_arr[i as usize].speed.y;

            // wall collision check and update
            infected_arr[i as usize].wall_collision(width, height);

            (i, end_idx) = Ball::infection_check(&mut infected_arr, &mut normal_arr, &mut recovered_arr, i as i32, end_idx as i32);
            i += 1;
        }

        // drawing normal balls
        for i in 0..normal_arr.len() {
            let ball = &mut normal_arr[i as usize];
            d.draw_circle_v(ball.pos, RADIUS, NORMAL_COLOR);

            // updating positions
            ball.pos.x += ball.speed.x;
            ball.pos.y += ball.speed.y;

            // wall collision check and update
            ball.wall_collision(width, height);
        }

        // drawing recovered particles
        for i in 0..recovered_arr.len() {
            let ball = &mut recovered_arr[i as usize];
            d.draw_circle_v(ball.pos, RADIUS, RECOVERED_COLOR);

            // updating positions
            ball.pos.x += ball.speed.x;
            ball.pos.y += ball.speed.y;

            // wall collision check and update
            ball.wall_collision(width, height);
        }

        draw_stats(&mut d, infected_arr.len(), normal_arr.len(), recovered_arr.len());

        let duration = start.elapsed();
        println!("Render time for frame: {:?}", duration);
    }
}