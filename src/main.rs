use std::fs::{self};

use macroquad::{prelude::*};

const GRAVITY: f32 = 1300.0;
const JUMP_FORCE: f32 = -460.0;
const PLAYER_SIZE: f32 = 25.0;
const BACKGROUND_COLOR: Color = color_u8!(8, 144, 0, 255);
const BG_SPEED: f32 = 300.0;

fn window_conf() -> Conf {
    Conf {
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        window_title: String::from("game"),
        platform:miniquad::conf::Platform{
            swap_interval: Some(0),
            ..Default::default()
        },
        ..Default::default()
    }
}

enum GameState{
    Playing,
    GameOver,
}

struct Player {
    x: f32,
    y: f32,
    velocity_y: f32,
    jump: bool,
}

impl Player {
    fn new() -> Player {
        Player {
            x: 100.0,
            y: 100.0,
            velocity_y: 0.0,
            jump: false,
        }
    }

    fn update(&mut self, dt: f32) {
        self.velocity_y += GRAVITY * dt;
        
        if self.jump || is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
            self.velocity_y = JUMP_FORCE;
            self.jump = false;
        }

        self.y += self.velocity_y * dt;

        if self.y + PLAYER_SIZE >= screen_height() {
            self.y = screen_height() - PLAYER_SIZE;
            self.velocity_y = 0.0;
        } else if self.y - PLAYER_SIZE < 0.0 {
            self.y = PLAYER_SIZE;
            self.velocity_y = 0.0;
        }

    }

    fn draw(&self, texture: &Texture2D) {
        let params = DrawTextureParams {
            dest_size: Some(vec2(PLAYER_SIZE * 2.0, PLAYER_SIZE * 2.0)),
            rotation: self.velocity_y * 0.001, 
            ..Default::default()
        };
        draw_texture_ex(texture, self.x-PLAYER_SIZE, self.y-PLAYER_SIZE, WHITE, params);
    }
    fn get_rect(&self) -> Rect {
        Rect::new(self.x-PLAYER_SIZE, self.y-PLAYER_SIZE, PLAYER_SIZE*1.90, PLAYER_SIZE*1.90)
}
}

struct Obstacle{
    x:f32,
    y:f32,
    size_x:f32,
    height:f32,
    passed:bool,
}


impl Obstacle{

    fn new(x:f32,y:f32,height:f32) ->Obstacle{
        Obstacle { x: (x), y: (y), size_x: (80f32), height: (height), passed: (false)}
    }
    fn draw(&self, texture: &Texture2D, flip:bool){
        let params = DrawTextureParams {
            dest_size: Some(vec2(self.size_x, self.height)),
            flip_y: flip,
            ..Default::default()
        };
        draw_texture_ex(texture, self.x, self.y, WHITE, params);
    }
    fn update(&mut self,dt:f32){
        self.x-=300f32*dt;
    }
    fn get_rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.size_x, self.height)
    }
    

}

fn get_highscore() -> u32 {
    match fs::read_to_string("highscore.txt") {
        Ok(content) => {
            content.trim().parse().unwrap_or(0)
        }
        Err(_) => 0,
    }
}
fn save_highscore(score: u32) {
    if let Err(e) = fs::write("highscore.txt", score.to_string()) {
        println!("Błąd zapisu highscore: {}", e);
    }
    else{
        println!("Zapisano highscore: {}", score);
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");
    let player_texture = load_texture("player.png").await.unwrap();
    player_texture.set_filter(FilterMode::Nearest);
    let pipe_texture = load_texture("pipe.png").await.unwrap();
    pipe_texture.set_filter(FilterMode::Nearest);
    let bg_texture = load_texture("background.png").await.unwrap();
    bg_texture.set_filter(FilterMode::Nearest);
    let mut player = Player::new();
    let mut obstacles: Vec<(Obstacle,Obstacle)> = Vec::new();
    let mut obs_timer=0.0;
    let mut time=1.5;
    let mut game_state = GameState::Playing;
    let mut score=0;
    let mut bg_scroll: f32 = 0.0;
    let mut high_score: u32 = get_highscore();
    const TIME_STEP: f32 = 1.0 / 75.0;
    let mut accumulator: f32 = 0.0;

    loop {
        let dt = get_frame_time(); 
        accumulator += dt;

        if accumulator > 0.1 {
            accumulator = 0.1;
        }
        
            match game_state {
                GameState::Playing =>{
                    if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left){
                        player.jump=true;
                    }
                }
                GameState::GameOver =>{
                    if is_key_pressed(KeyCode::R){
                        player=Player::new();
                        obstacles.clear();
                        game_state = GameState::Playing;
                        obs_timer=0.0;
                        score=0;
                        bg_scroll = 0.0;
                        accumulator = 0.0;
                    }
                }
                
            }

        while accumulator >= TIME_STEP {
            match game_state{
                GameState::Playing =>{
                    bg_scroll -= BG_SPEED * TIME_STEP;
                    if bg_scroll <= -screen_width() {
                        bg_scroll = 0.0;
                    }
                    //Tworzenie przeszkód
                    obs_timer+=TIME_STEP;
                    if obs_timer>=time{
                        let height = rand::gen_range(70f32,350f32);
                        let obs1=Obstacle::new(screen_width(),screen_height()-height-120f32,height);
                        let obs2=Obstacle::new(screen_width(),0.0,screen_height()-height-270.0);
                        obstacles.push((obs1,obs2));
                        obs_timer=0.0;
                        time=rand::gen_range(1.0, 2.0);
                    }
                    //Usuwanie przeszkód poza ekranem
                    obstacles.retain(|x| x.0.x>-100f32);
                    player.update(TIME_STEP);
                    for (obs1, obs2) in obstacles.iter_mut(){
                        obs1.update(TIME_STEP);
                        obs2.update(TIME_STEP);
                    }
                    //Wykrywanie kolizji
                    for (obs1,obs2) in obstacles.iter_mut(){
                        if player.get_rect().overlaps(&obs1.get_rect()){
                            game_state=GameState::GameOver;
                        }
                        if player.get_rect().overlaps(&obs2.get_rect()){
                            game_state=GameState::GameOver;
                        }
                        if player.x-30f32>obs1.x+obs1.size_x&&!obs1.passed{
                            score+=1;
                            obs1.passed=true;
                        }
                        
                    
                    }
                    if player.y+PLAYER_SIZE+120f32 >= screen_height() || player.y < 0.0{
                        
                        game_state = GameState::GameOver;
                    }
                }
            
            GameState::GameOver => {
                if score > high_score {
                    high_score = score;
                    save_highscore(high_score);
                }
            }
        }
            accumulator-=TIME_STEP;
    }

        clear_background(BACKGROUND_COLOR);
        let bg_params = DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height())),
            ..Default::default()
        };

        draw_texture_ex(
            &bg_texture,
            bg_scroll,
            0.0,
            WHITE,
            bg_params.clone(),
        );

        draw_texture_ex(
            &bg_texture,
            bg_scroll + screen_width(),
            0.0,
            WHITE,
            bg_params,
        );
        for (obs1,obs2) in obstacles.iter() {
            obs1.draw(&pipe_texture,false);
            obs2.draw(&pipe_texture,true);
        }
        player.draw(&player_texture);

        if let GameState::GameOver = game_state {
            draw_text("GAME OVER!", screen_width()/2.0 - 100.0, screen_height()/2.0, 60.0, RED);
            draw_text("Press 'R' to restart", screen_width()/2.0 - 120.0, screen_height()/2.0 + 50.0, 30.0, DARKGRAY);
        }
        draw_text(&format!("Score: {}", score), screen_width()-120.0, 20.0, 30.0, WHITE);
        draw_text(&format!("Best: {}", high_score), screen_width() - 120.0, 50.0, 30.0, GOLD);
        next_frame().await
    
    }
}