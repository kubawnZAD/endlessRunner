use macroquad::prelude::*;

const GRAVITY: f32 = 900.0;
const JUMP_FORCE: f32 = -600.0;
const PLAYER_SIZE: f32 = 25.0;
const BACKGROUND_COLOR: Color = color_u8!(8, 144, 0, 255);
const PLAYER_COLOR: Color = color_u8!(255, 217, 102, 255);
const FPS : f32= 75.0;
const TIME_STEP: f32 = 1.0 / FPS; 

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

    // Ta metoda teraz przyjmuje stały krok czasowy, nie zmienny
    fn update(&mut self, dt: f32) {
        self.velocity_y += GRAVITY * dt;
        
        // Input sprawdzamy tutaj lub w głównej pętli, ale aplikujemy siłę w fizyce
        if is_key_pressed(KeyCode::Space) {
            self.velocity_y = JUMP_FORCE;
        }

        self.y += self.velocity_y * dt;

        // Kolizje
        if self.y + PLAYER_SIZE >= screen_height() {
            self.y = screen_height() - PLAYER_SIZE;
            self.velocity_y = 0.0;
        } else if self.y - PLAYER_SIZE < 0.0 {
            self.y = PLAYER_SIZE;
            self.velocity_y = 0.0;
        }
    }

    fn draw(&self) {
        draw_circle(self.x, self.y, PLAYER_SIZE, PLAYER_COLOR);
    }
}

#[macroquad::main("endless-runner")]
async fn main() {
    let mut player = Player::new();
    
    // Akumulator czasu
    let mut accumulator = 0.0;

    loop {
        // Dodajemy czas, który upłynął od ostatniej klatki
        accumulator += get_frame_time();

        // Pętla while: wykonuj aktualizacje fizyki tak długo, 
        // jak mamy wystarczająco dużo zgromadzonego czasu.
        while accumulator >= TIME_STEP {
            player.update(TIME_STEP); // Zawsze przekazujemy stałą wartość!
            accumulator -= TIME_STEP;
        }

        clear_background(BACKGROUND_COLOR);
        player.draw();

        // Opcjonalnie: Wyświetl FPS, aby monitorować wydajność
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 30.0, WHITE);

        next_frame().await
    }
}