use crate::domain::{self, World};
use core::f32;
use egui::{Color32, ImageSource, Pos2, Ui, Vec2};
use std::sync::{Arc, Mutex};

const WINDOW_TITLE: &str = "L3.10 Snake (@Egor Trukhin)";
const ASSET_SNAKE_HEAD: ImageSource = egui::include_image!("../../../assets/snake_head.svg");
const ASSET_SNAKE_BODY: ImageSource = egui::include_image!("../../../assets/snake_body.svg");
const ASSET_APPLE: ImageSource = egui::include_image!("../../../assets/apple.svg");
const IMAGE_SIZE: egui::Vec2 = egui::vec2(26.0, 26.0);

pub struct App {
    world: Arc<Mutex<World>>,
    image_snake_head: egui::Image<'static>,
    image_snake_body: egui::Image<'static>,
    image_apple: egui::Image<'static>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.change_direction(ctx);
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                self.change_color(ui);
                ui.separator();
                self.change_name(ui);
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_snake(ui);
            self.draw_name(ui);
            self.draw_apples(ui);
        });
        ctx.request_repaint();
    }
}

impl App {
    pub fn new(world: &Arc<Mutex<World>>) -> Self {
        Self {
            world: world.clone(),
            image_snake_head: egui::Image::new(ASSET_SNAKE_HEAD),
            image_snake_body: egui::Image::new(ASSET_SNAKE_BODY),
            image_apple: egui::Image::new(ASSET_APPLE),
        }
    }

    pub fn run(world: &Arc<Mutex<World>>) {
        eframe::run_native(
            WINDOW_TITLE,
            eframe::NativeOptions::default(),
            Box::new(|cc| {
                egui_extras::install_image_loaders(&cc.egui_ctx);
                let style = egui::Style {
                    visuals: egui::Visuals::dark(),
                    ..egui::Style::default()
                };
                cc.egui_ctx.set_style(style);
                Ok(Box::new(App::new(world)))
            }),
        )
        .unwrap();
    }

    pub fn draw_snake(&mut self, ui: &mut Ui) {
        let world = match self.world.try_lock() {
            Ok(world) => world,
            Err(_) => return,
        };

        let mut head = true;
        let head_origin = Vec2::splat(0.5);
        let head_rotate: f32 = match world.snake.direction {
            domain::Direction::UP => 0.0f32.to_radians(),
            domain::Direction::RIGHT => 90.0f32.to_radians(),
            domain::Direction::DOWN => 180.0f32.to_radians(),
            domain::Direction::LEFT => 270.0f32.to_radians(),
        };

        let color = Color32::from_rgb(
            world.snake.color[0],
            world.snake.color[1],
            world.snake.color[2],
        );

        for (x, y) in world.snake.positions.iter() {
            let (x, y) = (*x as f32, *y as f32);
            let center = egui::Pos2::new(IMAGE_SIZE.x * (x + 0.5), IMAGE_SIZE.y * (y + 0.5));
            let size = egui::vec2(IMAGE_SIZE.x * 1.3, IMAGE_SIZE.y * 1.3);
            let rect = egui::Rect::from_center_size(center, size);

            let snake = if head {
                head = false;
                self.image_snake_head
                    .clone()
                    .rotate(head_rotate, head_origin)
            } else {
                self.image_snake_body.clone()
            };

            ui.put(rect, snake.tint(color));
        }
    }

    pub fn draw_apples(&mut self, ui: &mut Ui) {
        let world = match self.world.try_lock() {
            Ok(world) => world,
            Err(_) => return,
        };

        for apple in &world.apples {
            let (x, y) = (apple.position.0, apple.position.1);
            let min = egui::Pos2::new(IMAGE_SIZE.x * (x + 0) as f32, IMAGE_SIZE.y * (y + 0) as f32);
            let max = egui::Pos2::new(IMAGE_SIZE.x * (x + 1) as f32, IMAGE_SIZE.y * (y + 1) as f32);
            let rect = egui::Rect::from_min_max(min, max);
            ui.put(rect, self.image_apple.clone());
        }
    }

    pub fn draw_name(&mut self, ui: &mut Ui) {
        let world = match self.world.try_lock() {
            Ok(world) => world,
            Err(_) => return,
        };

        let (x, y) = world.snake.positions.front().unwrap();
        let (x, y) = (*x as f32, *y as f32);

        let center = Pos2::new(IMAGE_SIZE.x * (x + 0.5), IMAGE_SIZE.y * (y - 1.0));
        let size = Vec2::new(10.0 * world.snake.username.len() as f32, 1.0);
        let rect = egui::Rect::from_center_size(center, size);
        let label = egui::Label::new(&world.snake.username);
        ui.put(rect, label);
    }

    pub fn change_name(&mut self, ui: &mut Ui) {
        let mut world = match self.world.try_lock() {
            Ok(world) => world,
            Err(_) => return,
        };

        ui.label("Username");
        ui.text_edit_singleline(&mut world.snake.username);
    }

    pub fn change_color(&mut self, ui: &mut Ui) {
        let mut world = match self.world.try_lock() {
            Ok(world) => world,
            Err(_) => return,
        };

        ui.label("Color");
        ui.color_edit_button_srgb(&mut world.snake.color);
    }

    pub fn change_direction(&mut self, ctx: &egui::Context) {
        // Обработка нажатий клавиатуры
        let mut new_direction = None;
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            new_direction = Some(domain::Direction::UP);
        } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            new_direction = Some(domain::Direction::DOWN);
        } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
            new_direction = Some(domain::Direction::LEFT);
        } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
            new_direction = Some(domain::Direction::RIGHT);
        }
        if new_direction.is_none() {
            return;
        }

        let new_direction = new_direction.unwrap();
        let mut world = match self.world.try_lock() {
            Ok(world) => world,
            Err(_) => return,
        };

        match (&world.snake.direction, &new_direction) {
            // Игнорируем разворот на 180 градусов
            (domain::Direction::UP, domain::Direction::DOWN) => return,
            (domain::Direction::DOWN, domain::Direction::UP) => return,
            (domain::Direction::LEFT, domain::Direction::RIGHT) => return,
            (domain::Direction::RIGHT, domain::Direction::LEFT) => return,

            // Меняем направление змеи
            _ => world.snake.direction = new_direction,
        }
    }
}
