use app::domain::{
    snake::{Snake, SnakeDirection},
    world::World,
};
use egui::{
    vec2, Align, CentralPanel, Color32, Context, Image, ImageSource, Key, Label, Layout, Pos2,
    Rect, Stroke, Style, TopBottomPanel, Ui, Vec2, Visuals,
};
use std::sync::{Arc, RwLock};

const WINDOW_TITLE: &str = "L3.10 Snake (@Egor Trukhin)";
const ASSET_SNAKE_HEAD: ImageSource = egui::include_image!("../../assets/snake_head.svg");
const ASSET_SNAKE_BODY: ImageSource = egui::include_image!("../../assets/snake_body.svg");
const ASSET_APPLE: ImageSource = egui::include_image!("../../assets/apple.svg");
const CELL_SIZE: Vec2 = egui::vec2(30.0, 30.0);

pub struct Client {
    snake: Arc<RwLock<Snake>>,
    world: Arc<RwLock<World>>,
    image_snake_head: Image<'static>,
    image_snake_body: Image<'static>,
    image_apple: Image<'static>,
}

pub fn run(snake: Arc<RwLock<Snake>>, world: Arc<RwLock<World>>) {
    let size = (&world).read().unwrap().size;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_transparent(true)
            .with_resizable(false)
            .with_maximized(false)
            .with_maximize_button(false)
            .with_inner_size(vec2(
                CELL_SIZE.x * size.0 as f32,
                CELL_SIZE.y * size.1 as f32,
            )),

        ..Default::default()
    };

    eframe::run_native(
        WINDOW_TITLE,
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            let style = Style {
                visuals: Visuals::dark(),
                ..Style::default()
            };
            cc.egui_ctx.set_style(style);
            Ok(Box::new(Client::new(snake, world)))
        }),
    )
    .unwrap();
}

impl eframe::App for Client {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        // Верхняя панель настроек
        TopBottomPanel::top("options").show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                self.draw_score(ui);
                ui.separator();
                self.change_color(ui);
                self.change_name(ui);
            });
        });

        // Отрисовка игрового мира
        CentralPanel::default().show(ctx, |ui| {
            self.draw_borders(ui);
            self.draw_snakes(ui);
            self.draw_apples(ui);
        });

        self.change_direction(ctx); // Обработка управления
        ctx.request_repaint(); // Постоянное обновление экрана
    }
}

impl Client {
    pub fn new(snake: Arc<RwLock<Snake>>, world: Arc<RwLock<World>>) -> Self {
        Self {
            snake,
            world,
            image_snake_head: Image::new(ASSET_SNAKE_HEAD),
            image_snake_body: Image::new(ASSET_SNAKE_BODY),
            image_apple: Image::new(ASSET_APPLE),
        }
    }

    pub fn draw_borders(&mut self, ui: &mut Ui) {
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            ui.painter().rect_stroke(
                ui.max_rect(),
                egui::Rounding::default(),
                Stroke::new(4.0, Color32::DEBUG_COLOR),
            );
        });
    }

    pub fn draw_snakes(&mut self, ui: &mut Ui) {
        let world = match self.world.try_read() {
            Ok(world) => world,
            Err(_) => return,
        };

        for snake in &world.snakes {
            let color = Color32::from_rgb(snake.color[0], snake.color[1], snake.color[2]);
            let mut positions = snake.positions.iter().copied();
            let head = positions.next().unwrap();

            // Тело змеи
            for (x, y) in positions {
                let center = Pos2::new(
                    CELL_SIZE.x * (x as f32 + 0.5),
                    CELL_SIZE.y * (y as f32 + 0.5),
                );
                let rect = Rect::from_center_size(center, CELL_SIZE * 1.2);
                let image = self.image_snake_body.clone().tint(color);
                ui.put(rect, image);
            }

            // Голова змеи (с направлением)
            let head_origin = Vec2::splat(0.5);
            let head_angle = match snake.direction {
                SnakeDirection::UP => 0.0f32.to_radians(),
                SnakeDirection::RIGHT => 90.0f32.to_radians(),
                SnakeDirection::DOWN => 180.0f32.to_radians(),
                SnakeDirection::LEFT => 270.0f32.to_radians(),
            };
            let center = Pos2::new(
                CELL_SIZE.x * (head.0 as f32 + 0.5),
                CELL_SIZE.y * (head.1 as f32 + 0.5),
            );
            let rect = Rect::from_center_size(center, CELL_SIZE * 1.2);
            let image = self
                .image_snake_head
                .clone()
                .rotate(head_angle, head_origin)
                .tint(color);
            ui.put(rect, image);

            // Имя змеи (над головой)
            let center = Pos2::new(center.x, CELL_SIZE.y * (head.1 as f32 - 1.0));
            let size = Vec2::new(10.0 * snake.username.len() as f32, 1.0);
            let rect = Rect::from_center_size(center, size);
            let label = Label::new(&snake.username);
            ui.put(rect, label);
        }
    }

    pub fn draw_apples(&mut self, ui: &mut Ui) {
        if let Ok(world) = self.world.try_read() {
            for apple in &world.apples {
                let center = Pos2::new(
                    CELL_SIZE.x * (apple.position.0 as f32 + 0.5),
                    CELL_SIZE.y * (apple.position.1 as f32 + 0.5),
                );
                let rect = Rect::from_center_size(center, CELL_SIZE * 0.7);
                ui.put(rect, self.image_apple.clone());
            }
        }
    }

    pub fn draw_score(&self, ui: &mut Ui) {
        if let Ok(snake) = self.snake.try_read() {
            ui.label(format!("Score: {}", snake.score));
        }
    }

    pub fn change_name(&mut self, ui: &mut Ui) {
        if let Ok(mut snake) = self.snake.try_write() {
            ui.label("Username");
            ui.text_edit_singleline(&mut snake.username);
        }
    }

    pub fn change_color(&mut self, ui: &mut Ui) {
        if let Ok(mut snake) = self.snake.try_write() {
            ui.label("Color");
            ui.color_edit_button_srgb(&mut snake.color);
        }
    }

    pub fn change_direction(&mut self, ctx: &Context) {
        // Обработка нажатий клавиатуры
        let mut new_direction = None;
        if ctx.input(|i| i.key_pressed(Key::ArrowUp)) {
            new_direction = Some(SnakeDirection::UP);
        } else if ctx.input(|i| i.key_pressed(Key::ArrowDown)) {
            new_direction = Some(SnakeDirection::DOWN);
        } else if ctx.input(|i| i.key_pressed(Key::ArrowLeft)) {
            new_direction = Some(SnakeDirection::LEFT);
        } else if ctx.input(|i| i.key_pressed(Key::ArrowRight)) {
            new_direction = Some(SnakeDirection::RIGHT);
        }

        // Обновление направления
        if let Some(new_direction) = new_direction {
            if let Ok(mut snake) = self.snake.try_write() {
                match (&snake.direction, &new_direction) {
                    // Игнорируем разворот на 180 градусов
                    (SnakeDirection::UP, SnakeDirection::DOWN) => return,
                    (SnakeDirection::DOWN, SnakeDirection::UP) => return,
                    (SnakeDirection::LEFT, SnakeDirection::RIGHT) => return,
                    (SnakeDirection::RIGHT, SnakeDirection::LEFT) => return,
                    _ => snake.direction = new_direction,
                }
            }
        }
    }
}
