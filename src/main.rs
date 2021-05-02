use rand::Rng;
use std::collections::LinkedList;

use sfml::{
    graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable},
    system::{Clock, Vector2f},
    window::{ContextSettings, Event, Key, Style},
};

const SEGMENT_SIZE: f32 = 30.;
const GAME_WIDTH: i32 = 20;
const GAME_HEIGHT: i32 = 20;

fn put_food(head: &(i32, i32), segments: &LinkedList<(i32, i32)>) -> (i32, i32) {
    let food = loop {
        let food: (i32, i32) = (
            rand::thread_rng().gen_range(0, GAME_WIDTH),
            rand::thread_rng().gen_range(0, GAME_HEIGHT),
        );
        if *head == food || segments.contains(&food) {
            continue;
        }
        break food;
    };
    food
}

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    None,
}

struct SnakeGame {
    head: (i32, i32),
    segments: LinkedList<(i32, i32)>,
    food: (i32, i32),
    current_direction: Direction,
}

fn make_game() -> SnakeGame {
    let mut segments = LinkedList::new();
    segments.push_back((4, 5));
    segments.push_back((3, 5));
    let head = (5, 5);
    let food = put_food(&head, &segments);
    SnakeGame {
        head,
        segments,
        food,
        current_direction: Direction::Right,
    }
}

impl SnakeGame {
    fn step(&mut self, direction: Direction) -> i32 {
        let old_head = self.head;
        let mut old_tail = (-1, -1);
        match self.segments.back() {
            Some(seg) => {
                old_tail = *seg;
            }
            None => println!("has no value"),
        }
        self.segments.pop_back();

        if direction == Direction::Up && self.current_direction != Direction::Down {
            self.head.1 -= 1;
            self.current_direction = direction;
        } else if direction == Direction::Down && self.current_direction != Direction::Up {
            self.head.1 += 1;
            self.current_direction = direction;
        } else if direction == Direction::Left && self.current_direction != Direction::Right {
            self.head.0 -= 1;
            self.current_direction = direction;
        } else if direction == Direction::Right && self.current_direction != Direction::Left {
            self.head.0 += 1;
            self.current_direction = direction;
        } else {
            let second = self.segments.front();
            match second {
                Some(seg) => {
                    self.head.0 += self.head.0 - seg.0;
                    self.head.1 += self.head.1 - seg.1;
                }
                None => println!("Error: snake has less than 1 segment"),
            }
        }
        self.segments.push_front(old_head);
        if self.head == self.food {
            self.segments.push_back(old_tail);
            self.food = put_food(&self.head, &self.segments);
            return 1;
        }
        if self.segments.contains(&self.head) {
            return -1;
        }
        if self.head.0 < 0
            || self.head.0 >= GAME_WIDTH
            || self.head.1 < 0
            || self.head.1 >= GAME_HEIGHT
        {
            return -1;
        }
        0
    }
    fn render(&self, window: &mut RenderWindow) {
        for segment in self.segments.iter() {
            let mut seg = RectangleShape::new();
            seg.set_size(Vector2f::new(SEGMENT_SIZE - 3., SEGMENT_SIZE - 3.));
            seg.set_outline_thickness(3.);
            seg.set_outline_color(Color::BLACK);
            seg.set_fill_color(Color::rgb(200, 200, 200));
            seg.set_position(Vector2f::new(
                (segment.0 as f32) * SEGMENT_SIZE,
                (segment.1 as f32) * SEGMENT_SIZE,
            ));
            window.draw(&seg);
        }
        let mut seg = RectangleShape::new();
        seg.set_size(Vector2f::new(SEGMENT_SIZE - 3., SEGMENT_SIZE - 3.));
        seg.set_outline_thickness(3.);
        seg.set_outline_color(Color::BLACK);
        seg.set_fill_color(Color::rgb(240, 100, 100));
        seg.set_position(Vector2f::new(
            (self.head.0 as f32) * SEGMENT_SIZE,
            (self.head.1 as f32) * SEGMENT_SIZE,
        ));
        window.draw(&seg);

        let mut seg = RectangleShape::new();
        seg.set_size(Vector2f::new(SEGMENT_SIZE - 3., SEGMENT_SIZE - 3.));
        seg.set_outline_thickness(3.);
        seg.set_outline_color(Color::BLACK);
        seg.set_fill_color(Color::rgb(10, 200, 10));
        seg.set_position(Vector2f::new(
            (self.food.0 as f32) * SEGMENT_SIZE,
            (self.food.1 as f32) * SEGMENT_SIZE,
        ));
        window.draw(&seg);

        let mut wall = RectangleShape::new();
        wall.set_size(Vector2f::new(
            SEGMENT_SIZE,
            SEGMENT_SIZE * (GAME_HEIGHT + 1) as f32,
        ));
        wall.set_fill_color(Color::rgb(200, 200, 200));
        wall.set_position(Vector2f::new(SEGMENT_SIZE * GAME_WIDTH as f32, 0.));
        window.draw(&wall);

        let mut wall = RectangleShape::new();
        wall.set_size(Vector2f::new(
            SEGMENT_SIZE * (GAME_WIDTH + 1) as f32,
            SEGMENT_SIZE,
        ));
        wall.set_fill_color(Color::rgb(200, 200, 200));
        wall.set_position(Vector2f::new(0., SEGMENT_SIZE * GAME_HEIGHT as f32));
        window.draw(&wall);
    }
}

fn main() {
    let mut context_settings = ContextSettings::default();
    context_settings.antialiasing_level = 0;
    let mut window = RenderWindow::new((600, 600), "SFML snake", Style::CLOSE, &context_settings);
    let mut clock = Clock::start();
    // window.set_vertical_sync_enabled(true);
    clock.restart().as_seconds();
    let mut game = make_game();
    let mut score = 0;

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                _ => {}
            }
        }

        if clock.elapsed_time().as_seconds() > 0.3 {
            clock.restart();
            let reward;

            if Key::Up.is_pressed() {
                reward = game.step(Direction::Up);
            } else if Key::Down.is_pressed() {
                reward = game.step(Direction::Down);
            } else if Key::Left.is_pressed() {
                reward = game.step(Direction::Left);
            } else if Key::Right.is_pressed() {
                reward = game.step(Direction::Right);
            } else {
                reward = game.step(Direction::None);
            }
            if reward == -1 {
                // game over
                println!("Game over! Score: {}", score);
                return;
            }
            score += reward;
        }
        // Clear the window
        window.clear(Color::rgb(10, 10, 10));
        // Display things on screen
        game.render(&mut window);
        window.display()
    }
}
