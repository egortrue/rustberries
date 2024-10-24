use crate::domain::{self, Snake, World};
use core::f32;
use egui::{Color32, Pos2, Ui, Vec2};
use std::sync::{Arc, Mutex};

const IMAGE_SNAKE_HEAD: egui::ImageSource<'static> =
    egui::include_image!("../../../assets/snake_head.svg");
const IMAGE_SNAKE_BODY: egui::ImageSource<'static> =
    egui::include_image!("../../../assets/snake_body.svg");
const IMAGE_APPLE: egui::ImageSource<'static> = egui::include_image!("../../../assets/apple.svg");
const IMAGE_SIZE: egui::Vec2 = egui::vec2(30.0, 30.0);

pub struct App {
    world: Arc<Mutex<World>>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_snake(ui);
            self.draw_name(ui);
            self.draw_apple(ui);
            self.change_direction(ctx);
        });
        ctx.request_repaint();
    }
}

impl App {
    pub fn new(world: &Arc<Mutex<World>>) -> Self {
        Self {
            world: world.clone(),
        }
    }

    pub fn run(world: &Arc<Mutex<World>>) {
        eframe::run_native(
            "L3.10 Snake (@Egor Trukhin)",
            eframe::NativeOptions::default(),
            Box::new(|cc| {
                egui_extras::install_image_loaders(&cc.egui_ctx);
                let style = egui::Style {
                    visuals: egui::Visuals::light(),
                    ..egui::Style::default()
                };
                cc.egui_ctx.set_style(style);
                Ok(Box::new(App::new(world)))
            }),
        )
        .unwrap();
    }

    pub fn change_direction(&mut self, ctx: &egui::Context) {
        let mut world = match self.world.try_lock() {
            Ok(world) => world,
            Err(_) => return,
        };
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            world.snake.direction = domain::Direction::UP;
        } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            world.snake.direction = domain::Direction::DOWN;
        } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
            world.snake.direction = domain::Direction::LEFT;
        } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
            world.snake.direction = domain::Direction::RIGHT;
        }
    }

    pub fn draw_snake(&mut self, ui: &mut Ui) {
        let world = match self.world.try_lock() {
            Ok(world) => world,
            Err(_) => return,
        };

        let snake_head = egui::Image::new(IMAGE_SNAKE_HEAD);
        let snake_body = egui::Image::new(IMAGE_SNAKE_BODY);

        let mut head = true;
        let head_rotate: f32 = match world.snake.direction {
            domain::Direction::UP => 0.0,
            domain::Direction::DOWN => 180.0,
            domain::Direction::LEFT => 270.0,
            domain::Direction::RIGHT => 90.0,
        };
        let snake_head = snake_head.rotate(head_rotate.to_radians(), Vec2::splat(0.5));

        for (x, y) in world.snake.positions.iter() {
            let (x, y) = (*x as f32, *y as f32);
            let center = egui::Pos2::new(IMAGE_SIZE.x * (x + 0.5), IMAGE_SIZE.y * (y + 0.5));
            let size = egui::vec2(IMAGE_SIZE.x * 1.2, IMAGE_SIZE.y * 1.2);
            let rect = egui::Rect::from_center_size(center, size);

            let part = if head {
                head = false;
                snake_head.clone()
            } else {
                snake_body.clone()
            };

            ui.put(rect, part.tint(Color32::YELLOW));
        }
    }

    pub fn draw_apple(&mut self, ui: &mut Ui) {
        let apple = egui::Image::new(IMAGE_APPLE);
        let (x, y) = (2, 3);
        let min = egui::Pos2::new(IMAGE_SIZE.x * (x + 0) as f32, IMAGE_SIZE.y * (y + 0) as f32);
        let max = egui::Pos2::new(IMAGE_SIZE.x * (x + 1) as f32, IMAGE_SIZE.y * (y + 1) as f32);
        let rect = egui::Rect::from_min_max(min, max);
        ui.put(rect, apple);
    }

    pub fn draw_name(&mut self, ui: &mut Ui) {
        let world = match self.world.try_lock() {
            Ok(world) => world,
            Err(_) => return,
        };

        let name = "user1";
        let (x, y) = world.snake.positions.front().unwrap();
        let (x, y) = (*x as f32, *y as f32);

        let center = Pos2::new(IMAGE_SIZE.x * (x + 0.5), IMAGE_SIZE.y * (y - 0.5));
        let size = Vec2::new(10.0 * name.len() as f32, 1.0);
        let rect = egui::Rect::from_center_size(center, size);
        let label = egui::Label::new(name);
        ui.put(rect, label);
    }
}
