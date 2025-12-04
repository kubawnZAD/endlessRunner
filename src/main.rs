use macroquad::prelude::*;


const GRAVITY: f32 = 900.0;
const JUMP_FORCE: f32 = -600.0;
const PLAYER_SIZE: f32 = 25.0;
const BACKGROUND_COLOR: Color = color_u8!(8,144,0,255);
const PLAYER_COLOR: Color = color_u8!(255,217,102,255);
struct Player{
    x: f32,
    y: f32,
    velocity_y: f32,
}

impl Player {
    fn new() -> Player{
        Player { x: 100.0, y: 100.0, velocity_y: 0.0, }
    }

    fn update(&mut self, dt:f32){
        
        self.velocity_y+=GRAVITY*dt;
        if is_key_pressed(KeyCode::Space){
            self.velocity_y=JUMP_FORCE;
        }
        self.y+=self.velocity_y*dt;

        if self.y+PLAYER_SIZE>=screen_height(){
            self.y=screen_height()-PLAYER_SIZE;
            self.velocity_y=0.0;
        }
        else if self.y-PLAYER_SIZE<0f32{
            self.y=PLAYER_SIZE;
            self.velocity_y=0.0;
        }
    }

    fn draw(&self){
        draw_circle(self.x, self.y, PLAYER_SIZE,PLAYER_COLOR);
    }
}
#[macroquad::main("endless-runner")]
async fn main() {
    let mut player = Player::new();
    println!("{}",get_fps());
    loop {
        let dt = get_frame_time();
        
        clear_background(BACKGROUND_COLOR);
        player.update(dt);
        player.draw();


        next_frame().await
    }
}
