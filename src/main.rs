use macroquad::{prelude::*};

const GRAVITY: f32 = 1300.0;
const JUMP_FORCE: f32 = -480.0;
const PLAYER_SIZE: f32 = 25.0;
const BACKGROUND_COLOR: Color = color_u8!(8, 144, 0, 255);


fn window_conf() -> Conf {
    Conf {
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        window_title: String::from("game"),
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
}

impl Player {
    fn new() -> Player {
        Player {
            x: 100.0,
            y: 100.0,
            velocity_y: 0.0,
        }
    }

    fn update(&mut self, dt: f32) {
        self.velocity_y += GRAVITY * dt;
        
        if is_key_pressed(KeyCode::Space) {
            self.velocity_y = JUMP_FORCE;
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

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");
    let player_texture = load_texture("player.png").await.unwrap();
    player_texture.set_filter(FilterMode::Nearest);
    let pipe_texture = load_texture("pipe.png").await.unwrap();
    pipe_texture.set_filter(FilterMode::Nearest);
    let mut player = Player::new();
    let mut obstacles: Vec<(Obstacle,Obstacle)> = Vec::new();
    let mut obs_timer=0.0;
    let mut time=1.5;
    let mut game_state = GameState::Playing;
    let mut score=0;
    loop {
        let dt = get_frame_time();

        match game_state{
            GameState::Playing =>{
                //Tworzenie przeszkód
                obs_timer+=dt;
                if obs_timer>=time{
                    let height = rand::gen_range(50f32,450f32);
                    let obs1=Obstacle::new(screen_width(),screen_height()-height,height);
                    let obs2=Obstacle::new(screen_width(),0.0,screen_height()-height-150.0);
                    obstacles.push((obs1,obs2));
                    obs_timer=0.0;
                    time=rand::gen_range(1.0, 2.0);
                }
                //Usuwanie przeszkód poza ekranem
                obstacles.retain(|x| x.0.x>-100f32);
                player.update(dt);
                for (obs1, obs2) in obstacles.iter_mut(){
                    obs1.update(dt);
                    obs2.update(dt);
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
                if player.y > screen_height() || player.y < 0.0 {
                     game_state = GameState::GameOver;
                }
            }
        
        GameState::GameOver => {
            if is_key_pressed(KeyCode::R){
                player=Player::new();
                obstacles.clear();
                game_state = GameState::Playing;
                obs_timer=0.0;
                score=0;
            }
        }
    }

        clear_background(BACKGROUND_COLOR);
        player.draw(&player_texture);
        for (obs1,obs2) in obstacles.iter() {
            obs1.draw(&pipe_texture,false);
            obs2.draw(&pipe_texture,true);
        }
        

        if let GameState::GameOver = game_state {
            draw_text("GAME OVER!", screen_width()/2.0 - 100.0, screen_height()/2.0, 60.0, RED);
            draw_text("Press 'R' to restart", screen_width()/2.0 - 120.0, screen_height()/2.0 + 50.0, 30.0, DARKGRAY);
        }
        draw_text(&format!("Score: {}", score), screen_width()-120.0, 20.0, 30.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 30.0, WHITE);
        next_frame().await
    }
}