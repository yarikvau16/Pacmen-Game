use macroquad::prelude::*;
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};

// --- Збереження рекорду ---
#[derive(Serialize, Deserialize)]
struct GameRecord {
    score: i32,
    time: f32, // час у секундах
}

fn save_best_score(record: &GameRecord) {
    let json = serde_json::to_string(record).unwrap();
    fs::write("best_score.json", json).unwrap_or_else(|e| {
        println!("Помилка запису: {}", e);
    });
}

fn load_best_score() -> Option<GameRecord> {
    if !Path::new("best_score.json").exists() {
        return None;
    }
    let content = fs::read_to_string("best_score.json").ok()?;
    serde_json::from_str(&content).ok()
}

// --- Конфігурація вікна ---
fn window_conf() -> Conf {
    Conf {
        window_title: "Pacman - Apple Collector 🍏".to_owned(),
        window_width: 900,
        window_height: 650,
        window_resizable: false,
        ..Default::default()
    }

}


// --- Головна функція ---
#[macroquad::main(window_conf)]
async fn main() {
    let mut pacman_pos = vec2(400.0, 300.0);
    let mut pacman_mouth_open = false;
    let mut mouth_timer = 0.0;

    let mut apples: Vec<Vec2> = Vec::new();
    let mut score = 0;
    let mut spawn_timer = 0.0;
    let mut elapsed_time = 0.0; // час гри

    // Завантажуємо рекорд
    let mut best_score_record = load_best_score().unwrap_or(GameRecord { score: 0, time: 0.0 });

    // Початково 2 яблука
    for _ in 0..2 {
        apples.push(random_vec());
    }

    loop {
        clear_background(BLACK);
        let dt = get_frame_time();
        elapsed_time += dt;

        // --- Керування пакменом ---
        let speed = 4.0;
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) { pacman_pos.x += speed; }
        if is_key_down(KeyCode::Left)  || is_key_down(KeyCode::A) { pacman_pos.x -= speed; }
        if is_key_down(KeyCode::Up)    || is_key_down(KeyCode::W) { pacman_pos.y -= speed; }
        if is_key_down(KeyCode::Down)  || is_key_down(KeyCode::S) { pacman_pos.y += speed; }

        // --- Обмеження руху ---
        let radius = 20.0;
        let sw = screen_width();
        let sh = screen_height();
        pacman_pos.x = pacman_pos.x.clamp(radius, sw - radius);
        pacman_pos.y = pacman_pos.y.clamp(radius, sh - radius);

        // --- Малюємо яблука ---
        for apple in &apples {
            draw_circle(apple.x, apple.y, 10.0, RED);
            draw_circle(apple.x, apple.y - 10.0, 4.0, GREEN);
        }

        // --- Перевірка зіткнень ---
        apples.retain(|apple| {
            let dist = pacman_pos.distance(*apple);
            if dist < radius + 10.0 {
                score += 1;
                pacman_mouth_open = true;
                mouth_timer = 0.3;
                false
            } else { true }
        });

        // --- Спавн нових яблук ---
        spawn_timer += dt;
        if apples.len() < 2 && spawn_timer > 0.8 {
            apples.push(random_vec());
            spawn_timer = 0.0;
        }

        // --- Малюємо пакмена ---
        if pacman_mouth_open {
            draw_pacman(pacman_pos.x, pacman_pos.y, radius, 45.0);
            mouth_timer -= dt;
            if mouth_timer <= 0.0 { pacman_mouth_open = false; }
        } else {
            draw_circle(pacman_pos.x, pacman_pos.y, radius, YELLOW);
        }

        // --- Текст рахунку ---
        draw_text(&format!("Score: {}", score), 20.0, 40.0, 30.0, YELLOW);
        draw_text(&format!("Best: {} ({:.1}s)", best_score_record.score, best_score_record.time), 20.0, 80.0, 30.0, GREEN);

        // --- Оновлення рекорду ---
        if score > best_score_record.score {
            best_score_record = GameRecord { score, time: elapsed_time };
            save_best_score(&best_score_record);
        }

        next_frame().await;
    }


}

// --- Випадкове місце для яблука ---
fn random_vec() -> Vec2 {
    vec2(
        rand::gen_range(30.0, screen_width() - 30.0),
        rand::gen_range(30.0, screen_height() - 30.0),
    )
}

// --- Малювання пакмена з відкритим ротом ---
fn draw_pacman(x: f32, y: f32, r: f32, mouth_angle: f32) {
    let start_angle = mouth_angle.to_radians();
    let end_angle = (360.0 - mouth_angle).to_radians();

    draw_circle(x, y, r, YELLOW);

    let left_len = r * 1.7;
    let right_len = r * 1.3;

    draw_triangle(
        vec2(x, y),
        vec2(x + left_len * start_angle.cos(), y + left_len * start_angle.sin()),
        vec2(x + right_len * end_angle.cos(), y + right_len * end_angle.sin()),
        BLACK,
    );
}
