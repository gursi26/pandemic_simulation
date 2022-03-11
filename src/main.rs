extern crate raylib;
use raylib::prelude::*;
use rand::Rng;
use std::ffi::CString;



const TARGETFPS: u32 = 50;

// ball params
const RADIUS: f32 = 5.0;
const NUMBALLS: i32 = 800;
const MAXSPEED: i32 = 3;

// colors 
const INFECTED_COLOR: Color = Color::RED;
const NORMAL_COLOR: Color = Color::GREEN;
const RECOVERED_COLOR: Color = Color::DARKBLUE;
const DEAD_COLOR: Color = Color::LIGHTGRAY;
const BG_COLOR: Color = Color::WHITE;

const WIDTH: i32 = 1440; 
const HEIGHT: i32 = 900;
const STARTX: i32 = 250; 
const STARTY: i32 = 0;
const DEAD_BOX_START_Y: i32 = 105;
const DEAD_BOX_HEIGHT: i32 = 200;
const LINE_WIDTH: i32 = 3;

// infection params
const INFECTION_RADIUS: f32 = 1.5;
const INFECTION_RATE: f32 = 0.1;
const INITIAL_INFECTED_POPULATION: i32 = 5;
const BASE_RECOVERY_TIME: i32 = 4; // time in seconds for recovery/death
const RECOVERY_TIME_RANGE: i32 = 2; // recovery_time = BASE_RECOVERY_TIME + rand(-RECOVERY_TIME_RANGE, RECOVERY_TIME_RANGE)
const FATALITY_RATE: f32 = 0.01;



#[derive(Clone, Copy)]
struct Ball {
    pos: Vector2,
    speed: Vector2,
    time_infected: i32,
    time_to_recovery: i32,
    will_die: bool,
}

impl Ball {

    fn new_ball(startx: i32, starty: i32, endx: i32, endy: i32) -> Ball {
        let posx = rand::thread_rng().gen_range(startx + RADIUS as i32..(endx - RADIUS as i32)) as f32;
        let posy = rand::thread_rng().gen_range(starty + RADIUS as i32..(endy - RADIUS as i32)) as f32;
        let mut speedx = rand::thread_rng().gen_range(-MAXSPEED..MAXSPEED) as f32;
        let mut speedy = rand::thread_rng().gen_range(-MAXSPEED..MAXSPEED) as f32;

        if speedx == 0.0 {speedx += 1.0;};
        if speedy == 0.0 {speedy += 1.0;};

        Ball {
            pos: Vector2::new(posx, posy),
            speed: Vector2::new(speedx, speedy),
            time_infected: 0,
            time_to_recovery: (BASE_RECOVERY_TIME + rand::thread_rng().gen_range(-RECOVERY_TIME_RANGE..RECOVERY_TIME_RANGE)) * TARGETFPS as i32,
            will_die: (rand::thread_rng().gen_range(0..100) as f32 / 100.0) < FATALITY_RATE,
        }
    }

    fn populate(
        startx: i32,
        starty: i32,
        endx: i32,
        endy: i32,
        infected_arr: &mut Vec<Ball>,
        normal_arr: &mut Vec<Ball>,
        initial_infected: i32,
    ) {
        for _i in 0..(NUMBALLS - initial_infected) {
            let newball = Ball::new_ball(startx, starty, endx, endy);
            normal_arr.push(newball);
        }
        for _i in 0..initial_infected {
            let newball = Ball::new_ball(startx, starty, endx, endy);
            infected_arr.push(newball);
        }
    }

    fn update_position(&mut self) {
        self.pos.x += self.speed.x;
        self.pos.y += self.speed.y;
    }

    fn infection_check(
        infected_arr: &mut Vec<Ball>,
        normal_arr: &mut Vec<Ball>,
        recovered_arr: &mut Vec<Ball>,
        dead_arr: &mut Vec<Ball>,
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
                    let newly_infected = normal_arr.remove(j as usize);
                    infected_arr.push(newly_infected);
                    j -= 1;
                    end_idx -= 1;
                }
            }
            j += 1;
        };
        if infected_arr[i as usize].time_infected >= infected_arr[i as usize].time_to_recovery {
            if infected_arr[i as usize].will_die {
                let mut newly_dead = infected_arr.remove(i as usize);
                newly_dead.pos.x = rand::thread_rng().gen_range(RADIUS as i32..STARTX - RADIUS as i32) as f32;
                newly_dead.pos.y = rand::thread_rng().gen_range(DEAD_BOX_START_Y + 45 as i32..DEAD_BOX_START_Y + DEAD_BOX_HEIGHT - RADIUS as i32) as f32;
                dead_arr.push(newly_dead);
            } else {
                recovered_arr.push(infected_arr.remove(i as usize));
            }
            i -= 1;
            outer_loop_end -= 1;
        } else {
            infected_arr[i as usize].time_infected += 1;
        }
        (i, outer_loop_end)
    }

    fn wall_collision(&mut self, startx: i32, starty: i32, endx: i32, endy: i32) {
        if (self.pos.x + RADIUS) > (endx as f32) || (self.pos.x - RADIUS) < (startx as f32) {
            self.speed.x *= -1.0;
        }
        if (self.pos.y + RADIUS) > (endy as f32) || (self.pos.y - RADIUS) < (starty as f32) {
            self.speed.y *= -1.0;
        }
    }
}

fn draw_stats(d: &mut RaylibDrawHandle, infected_len: usize, normal_len: usize, recovered_len: usize) {
        d.draw_line_ex(Vector2::new((STARTX - LINE_WIDTH) as f32, 0.0), Vector2::new((STARTX - LINE_WIDTH) as f32, WIDTH as f32), LINE_WIDTH as f32, Color::BLACK);
        d.draw_line_ex(Vector2::new(0.0, DEAD_BOX_START_Y as f32), Vector2::new((STARTX - LINE_WIDTH) as f32, DEAD_BOX_START_Y as f32), LINE_WIDTH as f32, Color::BLACK);
        let fps: String = d.get_fps().to_string();
        let num_infected: String = infected_len.to_string();
        let num_normal: String = normal_len.to_string();
        let num_recovered: String = recovered_len.to_string();

        let mut fps_string: String = "FPS : ".to_owned();
        let mut normal: String = "Normal   : ".to_owned();
        let mut infected: String = "Infected   : ".to_owned();
        let mut recovered: String = "Recovered   : ".to_owned();
        normal.push_str(&num_normal);
        infected.push_str(&num_infected);
        recovered.push_str(&num_recovered);
        fps_string.push_str(&fps);

        d.draw_text(&fps_string, 5, 5, 20, Color::BLACK);

        d.draw_text(&normal, 5, 30, 20, NORMAL_COLOR);
        d.draw_circle(84, 40, RADIUS, NORMAL_COLOR);

        d.draw_text(&infected, 5, 55, 20, INFECTED_COLOR);
        d.draw_circle(105, 65, RADIUS + 1.0, INFECTED_COLOR);
        d.draw_circle(105, 65, RADIUS - 1.0, Color::WHITE);

        d.draw_text(&recovered, 5, 80, 20, RECOVERED_COLOR);
        d.draw_circle(126, 91, RADIUS, RECOVERED_COLOR);
}

fn draw_dead_box(d: &mut RaylibDrawHandle, dead_len: usize) {
    d.draw_line_ex(Vector2::new(0.0, DEAD_BOX_START_Y as f32), Vector2::new((STARTX - LINE_WIDTH) as f32, DEAD_BOX_START_Y as f32), LINE_WIDTH as f32, Color::BLACK);
    d.draw_line_ex(Vector2::new(0.0, (DEAD_BOX_START_Y + DEAD_BOX_HEIGHT) as f32), Vector2::new((STARTX - LINE_WIDTH) as f32, (DEAD_BOX_START_Y + DEAD_BOX_HEIGHT) as f32), LINE_WIDTH as f32, Color::BLACK);

    let num_dead: String = dead_len.to_string();
    let mut dead_string: String = "Dead   : ".to_owned();
    dead_string.push_str(&num_dead);
    d.draw_text(&dead_string, 5, DEAD_BOX_START_Y + 5, 20, Color::BLACK);
    d.draw_circle(68, DEAD_BOX_START_Y + 15, RADIUS, DEAD_COLOR);
}



fn main() {
    let (mut rl, thread) = init().size(WIDTH, HEIGHT).fullscreen().title("Pandemic Simulation").build();
    rl.set_target_fps(TARGETFPS);

    let mut infected_arr: Vec<Ball> = Vec::new();
    let mut normal_arr: Vec<Ball> = Vec::new();
    let mut recovered_arr: Vec<Ball> = Vec::new();
    let mut dead_arr: Vec<Ball> = Vec::new();
    Ball::populate(STARTX, STARTY, WIDTH, HEIGHT, &mut infected_arr, &mut normal_arr, INITIAL_INFECTED_POPULATION);

    let mut play = true;
    let mut reset: bool;
    let play_rect = Rectangle{x:(STARTX - 53) as f32, y:10.0, width:40.0, height:40.0};
    let reset_rect = Rectangle{x:(STARTX - 53) as f32, y:56.0, width:40.0, height:40.0};


    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(BG_COLOR);

        if play {
            play = d.gui_toggle(play_rect, Some(CString::new("Pause".as_bytes()).unwrap().as_c_str()), play);
        } else {
            play = d.gui_toggle(play_rect, Some(CString::new("Play".as_bytes()).unwrap().as_c_str()), play);
        }

        reset = d.gui_button(reset_rect, Some(CString::new("Reset".as_bytes()).unwrap().as_c_str()));
        if reset {
            infected_arr.clear();
            normal_arr.clear();
            dead_arr.clear();
            recovered_arr.clear(); 
            Ball::populate(STARTX, STARTY, WIDTH, HEIGHT, &mut infected_arr, &mut normal_arr, INITIAL_INFECTED_POPULATION)
        }

        let mut i: i32 = 0; 
        let mut end_idx: i32 = infected_arr.len() as i32;
        while i < end_idx {
            d.draw_circle_v(infected_arr[i as usize].pos, RADIUS + 1.0, INFECTED_COLOR);
            d.draw_circle_v(infected_arr[i as usize].pos, RADIUS - 1.0, Color::WHITE);

            if play {
                infected_arr[i as usize].update_position();
                infected_arr[i as usize].wall_collision(STARTX, STARTY, WIDTH, HEIGHT);
                (i, end_idx) = Ball::infection_check(&mut infected_arr, &mut normal_arr, &mut recovered_arr, &mut dead_arr, 
                    i as i32, end_idx as i32);
            }
            i += 1;
        }

        // drawing normal balls
        for ball in normal_arr.iter_mut() {
            d.draw_circle_v((*ball).pos, RADIUS, NORMAL_COLOR);
            if play {
                (*ball).update_position();
                (*ball).wall_collision(STARTX, STARTY, WIDTH, HEIGHT);
            }
        }

        // drawing recovered particles
        for ball in recovered_arr.iter_mut() {
            d.draw_circle_v((*ball).pos, RADIUS, RECOVERED_COLOR);
            if play {
                (*ball).update_position();
                (*ball).wall_collision(STARTX, STARTY, WIDTH, HEIGHT);
            }
        }

        // drawing dead particles
        draw_dead_box(&mut d, dead_arr.len());
        for ball in dead_arr.iter_mut() {
            d.draw_circle_v((*ball).pos, RADIUS, DEAD_COLOR);
        }

        draw_stats(&mut d, infected_arr.len(), normal_arr.len(), recovered_arr.len());
    }
}
