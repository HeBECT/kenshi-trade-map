use eframe::egui;
use egui::{Color32, Pos2, Rect, Stroke, Vec2, Rounding};
use egui::epaint::Shadow;
use image::DynamicImage;
use serde::{Deserialize, Serialize};

// Встроенная ватермарка - изображение встроено в исполняемый файл
// Это изображение нельзя удалить или изменить без перекомпиляции
const WATERMARK_BYTES: &[u8] = include_bytes!("png.png");

// Современная цветовая палитра
#[allow(dead_code)]
struct ModernColors {
    // Основные цвета
    primary: Color32,
    primary_hover: Color32,
    primary_dark: Color32,
    secondary: Color32,
    secondary_hover: Color32,
    accent: Color32,
    accent_hover: Color32,
    
    // Фоны
    bg_dark: Color32,
    bg_medium: Color32,
    bg_light: Color32,
    bg_card: Color32,
    
    // Текст
    text_primary: Color32,
    text_secondary: Color32,
    text_muted: Color32,
    
    // Состояния
    success: Color32,
    success_hover: Color32,
    warning: Color32,
    danger: Color32,
    danger_hover: Color32,
    info: Color32,
    info_hover: Color32,
    
    // Градиенты и эффекты
    gradient_start: Color32,
    gradient_end: Color32,
    glow_primary: Color32,
    glow_secondary: Color32,
}

impl ModernColors {
    fn dark() -> Self {
        Self {
            // Киберпанк цвета в стиле сайта
            primary: Color32::from_rgb(88, 101, 242),      // Discord blue
            primary_hover: Color32::from_rgb(114, 137, 218),
            primary_dark: Color32::from_rgb(71, 82, 196),
            secondary: Color32::from_rgb(114, 137, 218),   // Light blue
            secondary_hover: Color32::from_rgb(142, 161, 227),
            accent: Color32::from_rgb(255, 115, 250),      // Neon pink
            accent_hover: Color32::from_rgb(255, 145, 252),
            
            // Темный фон как на сайте
            bg_dark: Color32::from_rgb(17, 17, 27),        // Very dark blue-black
            bg_medium: Color32::from_rgb(23, 25, 35),      // Dark blue-gray
            bg_light: Color32::from_rgb(32, 34, 46),       // Medium dark
            bg_card: Color32::from_rgb(28, 30, 42),        // Card background
            
            // Текст
            text_primary: Color32::from_rgb(255, 255, 255),
            text_secondary: Color32::from_rgb(185, 187, 190),
            text_muted: Color32::from_rgb(114, 118, 125),
            
            // Яркие акценты
            success: Color32::from_rgb(67, 181, 129),      // Green
            success_hover: Color32::from_rgb(87, 201, 149),
            warning: Color32::from_rgb(250, 166, 26),      // Orange
            danger: Color32::from_rgb(237, 66, 69),        // Red
            danger_hover: Color32::from_rgb(242, 96, 99),
            info: Color32::from_rgb(0, 176, 244),          // Cyan
            info_hover: Color32::from_rgb(30, 196, 255),
            
            // Неоновые градиенты
            gradient_start: Color32::from_rgb(88, 101, 242),  // Blue
            gradient_end: Color32::from_rgb(255, 115, 250),   // Pink
            glow_primary: Color32::from_rgba_unmultiplied(88, 101, 242, 80),
            glow_secondary: Color32::from_rgba_unmultiplied(255, 115, 250, 60),
        }
    }
    
    fn light() -> Self {
        Self {
            primary: Color32::from_rgb(88, 101, 242),
            primary_hover: Color32::from_rgb(114, 137, 218),
            primary_dark: Color32::from_rgb(71, 82, 196),
            secondary: Color32::from_rgb(114, 137, 218),
            secondary_hover: Color32::from_rgb(142, 161, 227),
            accent: Color32::from_rgb(255, 115, 250),
            accent_hover: Color32::from_rgb(255, 145, 252),
            
            bg_dark: Color32::from_rgb(248, 249, 250),
            bg_medium: Color32::from_rgb(241, 243, 245),
            bg_light: Color32::from_rgb(255, 255, 255),
            bg_card: Color32::from_rgb(255, 255, 255),
            
            text_primary: Color32::from_rgb(32, 34, 37),
            text_secondary: Color32::from_rgb(79, 84, 92),
            text_muted: Color32::from_rgb(114, 118, 125),
            
            success: Color32::from_rgb(67, 181, 129),
            success_hover: Color32::from_rgb(87, 201, 149),
            warning: Color32::from_rgb(250, 166, 26),
            danger: Color32::from_rgb(237, 66, 69),
            danger_hover: Color32::from_rgb(242, 96, 99),
            info: Color32::from_rgb(0, 176, 244),
            info_hover: Color32::from_rgb(30, 196, 255),
            
            gradient_start: Color32::from_rgb(88, 101, 242),
            gradient_end: Color32::from_rgb(255, 115, 250),
            glow_primary: Color32::from_rgba_unmultiplied(88, 101, 242, 40),
            glow_secondary: Color32::from_rgba_unmultiplied(255, 115, 250, 30),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
enum Language {
    Russian,
    English,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
enum Theme {
    Dark,
    Light,
}

struct Texts {
    // Top panel
    #[allow(dead_code)]
    title: &'static str,
    load_map: &'static str,
    add_city: &'static str,
    delete_city: &'static str,
    save: &'static str,
    load: &'static str,
    cities: &'static str,
    routes: &'static str,
    zoom: &'static str,
    language: &'static str,
    theme: &'static str,
    
    // Route dialog
    new_route: &'static str,
    route_name: &'static str,
    route_color: &'static str,
    create: &'static str,
    cancel: &'static str,
    
    // City dialog
    new_city: &'static str,
    city_name: &'static str,
    confirm: &'static str,
    
    // Delete city dialog
    delete_city_title: &'static str,
    delete_city_message: &'static str,
    
    // Trade panel
    trade_route: &'static str,
    item: &'static str,
    buy_markup: &'static str,
    sell_markup: &'static str,
    hold: &'static str,
    sell: &'static str,
    add_item: &'static str,
    delete_route: &'static str,
    close: &'static str,
    
    // Hints
    hint_route: &'static str,
    hint_city: &'static str,
    hint_delete_city: &'static str,
    
    // Map
    load_map_prompt: &'static str,
}

impl Texts {
    fn get(lang: Language) -> Self {
        match lang {
            Language::Russian => Self {
                title: "Kenshi Trade Map",
                load_map: "Загрузить карту",
                add_city: "Добавить город",
                delete_city: "Удалить город",
                save: "Сохранить пути",
                load: "Загрузить пути",
                cities: "Городов",
                routes: "Маршрутов",
                zoom: "Зум",
                language: "Language",
                theme: "Тема",
                
                new_route: "Новый маршрут",
                route_name: "Название маршрута:",
                route_color: "Цвет маршрута:",
                create: "Создать",
                cancel: "Отмена",
                
                new_city: "Новый город",
                city_name: "Название города:",
                confirm: "Подтвердить",
                
                delete_city_title: "Удаление города",
                delete_city_message: "Вы точно хотите удалить этот город и все пути, прикреплённые к нему?",
                
                trade_route: "Торговый маршрут",
                item: "Товар:",
                buy_markup: "Наценка Покупки %:",
                sell_markup: "Наценка Продажи %:",
                hold: "Придержать",
                sell: "Продавать",
                add_item: "+ Добавить товар",
                delete_route: "Удалить маршрут",
                close: "Закрыть",
                
                hint_route: "ЛКМ по городу — выбрать конец маршрута | Esc — отмена",
                hint_city: "ЛКМ на карте — разместить город | Esc — отмена",
                hint_delete_city: "ЛКМ по городу — удалить город | Esc — отмена",
                
                load_map_prompt: "Загрузите карту Kenshi",
            },
            Language::English => Self {
                title: "Kenshi Trade Map",
                load_map: "Load Map",
                add_city: "Add City",
                delete_city: "Delete City",
                save: "Save Routes",
                load: "Edit Routes",
                cities: "Cities",
                routes: "Routes",
                zoom: "Zoom",
                language: "Язык",
                theme: "Theme",
                
                new_route: "New Route",
                route_name: "Route name:",
                route_color: "Route color:",
                create: "Create",
                cancel: "Cancel",
                
                new_city: "New City",
                city_name: "City name:",
                confirm: "Confirm",
                
                delete_city_title: "Delete City",
                delete_city_message: "Are you sure you want to delete this city and all routes attached to it?",
                
                trade_route: "Trade Route",
                item: "Item:",
                buy_markup: "Buy Markup %:",
                sell_markup: "Sell Markup %:",
                hold: "Hold",
                sell: "Sell",
                add_item: "+ Add Item",
                delete_route: "Delete Route",
                close: "Close",
                
                hint_route: "LMB on city — select end | Esc — cancel",
                hint_city: "LMB on map — place city | Esc — cancel",
                hint_delete_city: "LMB on city — delete city | Esc — cancel",
                
                load_map_prompt: "Load Kenshi map",
            },
        }
    }
}

fn apply_modern_style(ctx: &egui::Context, theme: Theme) {
    let colors = match theme {
        Theme::Dark => ModernColors::dark(),
        Theme::Light => ModernColors::light(),
    };
    
    let mut style = (*ctx.style()).clone();
    
    // Киберпанк стиль с острыми углами
    style.visuals.window_rounding = Rounding::same(8.0);
    style.visuals.menu_rounding = Rounding::same(6.0);
    style.visuals.widgets.noninteractive.rounding = Rounding::same(4.0);
    style.visuals.widgets.inactive.rounding = Rounding::same(4.0);
    style.visuals.widgets.hovered.rounding = Rounding::same(4.0);
    style.visuals.widgets.active.rounding = Rounding::same(4.0);
    
    // Неоновые тени
    style.visuals.window_shadow = Shadow {
        offset: Vec2::new(0.0, 8.0),
        blur: 24.0,
        spread: 0.0,
        color: Color32::from_rgba_unmultiplied(88, 101, 242, 100),
    };
    
    style.visuals.popup_shadow = Shadow {
        offset: Vec2::new(0.0, 6.0),
        blur: 18.0,
        spread: 0.0,
        color: Color32::from_rgba_unmultiplied(88, 101, 242, 80),
    };
    
    // Темный фон как на сайте
    style.visuals.panel_fill = colors.bg_medium;
    style.visuals.window_fill = colors.bg_card;
    style.visuals.extreme_bg_color = colors.bg_dark;
    
    // Яркие виджеты с неоновым свечением
    style.visuals.widgets.noninteractive.bg_fill = colors.bg_medium;
    style.visuals.widgets.noninteractive.weak_bg_fill = colors.bg_light;
    style.visuals.widgets.inactive.bg_fill = colors.bg_light;
    style.visuals.widgets.inactive.weak_bg_fill = colors.bg_medium;
    style.visuals.widgets.hovered.bg_fill = colors.primary_hover;
    style.visuals.widgets.hovered.weak_bg_fill = colors.primary;
    style.visuals.widgets.active.bg_fill = colors.primary_dark;
    style.visuals.widgets.active.weak_bg_fill = colors.primary;
    
    // Яркий белый текст
    style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.5, colors.text_secondary);
    style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.5, colors.text_primary);
    style.visuals.widgets.hovered.fg_stroke = Stroke::new(1.5, Color32::WHITE);
    style.visuals.widgets.active.fg_stroke = Stroke::new(1.5, Color32::WHITE);
    
    // Неоновое выделение
    style.visuals.selection.bg_fill = colors.primary;
    style.visuals.selection.stroke = Stroke::new(2.0, colors.primary_hover);
    
    // Яркие ссылки
    style.visuals.hyperlink_color = colors.info;
    
    // Компактные отступы
    style.spacing.item_spacing = Vec2::new(8.0, 8.0);
    style.spacing.button_padding = Vec2::new(14.0, 8.0);
    style.spacing.window_margin = egui::Margin::same(12.0);
    style.spacing.menu_margin = egui::Margin::same(6.0);
    
    // Настройки пиксельного шрифта (сохраняем настройки из setup_pixel_fonts)
    if !style.text_styles.contains_key(&egui::TextStyle::Heading) {
        style.text_styles.insert(
            egui::TextStyle::Heading,
            egui::FontId::new(24.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Body,
            egui::FontId::new(15.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::new(15.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Small,
            egui::FontId::new(12.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Monospace,
            egui::FontId::new(14.0, egui::FontFamily::Monospace),
        );
    }
    
    ctx.set_style(style);
}

fn setup_pixel_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    
    // Загружаем шрифт Arcade Jeu из встроенного файла
    let arcade_font_data = include_bytes!("ArcadeJeu-Regular.otf");
    
    fonts.font_data.insert(
        "ArcadeJeu".to_owned(),
        egui::FontData::from_static(arcade_font_data),
    );
    
    // Устанавливаем Arcade Jeu как основной шрифт
    fonts.families.insert(
        egui::FontFamily::Proportional,
        vec!["ArcadeJeu".to_owned()],
    );
    
    fonts.families.insert(
        egui::FontFamily::Monospace,
        vec!["ArcadeJeu".to_owned()],
    );
    
    ctx.set_fonts(fonts);
    
    // Настраиваем размеры текста для аркадного пиксельного вида
    let mut style = (*ctx.style()).clone();
    
    style.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(24.0, egui::FontFamily::Monospace),
    );
    style.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(15.0, egui::FontFamily::Monospace),
    );
    style.text_styles.insert(
        egui::TextStyle::Button,
        egui::FontId::new(15.0, egui::FontFamily::Monospace),
    );
    style.text_styles.insert(
        egui::TextStyle::Small,
        egui::FontId::new(12.0, egui::FontFamily::Monospace),
    );
    style.text_styles.insert(
        egui::TextStyle::Monospace,
        egui::FontId::new(14.0, egui::FontFamily::Monospace),
    );
    
    // Увеличиваем межбуквенное расстояние для аркадного стиля
    style.spacing.item_spacing = Vec2::new(10.0, 10.0);
    
    ctx.set_style(style);
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_title("Kenshi Trade Map — Cyberpunk Edition"),
        ..Default::default()
    };

    eframe::run_native(
        "Kenshi Trade Map",
        options,
        Box::new(|cc| {
            setup_pixel_fonts(&cc.egui_ctx);
            apply_modern_style(&cc.egui_ctx, Theme::Dark);
            Box::new(KenshiTradeMap::default())
        }),
    )
}

#[derive(Clone, Serialize, Deserialize)]
struct City {
    id: usize,
    name: String,
    x: f32,
    y: f32,
}

#[derive(Clone, Serialize, Deserialize)]
struct TradeItem {
    name: String,
    buy_markup: f32,   // Наценка покупки %
    sell_markup: f32,  // Наценка продажи %
    #[serde(default)]
    hold: bool,        // Придержать
    #[serde(default)]
    sell: bool,        // Продавать
}

#[derive(Clone, Serialize, Deserialize)]
struct TradeRoute {
    id: usize,
    name: String,
    #[serde(skip, default)]
    start_point: Pos2,
    #[serde(skip, default)]
    end_point: Pos2,
    start_city_name: String,
    end_city_name: String,
    color: [u8; 3],
    items: Vec<TradeItem>,
    #[serde(default)]
    offset_x: f32,  // Смещение по X для дублирующихся маршрутов
    #[serde(default)]
    offset_y: f32,  // Смещение по Y для дублирующихся маршрутов
}

struct Camera {
    offset: Vec2,
    zoom: f32,
    target_zoom: f32,
}

impl Camera {
    fn world_to_screen(&self, world_pos: Pos2, screen_center: Pos2) -> Pos2 {
        let scaled = (world_pos.to_vec2() * self.zoom) + self.offset;
        screen_center + scaled
    }

    fn screen_to_world(&self, screen_pos: Pos2, screen_center: Pos2) -> Pos2 {
        let relative = screen_pos - screen_center;
        ((relative - self.offset) / self.zoom).to_pos2()
    }

    fn update_smooth_zoom(&mut self) {
        // Более плавная интерполяция (медленнее)
        let lerp_factor = 0.12;
        self.zoom += (self.target_zoom - self.zoom) * lerp_factor;
    }
}

enum AppState {
    Normal,
    PlacingCity,
    CreatingRoute(usize), // Содержит ID начального города
    DeletingCity, // Режим удаления города
}

struct KenshiTradeMap {
    cities: Vec<City>,
    routes: Vec<TradeRoute>,
    camera: Camera,
    map_texture: Option<egui::TextureHandle>,
    map_size: Vec2,
    
    // State
    state: AppState,
    hovered_city: Option<usize>,
    
    // UI State
    show_route_dialog: bool,
    route_name_input: String,
    route_color: Color32,
    pending_route: Option<(usize, usize)>, // (start_city_id, end_city_id)
    
    // City creation
    show_city_name_dialog: bool,
    city_name_input: String,
    pending_city_pos: Option<Pos2>,
    
    // City deletion
    show_delete_city_dialog: bool,
    pending_delete_city_id: Option<usize>,
    
    // Trade panel
    show_trade_panel: bool,
    active_route_id: Option<usize>,
    
    // Language & Theme
    language: Language,
    theme: Theme,
    
    // UI State
    show_top_panel: bool,
    
    // Watermark
    watermark_texture: Option<egui::TextureHandle>,
    
    // ID counters to prevent ID reuse after deletion
    next_city_id: usize,
    next_route_id: usize,
}

impl Default for KenshiTradeMap {
    fn default() -> Self {
        Self {
            cities: Vec::new(),
            routes: Vec::new(),
            camera: Camera {
                offset: Vec2::ZERO,
                zoom: 0.3,
                target_zoom: 0.3,
            },
            map_texture: None,
            map_size: Vec2::new(3000.0, 2000.0),
            state: AppState::Normal,
            hovered_city: None,
            show_route_dialog: false,
            route_name_input: String::new(),
            route_color: Color32::from_rgb(76, 175, 80),
            pending_route: None,
            show_city_name_dialog: false,
            city_name_input: String::new(),
            pending_city_pos: None,
            show_delete_city_dialog: false,
            pending_delete_city_id: None,
            show_trade_panel: false,
            active_route_id: None,
            language: Language::Russian,
            theme: Theme::Dark,
            show_top_panel: true,
            watermark_texture: None,
            next_city_id: 0,
            next_route_id: 0,
        }
    }
}

impl eframe::App for KenshiTradeMap {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.camera.update_smooth_zoom();

        // Apply modern theme
        apply_modern_style(ctx, self.theme);
        
        let colors = match self.theme {
            Theme::Dark => ModernColors::dark(),
            Theme::Light => ModernColors::light(),
        };

        let texts = Texts::get(self.language);

        // Top panel with stunning gradient design
        if self.show_top_panel {
            egui::TopBottomPanel::top("top_panel")
            .frame(egui::Frame::none()
                .fill(colors.bg_medium)
                .inner_margin(egui::Margin::symmetric(24.0, 16.0)))
            .show(ctx, |ui| {
                // Определяем доступную ширину для адаптивности
                let available_width = ui.available_width();
                
                // Адаптивные размеры в зависимости от ширины окна
                let (button_width, button_height, font_size, title_size, spacing) = if available_width < 900.0 {
                    // Очень маленькое окно
                    (110.0, 34.0, 8.0, 14.0, 2.0)
                } else if available_width < 1100.0 {
                    // Маленькое окно
                    (130.0, 36.0, 9.0, 16.0, 3.0)
                } else if available_width < 1280.0 {
                    // Среднее окно
                    (160.0, 38.0, 10.0, 18.0, 4.0)
                } else {
                    // Большое окно (по умолчанию)
                    (180.0, 40.0, 11.0, 20.0, 4.0)
                };
                
                // Первая строка: заголовок и основные кнопки
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing.x = spacing;
                    
                    // Logo/Title with gradient effect
                    let title_text = "Kenshi Trade Map";
                    self.draw_gradient_text_sized(ui, title_text, title_size, colors.gradient_start, colors.gradient_end);
                    
                    ui.add_space(spacing * 2.0);
                    ui.separator();
                    ui.add_space(spacing * 2.0);
                    
                    // Map controls with gradient buttons - БИРЮЗОВЫЙ
                    if self.draw_gradient_button_sized(ui, texts.load_map, Color32::from_rgb(0, 128, 128), Color32::from_rgb(0, 206, 209), Vec2::new(button_width, button_height), font_size)
                        .on_hover_text("Загрузить карту Kenshi")
                        .clicked() {
                        self.load_map_dialog(ctx);
                    }
                    
                    ui.add_space(spacing);
                    
                    // City controls with gradient buttons - ЗЕЛЁНЫЙ (оставляем)
                    if self.draw_gradient_button_sized(ui, texts.add_city, colors.success, colors.success_hover, Vec2::new(button_width, button_height), font_size)
                        .on_hover_text("Добавить новый город")
                        .clicked() {
                        self.state = AppState::PlacingCity;
                    }
                    
                    // КРАСНЫЙ (оставляем)
                    if self.draw_gradient_button_sized(ui, texts.delete_city, colors.danger, colors.danger_hover, Vec2::new(button_width, button_height), font_size)
                        .on_hover_text("Удалить город")
                        .clicked() {
                        self.state = AppState::DeletingCity;
                    }
                    
                    ui.add_space(spacing * 2.0);
                    ui.separator();
                    ui.add_space(spacing * 2.0);
                    
                    // Save/Load with gradient - БИРЮЗОВЫЙ
                    if self.draw_gradient_button_sized(ui, texts.save, Color32::from_rgb(0, 128, 128), Color32::from_rgb(0, 206, 209), Vec2::new(button_width, button_height), font_size)
                        .on_hover_text("Сохранить данные")
                        .clicked() {
                        self.save_data();
                    }
                    
                    if self.draw_gradient_button_sized(ui, texts.load, Color32::from_rgb(0, 128, 128), Color32::from_rgb(0, 206, 209), Vec2::new(button_width, button_height), font_size)
                        .on_hover_text("Загрузить данные")
                        .clicked() {
                        self.load_data();
                    }
                });
                
                // Вторая строка: статистика и кнопки языка/темы
                ui.add_space(spacing);
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing.x = spacing * 2.0;
                    
                    // Stats with gradient text (адаптивный размер)
                    let stat_size = if available_width < 900.0 { 9.0 } else if available_width < 1100.0 { 10.0 } else { 11.0 };
                    self.draw_gradient_text_sized(ui, &format!("{}: {}", texts.cities, self.cities.len()), stat_size, colors.success, colors.success_hover);
                    ui.add_space(spacing);
                    self.draw_gradient_text_sized(ui, &format!("{}: {}", texts.routes, self.routes.len()), stat_size, colors.info, colors.info_hover);
                    ui.add_space(spacing);
                    self.draw_gradient_text_sized(ui, &format!("{}: {:.1}x", texts.zoom, self.camera.zoom), stat_size, colors.accent, colors.accent_hover);
                    
                    ui.add_space(spacing * 3.0);
                    ui.separator();
                    ui.add_space(spacing * 3.0);
                    
                    // Language toggle - ТЁМНО-БИРЮЗОВЫЙ
                    if self.draw_gradient_button_sized(ui, texts.language, Color32::from_rgb(0, 128, 128), Color32::from_rgb(0, 206, 209), Vec2::new(button_width * 0.9, button_height), font_size)
                        .on_hover_text("Сменить язык")
                        .clicked() {
                        self.language = match self.language {
                            Language::Russian => Language::English,
                            Language::English => Language::Russian,
                        };
                    }
                    
                    ui.add_space(spacing);
                    
                    // Theme toggle - БИРЮЗОВЫЙ
                    if self.draw_gradient_button_sized(ui, texts.theme, Color32::from_rgb(0, 128, 128), Color32::from_rgb(0, 206, 209), Vec2::new(button_width * 0.9, button_height), font_size)
                        .on_hover_text("Переключить тему")
                        .clicked() {
                        self.theme = match self.theme {
                            Theme::Dark => Theme::Light,
                            Theme::Light => Theme::Dark,
                        };
                    }
                });
                
                // Toggle button bar at the bottom of panel - ИСПРАВЛЕНО
                ui.add_space(8.0);
                let panel_width = ui.available_width() + 48.0; // Полная ширина панели с отступами
                let toggle_height = 34.0;
                
                // Получаем текущую позицию
                let current_y = ui.min_rect().max.y;
                
                // Создаём rect на всю ширину окна (от 0 до panel_width)
                let toggle_rect = Rect::from_min_size(
                    Pos2::new(0.0, current_y),
                    Vec2::new(panel_width, toggle_height)
                );
                
                let toggle_response = ui.interact(toggle_rect, egui::Id::new("toggle_panel_bar"), egui::Sense::click());
                
                if toggle_response.clicked() {
                    self.show_top_panel = false;
                }
                
                // Draw toggle bar
                let painter = ui.painter();
                painter.rect_filled(
                    toggle_rect,
                    Rounding::ZERO,
                    if toggle_response.hovered() {
                        Color32::from_rgba_unmultiplied(88, 101, 242, 60)
                    } else {
                        Color32::from_rgba_unmultiplied(88, 101, 242, 30)
                    },
                );
                
                // Draw ^ symbol ТОЧНО в центре - используем центр rect
                let center_pos = toggle_rect.center();
                
                painter.text(
                    center_pos,
                    egui::Align2::CENTER_CENTER,
                    "^",
                    egui::FontId::monospace(20.0),
                    Color32::WHITE,
                );
            });
        } else {
            // Show toggle button when panel is hidden
            egui::TopBottomPanel::top("toggle_bar")
                .frame(egui::Frame::none())
                .exact_height(34.0)
                .show(ctx, |ui| {
                    let toggle_rect = ui.max_rect();
                    let toggle_response = ui.interact(toggle_rect, egui::Id::new("toggle_bar_hidden"), egui::Sense::click());
                    
                    if toggle_response.clicked() {
                        self.show_top_panel = true;
                    }
                    
                    // Draw toggle bar
                    let painter = ui.painter();
                    painter.rect_filled(
                        toggle_rect,
                        Rounding::ZERO,
                        if toggle_response.hovered() {
                            Color32::from_rgba_unmultiplied(88, 101, 242, 60)
                        } else {
                            Color32::from_rgba_unmultiplied(88, 101, 242, 30)
                        },
                    );
                    
                    // Draw v symbol ТОЧНО в центре
                    let center_pos = toggle_rect.center();
                    
                    painter.text(
                        center_pos,
                        egui::Align2::CENTER_CENTER,
                        "v",
                        egui::FontId::monospace(20.0),
                        Color32::WHITE,
                    );
                });
        }

        // Trade panel with modern design
        if self.show_trade_panel {
            egui::SidePanel::right("trade_panel")
                .default_width(420.0)
                .frame(egui::Frame::none()
                    .fill(colors.bg_medium)
                    .inner_margin(egui::Margin::same(16.0)))
                .show(ctx, |ui| {
                    self.draw_trade_panel(ui, &colors);
                });
        }

        // Main canvas with modern background
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(colors.bg_dark))
            .show(ctx, |ui| {
                let (response, mut painter) = ui.allocate_painter(
                    ui.available_size(),
                    egui::Sense::click_and_drag(),
                );

                let rect = response.rect;
                let center = rect.center();

                // Modern background with subtle gradient effect
                painter.rect_filled(rect, 0.0, colors.bg_dark);

                // Draw map if loaded
                if let Some(texture) = &self.map_texture {
                    let map_rect = Rect::from_center_size(
                        center + self.camera.offset,
                        self.map_size * self.camera.zoom,
                    );
                    painter.image(
                        texture.id(),
                        map_rect,
                        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                        Color32::WHITE,
                    );
                    
                    // Subtle overlay
                    painter.rect_filled(map_rect, 0.0, Color32::from_black_alpha(60));
                } else {
                    // Modern empty state without emoji
                    painter.text(
                        center,
                        egui::Align2::CENTER_CENTER,
                        texts.load_map_prompt,
                        egui::FontId::monospace(24.0),
                        colors.text_secondary,
                    );
                }

                // Draw routes
                for route in &self.routes {
                    self.draw_route(&painter, route, center);
                }

                // Draw cities with stunning glow effects
                self.hovered_city = None;
                for (idx, city) in self.cities.iter().enumerate() {
                    let screen_pos = self.camera.world_to_screen(
                        Pos2::new(city.x, city.y),
                        center,
                    );

                    if rect.contains(screen_pos) {
                        let radius = 10.0 * self.camera.zoom;
                        let is_hovered = response.hover_pos()
                            .map(|p| p.distance(screen_pos) < radius + 15.0)
                            .unwrap_or(false);

                        if is_hovered {
                            self.hovered_city = Some(idx);
                            
                            // Epic multi-layer glow effect
                            painter.circle_filled(
                                screen_pos,
                                radius + 35.0,
                                Color32::from_rgba_unmultiplied(99, 102, 241, 10),
                            );
                            painter.circle_filled(
                                screen_pos,
                                radius + 28.0,
                                Color32::from_rgba_unmultiplied(99, 102, 241, 20),
                            );
                            painter.circle_filled(
                                screen_pos,
                                radius + 20.0,
                                Color32::from_rgba_unmultiplied(139, 92, 246, 30),
                            );
                            painter.circle_filled(
                                screen_pos,
                                radius + 12.0,
                                Color32::from_rgba_unmultiplied(236, 72, 153, 50),
                            );
                        }

                        // Outer glow ring
                        painter.circle_filled(
                            screen_pos,
                            radius + 6.0,
                            if is_hovered {
                                Color32::from_rgba_unmultiplied(236, 72, 153, 80)
                            } else {
                                Color32::from_rgba_unmultiplied(99, 102, 241, 40)
                            },
                        );
                        
                        // Shadow for depth
                        painter.circle_filled(
                            screen_pos + Vec2::new(2.0, 3.0),
                            radius,
                            Color32::from_black_alpha(100),
                        );
                        
                        // Main circle with gradient effect
                        painter.circle_filled(
                            screen_pos,
                            radius,
                            if is_hovered {
                                colors.accent
                            } else {
                                colors.primary
                            },
                        );
                        
                        // Inner bright core
                        painter.circle_filled(
                            screen_pos - Vec2::new(2.0, 2.0),
                            radius * 0.5,
                            Color32::from_white_alpha(180),
                        );
                        
                        // Glossy highlight
                        painter.circle_filled(
                            screen_pos - Vec2::new(3.0, 3.0),
                            radius * 0.3,
                            Color32::from_white_alpha(220),
                        );
                        
                        // Outer border with glow
                        painter.circle_stroke(
                            screen_pos,
                            radius,
                            Stroke::new(2.5, if is_hovered { 
                                Color32::WHITE 
                            } else { 
                                colors.primary_hover 
                            }),
                        );

                        // City name with epic badge
                        if is_hovered {
                            let name_pos = screen_pos + Vec2::new(0.0, -radius - 28.0);
                            
                            // Badge background with shadow
                            let text_size = painter.fonts(|f| {
                                f.layout_no_wrap(
                                    city.name.clone(),
                                    egui::FontId::monospace(15.0),
                                    Color32::WHITE,
                                ).size()
                            });
                            
                            let badge_rect = Rect::from_center_size(
                                name_pos,
                                text_size + Vec2::new(24.0, 14.0),
                            );
                            
                            // Glow effect
                            painter.rect_filled(
                                badge_rect.expand(4.0),
                                Rounding::same(10.0),
                                Color32::from_rgba_unmultiplied(236, 72, 153, 30),
                            );
                            
                            // Shadow
                            painter.rect_filled(
                                badge_rect.translate(Vec2::new(0.0, 3.0)),
                                Rounding::same(8.0),
                                Color32::from_black_alpha(120),
                            );
                            
                            // Badge with gradient
                            painter.rect_filled(
                                badge_rect,
                                Rounding::same(8.0),
                                colors.primary,
                            );
                            
                            // Border
                            painter.rect_stroke(
                                badge_rect,
                                Rounding::same(8.0),
                                Stroke::new(2.0, colors.accent),
                            );
                            
                            // Text with shadow
                            painter.text(
                                name_pos + Vec2::new(1.0, 1.0),
                                egui::Align2::CENTER_CENTER,
                                &city.name,
                                egui::FontId::monospace(15.0),
                                Color32::from_black_alpha(100),
                            );
                            painter.text(
                                name_pos,
                                egui::Align2::CENTER_CENTER,
                                &city.name,
                                egui::FontId::monospace(15.0),
                                Color32::WHITE,
                            );
                        }
                    }
                }

                self.handle_input(&response, center, ctx);

                // Epic hints with glowing badge
                let hint_text = match &self.state {
                    AppState::CreatingRoute(_) => texts.hint_route,
                    AppState::PlacingCity => texts.hint_city,
                    AppState::DeletingCity => texts.hint_delete_city,
                    _ => "",
                };

                if !hint_text.is_empty() {
                    let hint_pos = Pos2::new(rect.center().x, rect.max.y - 50.0);
                    
                    // Calculate text size
                    let text_size = painter.fonts(|f| {
                        f.layout_no_wrap(
                            hint_text.to_string(),
                            egui::FontId::monospace(17.0),
                            Color32::WHITE,
                        ).size()
                    });
                    
                    let badge_rect = Rect::from_center_size(
                        hint_pos,
                        text_size + Vec2::new(40.0, 20.0),
                    );
                    
                    // Outer glow
                    painter.rect_filled(
                        badge_rect.expand(8.0),
                        Rounding::same(14.0),
                        Color32::from_rgba_unmultiplied(59, 130, 246, 20),
                    );
                    painter.rect_filled(
                        badge_rect.expand(4.0),
                        Rounding::same(12.0),
                        Color32::from_rgba_unmultiplied(59, 130, 246, 40),
                    );
                    
                    // Shadow
                    painter.rect_filled(
                        badge_rect.translate(Vec2::new(0.0, 4.0)),
                        Rounding::same(10.0),
                        Color32::from_black_alpha(140),
                    );
                    
                    // Badge background with gradient
                    painter.rect_filled(
                        badge_rect,
                        Rounding::same(10.0),
                        colors.info,
                    );
                    
                    // Border with glow
                    painter.rect_stroke(
                        badge_rect,
                        Rounding::same(10.0),
                        Stroke::new(2.5, colors.info_hover),
                    );
                    
                    // Text with shadow
                    painter.text(
                        hint_pos + Vec2::new(1.0, 1.0),
                        egui::Align2::CENTER_CENTER,
                        hint_text,
                        egui::FontId::monospace(17.0),
                        Color32::from_black_alpha(120),
                    );
                    painter.text(
                        hint_pos,
                        egui::Align2::CENTER_CENTER,
                        hint_text,
                        egui::FontId::monospace(17.0),
                        Color32::WHITE,
                    );
                }

                // Load watermark on first frame if not loaded
                if self.watermark_texture.is_none() {
                    self.load_watermark(ctx);
                }

                // Draw watermark in bottom-right corner with modern effects
                if let Some(watermark) = &self.watermark_texture {
                    let watermark_size = Vec2::new(130.0, 130.0); // Уменьшено со 150 до 130
                    let watermark_pos = Pos2::new(
                        rect.max.x - watermark_size.x - 5.0, // Ближе к краю (было 10.0)
                        rect.max.y - watermark_size.y - 5.0, // Ближе к краю (было 10.0)
                    );
                    let watermark_rect = Rect::from_min_size(watermark_pos, watermark_size);
                    
                    // Draw shadow for watermark
                    let shadow_offset = Vec2::new(0.0, 4.0);
                    let shadow_rect = watermark_rect.translate(shadow_offset);
                    painter.rect_filled(
                        shadow_rect,
                        Rounding::same(12.0),
                        Color32::from_black_alpha(80),
                    );
                    
                    // Draw watermark image with rounded corners using clip rect
                    painter.set_clip_rect(watermark_rect);
                    
                    // Draw watermark image
                    painter.image(
                        watermark.id(),
                        watermark_rect,
                        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                        Color32::WHITE,
                    );
                    
                    // Reset clip rect
                    painter.set_clip_rect(rect);
                    
                    // Check if mouse is hovering over watermark
                    if let Some(hover_pos) = response.hover_pos() {
                        if watermark_rect.contains(hover_pos) {
                            // Show tooltip with modern styling
                            egui::show_tooltip_at_pointer(ctx, egui::Id::new("watermark_tooltip"), |ui| {
                                ui.label(egui::RichText::new("mushoku tensei | Реинкарнация безработного")
                                    .size(13.0)
                                    .color(colors.text_primary));
                            });
                        }
                    }
                }
            });

        // Route dialog
        // Route dialog with modern design
        if self.show_route_dialog {
            egui::Window::new(egui::RichText::new(texts.new_route).size(18.0).strong())
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .frame(egui::Frame::window(&ctx.style())
                    .fill(colors.bg_card)
                    .rounding(Rounding::same(16.0))
                    .inner_margin(egui::Margin::same(20.0)))
                .show(ctx, |ui| {
                    ui.add_space(4.0);
                    
                    ui.label(egui::RichText::new(texts.route_name).size(14.0).color(colors.text_secondary));
                    ui.add_space(4.0);
                    let text_edit = egui::TextEdit::singleline(&mut self.route_name_input)
                        .desired_width(300.0)
                        .margin(Vec2::new(12.0, 8.0));
                    ui.add(text_edit);
                    
                    ui.add_space(16.0);
                    
                    ui.label(egui::RichText::new(texts.route_color).size(14.0).color(colors.text_secondary));
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        egui::color_picker::color_edit_button_srgba(
                            ui,
                            &mut self.route_color,
                            egui::color_picker::Alpha::Opaque,
                        );
                        
                        ui.add_space(8.0);
                        
                        // Предустановленные цвета без эмодзи
                        let preset_colors = [
                            Color32::from_rgb(76, 175, 80),   // Зеленый
                            Color32::from_rgb(33, 150, 243),  // Синий
                            Color32::from_rgb(255, 152, 0),   // Оранжевый
                            Color32::from_rgb(233, 30, 99),   // Розовый
                            Color32::from_rgb(0, 188, 212),   // Голубой
                            Color32::from_rgb(255, 235, 59),  // Желтый
                            Color32::from_rgb(156, 39, 176),  // Фиолетовый
                            Color32::from_rgb(244, 67, 54),   // Красный
                        ];
                        
                        for color in preset_colors {
                            let button = egui::Button::new("")
                                .fill(color)
                                .rounding(Rounding::same(6.0))
                                .min_size(Vec2::new(32.0, 32.0));
                            if ui.add(button).clicked() {
                                self.route_color = color;
                            }
                        }
                    });
                    
                    ui.add_space(20.0);
                    
                    ui.horizontal(|ui| {
                        if ui.add(egui::Button::new(egui::RichText::new(texts.create).size(14.0).color(Color32::WHITE))
                            .fill(colors.success)
                            .rounding(Rounding::same(8.0))
                            .min_size(Vec2::new(100.0, 36.0)))
                            .clicked() {
                            self.confirm_route_creation();
                        }
                        
                        ui.add_space(8.0);
                        
                        if ui.add(egui::Button::new(egui::RichText::new(texts.cancel).size(14.0))
                            .fill(colors.bg_light)
                            .rounding(Rounding::same(8.0))
                            .min_size(Vec2::new(100.0, 36.0)))
                            .clicked() {
                            self.cancel_route_creation();
                        }
                    });
                });
        }

        // City name dialog with modern design
        if self.show_city_name_dialog {
            egui::Window::new(egui::RichText::new(texts.new_city).size(18.0).strong())
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .frame(egui::Frame::window(&ctx.style())
                    .fill(colors.bg_card)
                    .rounding(Rounding::same(16.0))
                    .inner_margin(egui::Margin::same(20.0)))
                .show(ctx, |ui| {
                    ui.add_space(4.0);
                    
                    ui.label(egui::RichText::new(texts.city_name).size(14.0).color(colors.text_secondary));
                    ui.add_space(4.0);
                    let text_edit = egui::TextEdit::singleline(&mut self.city_name_input)
                        .desired_width(300.0)
                        .margin(Vec2::new(12.0, 8.0));
                    let response = ui.add(text_edit);
                    
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        self.confirm_city_creation();
                    }
                    
                    ui.add_space(20.0);
                    
                    ui.horizontal(|ui| {
                        if ui.add(egui::Button::new(egui::RichText::new(texts.confirm).size(14.0).color(Color32::WHITE))
                            .fill(colors.success)
                            .rounding(Rounding::same(8.0))
                            .min_size(Vec2::new(100.0, 36.0)))
                            .clicked() {
                            self.confirm_city_creation();
                        }
                        
                        ui.add_space(8.0);
                        
                        if ui.add(egui::Button::new(egui::RichText::new(texts.cancel).size(14.0))
                            .fill(colors.bg_light)
                            .rounding(Rounding::same(8.0))
                            .min_size(Vec2::new(100.0, 36.0)))
                            .clicked() {
                            self.cancel_city_creation();
                        }
                    });
                });
        }

        // Delete city dialog with modern design
        if self.show_delete_city_dialog {
            egui::Window::new(egui::RichText::new(texts.delete_city_title).size(18.0).strong())
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .frame(egui::Frame::window(&ctx.style())
                    .fill(colors.bg_card)
                    .rounding(Rounding::same(16.0))
                    .inner_margin(egui::Margin::same(20.0)))
                .show(ctx, |ui| {
                    ui.add_space(4.0);
                    
                    ui.label(egui::RichText::new(texts.delete_city_message).size(14.0).color(colors.text_secondary));
                    
                    ui.add_space(20.0);
                    
                    ui.horizontal(|ui| {
                        if ui.add(egui::Button::new(egui::RichText::new(texts.confirm).size(14.0).color(Color32::WHITE))
                            .fill(colors.danger)
                            .rounding(Rounding::same(8.0))
                            .min_size(Vec2::new(120.0, 36.0)))
                            .clicked() {
                            self.confirm_city_deletion();
                        }
                        
                        ui.add_space(8.0);
                        
                        if ui.add(egui::Button::new(egui::RichText::new(texts.cancel).size(14.0))
                            .fill(colors.bg_light)
                            .rounding(Rounding::same(8.0))
                            .min_size(Vec2::new(120.0, 36.0)))
                            .clicked() {
                            self.cancel_city_deletion();
                        }
                    });
                });
        }

        // Context menu для создания маршрута
        if let Some(city_idx) = self.hovered_city {
            if ctx.input(|i| i.pointer.secondary_clicked()) {
                self.start_route_creation(city_idx);
            }
        }

        ctx.request_repaint();
    }
}

impl KenshiTradeMap {
    // Функция для рисования текста с градиентом (адаптивный размер)
    fn draw_gradient_text_sized(&self, ui: &mut egui::Ui, text: &str, size: f32, color_start: Color32, color_end: Color32) {
        let (rect, _response) = ui.allocate_exact_size(
            Vec2::new(ui.fonts(|f| f.layout_no_wrap(text.to_string(), egui::FontId::monospace(size), Color32::WHITE).size().x), size * 1.2),
            egui::Sense::hover()
        );
        
        let painter = ui.painter();
        
        // Рисуем каждую букву с градиентом
        let char_count = text.chars().count() as f32;
        let mut x_offset = rect.min.x;
        
        for (i, ch) in text.chars().enumerate() {
            let t = i as f32 / char_count.max(1.0);
            let color = Color32::from_rgb(
                (color_start.r() as f32 * (1.0 - t) + color_end.r() as f32 * t) as u8,
                (color_start.g() as f32 * (1.0 - t) + color_end.g() as f32 * t) as u8,
                (color_start.b() as f32 * (1.0 - t) + color_end.b() as f32 * t) as u8,
            );
            
            let char_str = ch.to_string();
            let char_width = ui.fonts(|f| f.layout_no_wrap(char_str.clone(), egui::FontId::monospace(size), color).size().x);
            
            painter.text(
                Pos2::new(x_offset, rect.center().y),
                egui::Align2::LEFT_CENTER,
                &char_str,
                egui::FontId::monospace(size),
                color,
            );
            
            x_offset += char_width;
        }
    }
    
    // Функция для рисования текста с градиентом (старая версия для совместимости)
    fn draw_gradient_text(&self, ui: &mut egui::Ui, text: &str, size: f32, color_start: Color32, color_end: Color32) {
        self.draw_gradient_text_sized(ui, text, size, color_start, color_end);
    }
    
    // Функция для рисования кнопки с градиентом (адаптивный размер)
    fn draw_gradient_button_sized(&self, ui: &mut egui::Ui, text: &str, color_start: Color32, color_end: Color32, min_size: Vec2, font_size: f32) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(min_size, egui::Sense::click());
        
        let painter = ui.painter();
        
        // Овальная форма - радиус закругления = половина высоты
        let oval_rounding = Rounding::same(rect.height() / 2.0);
        
        // ПРЯМОУГОЛЬНОЕ свечение с острыми углами (4.0 радиус)
        let sharp_rounding = Rounding::same(4.0);
        
        // СНАЧАЛА рисуем свечение при наведении (под градиентом) - ПРЯМОУГОЛЬНОЕ
        if response.hovered() {
            // Внешнее свечение (больше кнопки) - ПРЯМОУГОЛЬНОЕ с острыми углами
            painter.rect_filled(
                rect.expand(4.0),
                sharp_rounding,
                Color32::from_white_alpha(20),
            );
            painter.rect_filled(
                rect.expand(2.0),
                sharp_rounding,
                Color32::from_white_alpha(30),
            );
        }
        
        // ПОТОМ рисуем фон кнопки с градиентом
        let gradient_steps = 20;
        let step_height = rect.height() / gradient_steps as f32;
        
        for i in 0..gradient_steps {
            let t = i as f32 / gradient_steps as f32;
            let color = Color32::from_rgb(
                (color_start.r() as f32 * (1.0 - t) + color_end.r() as f32 * t) as u8,
                (color_start.g() as f32 * (1.0 - t) + color_end.g() as f32 * t) as u8,
                (color_start.b() as f32 * (1.0 - t) + color_end.b() as f32 * t) as u8,
            );
            
            let step_rect = Rect::from_min_size(
                Pos2::new(rect.min.x, rect.min.y + i as f32 * step_height),
                Vec2::new(rect.width(), step_height + 1.0),
            );
            
            painter.rect_filled(step_rect, oval_rounding, color);
        }
        
        // Дополнительное свечение поверх кнопки при наведении - ПРЯМОУГОЛЬНОЕ
        if response.hovered() {
            painter.rect_filled(rect, sharp_rounding, Color32::from_white_alpha(25));
        }
        
        // Рисуем текст ЧЁРНЫМ цветом (вместо градиента)
        let text_color = Color32::BLACK;
        
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::monospace(font_size),
            text_color,
        );
        
        response
    }
    
    // Функция для рисования кнопки с градиентом (старая версия для совместимости)
    fn draw_gradient_button(&self, ui: &mut egui::Ui, text: &str, color_start: Color32, color_end: Color32, min_size: Vec2) -> egui::Response {
        self.draw_gradient_button_sized(ui, text, color_start, color_end, min_size, 14.0)
    }
    
    fn draw_route(&self, painter: &egui::Painter, route: &TradeRoute, center: Pos2) {
        let color = Color32::from_rgb(route.color[0], route.color[1], route.color[2]);
        let glow_color = Color32::from_rgba_unmultiplied(route.color[0], route.color[1], route.color[2], 60);
        
        let p1 = self.camera.world_to_screen(route.start_point, center);
        let p2 = self.camera.world_to_screen(route.end_point, center);
        
        // Glow effect for route
        painter.line_segment([p1, p2], Stroke::new(12.0 * self.camera.zoom, glow_color));
        painter.line_segment([p1, p2], Stroke::new(9.0 * self.camera.zoom, glow_color));
        
        // Main route line with gradient effect
        painter.line_segment([p1, p2], Stroke::new(7.0 * self.camera.zoom, color));
        
        // Bright center line
        let bright_color = Color32::from_rgba_unmultiplied(
            ((color.r() as u16 + 100).min(255)) as u8,
            ((color.g() as u16 + 100).min(255)) as u8,
            ((color.b() as u16 + 100).min(255)) as u8,
            255,
        );
        painter.line_segment([p1, p2], Stroke::new(3.0 * self.camera.zoom, bright_color));
        
        // Enhanced arrow at the end
        let direction = (p2 - p1).normalized();
        let arrow_size = 18.0 * self.camera.zoom;
        let arrow_angle = std::f32::consts::PI / 5.5;
        
        let arrow_left = p2 - direction.rot90() * arrow_size * arrow_angle.sin() - direction * arrow_size * arrow_angle.cos();
        let arrow_right = p2 + direction.rot90() * arrow_size * arrow_angle.sin() - direction * arrow_size * arrow_angle.cos();
        
        // Arrow glow
        painter.line_segment([p2, arrow_left], Stroke::new(8.0 * self.camera.zoom, glow_color));
        painter.line_segment([p2, arrow_right], Stroke::new(8.0 * self.camera.zoom, glow_color));
        
        // Arrow main lines
        painter.line_segment([p2, arrow_left], Stroke::new(6.0 * self.camera.zoom, color));
        painter.line_segment([p2, arrow_right], Stroke::new(6.0 * self.camera.zoom, color));
        
        // Arrow bright lines
        painter.line_segment([p2, arrow_left], Stroke::new(3.0 * self.camera.zoom, bright_color));
        painter.line_segment([p2, arrow_right], Stroke::new(3.0 * self.camera.zoom, bright_color));
        
        // Enhanced start and end points with glow
        painter.circle_filled(p1, 8.0 * self.camera.zoom, glow_color);
        painter.circle_filled(p1, 6.0 * self.camera.zoom, color);
        painter.circle_filled(p1, 3.0 * self.camera.zoom, bright_color);
        
        painter.circle_filled(p2, 8.0 * self.camera.zoom, glow_color);
        painter.circle_filled(p2, 6.0 * self.camera.zoom, color);
        painter.circle_filled(p2, 3.0 * self.camera.zoom, bright_color);
    }

    fn handle_input(&mut self, response: &egui::Response, center: Pos2, ctx: &egui::Context) {
        // Zoom with normalized and slower speed, centered on cursor
        if let Some(hover_pos) = response.hover_pos() {
            let scroll_delta = ctx.input(|i| i.smooth_scroll_delta.y);
            if scroll_delta != 0.0 {
                // Нормализуем скорость скролла (ограничиваем максимальное значение)
                let normalized_delta = scroll_delta.clamp(-50.0, 50.0);
                
                // Более медленный zoom factor (было 1.15, стало 1.08)
                let zoom_speed = 1.0 + (normalized_delta / 500.0);
                let zoom_factor = zoom_speed.clamp(0.92, 1.08);
                
                // Сохраняем мировую позицию под курсором
                let world_pos_before = self.camera.screen_to_world(hover_pos, center);
                
                // Применяем новый зум
                self.camera.target_zoom = (self.camera.target_zoom * zoom_factor).clamp(0.1, 3.0);
                
                // Корректируем offset так, чтобы мировая позиция под курсором осталась на месте
                // Вычисляем где будет эта точка после зума
                let world_pos_after_screen = self.camera.world_to_screen(world_pos_before, center);
                
                // Смещаем offset чтобы компенсировать разницу
                self.camera.offset += hover_pos - world_pos_after_screen;
            }
        }

        // Pan
        if response.dragged_by(egui::PointerButton::Primary) {
            match self.state {
                AppState::CreatingRoute(_) | AppState::PlacingCity | AppState::DeletingCity => {}
                _ => {
                    if self.hovered_city.is_none() {
                        self.camera.offset += response.drag_delta();
                    }
                }
            }
        }

        // Click handling
        if response.clicked() {
            match &self.state {
                AppState::PlacingCity => {
                    if let Some(pos) = response.hover_pos() {
                        let world_pos = self.camera.screen_to_world(pos, center);
                        self.pending_city_pos = Some(world_pos);
                        self.show_city_name_dialog = true;
                    }
                }
                AppState::CreatingRoute(start_city_id) => {
                    if let Some(end_city_idx) = self.hovered_city {
                        let end_city_id = self.cities[end_city_idx].id;
                        if *start_city_id != end_city_id {
                            self.pending_route = Some((*start_city_id, end_city_id));
                            self.show_route_dialog = true;
                        }
                    }
                }
                AppState::DeletingCity => {
                    if let Some(city_idx) = self.hovered_city {
                        let city_id = self.cities[city_idx].id;
                        self.pending_delete_city_id = Some(city_id);
                        self.show_delete_city_dialog = true;
                    }
                }
                AppState::Normal => {
                    if let Some(pos) = response.hover_pos() {
                        let world_pos = self.camera.screen_to_world(pos, center);
                        if let Some(route_id) = self.find_route_at(world_pos) {
                            self.active_route_id = Some(route_id);
                            self.show_trade_panel = true;
                        }
                    }
                }
            }
        }

        // Keyboard
        ctx.input(|i| {
            if i.key_pressed(egui::Key::Escape) {
                match self.state {
                    AppState::CreatingRoute(_) => {
                        self.state = AppState::Normal;
                    }
                    AppState::PlacingCity => {
                        self.cancel_city_creation();
                    }
                    AppState::DeletingCity => {
                        self.cancel_city_deletion();
                    }
                    _ => {
                        self.show_trade_panel = false;
                    }
                }
            }
        });
    }

    fn start_route_creation(&mut self, city_idx: usize) {
        let city_id = self.cities[city_idx].id;
        self.state = AppState::CreatingRoute(city_id);
    }

    fn confirm_route_creation(&mut self) {
        if let Some((start_id, end_id)) = self.pending_route {
            let start_city = self.cities.iter().find(|c| c.id == start_id).unwrap();
            let end_city = self.cities.iter().find(|c| c.id == end_id).unwrap();
            
            let name = if self.route_name_input.is_empty() {
                format!("{} → {}", start_city.name, end_city.name)
            } else {
                self.route_name_input.clone()
            };

            // Проверяем, есть ли уже маршруты между этими городами
            let existing_routes_count = self.routes.iter()
                .filter(|r| {
                    (r.start_city_name == start_city.name && r.end_city_name == end_city.name) ||
                    (r.start_city_name == end_city.name && r.end_city_name == start_city.name)
                })
                .count();

            // Вычисляем смещение для маршрута
            let base_start = Pos2::new(start_city.x, start_city.y);
            let base_end = Pos2::new(end_city.x, end_city.y);
            let mut start_point = base_start;
            let mut end_point = base_end;
            
            let mut offset_x = 0.0;
            let mut offset_y = 0.0;
            
            if existing_routes_count > 0 {
                // Смещаем маршрут перпендикулярно линии между городами
                let direction = (base_end - base_start).normalized();
                let perpendicular = Vec2::new(-direction.y, direction.x);
                let offset_distance = 15.0 * (existing_routes_count as f32);
                let offset = perpendicular * offset_distance;
                
                offset_x = offset.x;
                offset_y = offset.y;
                
                start_point += offset;
                end_point += offset;
            }

            let route = TradeRoute {
                id: self.next_route_id,
                name,
                start_point,
                end_point,
                start_city_name: start_city.name.clone(),
                end_city_name: end_city.name.clone(),
                color: [self.route_color.r(), self.route_color.g(), self.route_color.b()],
                items: Vec::new(),
                offset_x,
                offset_y,
            };

            self.routes.push(route);
            self.next_route_id += 1;
        }
        self.cancel_route_creation();
    }

    fn cancel_route_creation(&mut self) {
        self.state = AppState::Normal;
        self.show_route_dialog = false;
        self.route_name_input.clear();
        self.pending_route = None;
        self.route_color = Color32::from_rgb(76, 175, 80);
    }

    fn confirm_city_creation(&mut self) {
        if let Some(pos) = self.pending_city_pos {
            let name = if self.city_name_input.is_empty() {
                format!("Город {}", self.next_city_id + 1)
            } else {
                self.city_name_input.clone()
            };

            let city = City {
                id: self.next_city_id,
                name,
                x: pos.x,
                y: pos.y,
            };

            self.cities.push(city);
            self.next_city_id += 1;
        }
        self.cancel_city_creation();
    }

    fn cancel_city_creation(&mut self) {
        self.state = AppState::Normal;
        self.show_city_name_dialog = false;
        self.city_name_input.clear();
        self.pending_city_pos = None;
    }

    fn confirm_city_deletion(&mut self) {
        if let Some(city_id) = self.pending_delete_city_id {
            // Находим город по ID
            if let Some(city_idx) = self.cities.iter().position(|c| c.id == city_id) {
                let city_name = self.cities[city_idx].name.clone();
                
                // Удаляем все маршруты, связанные с этим городом
                self.routes.retain(|route| {
                    route.start_city_name != city_name && route.end_city_name != city_name
                });
                
                // Удаляем город
                self.cities.remove(city_idx);
            }
        }
        self.cancel_city_deletion();
    }

    fn cancel_city_deletion(&mut self) {
        self.state = AppState::Normal;
        self.show_delete_city_dialog = false;
        self.pending_delete_city_id = None;
    }

    fn find_route_at(&self, world_pos: Pos2) -> Option<usize> {
        let threshold = 20.0 / self.camera.zoom;
        
        for route in &self.routes {
            let dist = Self::distance_to_segment(world_pos, route.start_point, route.end_point);
            if dist < threshold {
                return Some(route.id);
            }
        }
        
        None
    }

    fn distance_to_segment(p: Pos2, a: Pos2, b: Pos2) -> f32 {
        let ab = b - a;
        let ap = p - a;
        let t = (ap.dot(ab) / ab.length_sq()).clamp(0.0, 1.0);
        let closest = a + ab * t;
        p.distance(closest)
    }

    fn draw_trade_panel(&mut self, ui: &mut egui::Ui, colors: &ModernColors) {
        let texts = Texts::get(self.language);
        
        ui.heading(egui::RichText::new(texts.trade_route).size(20.0).strong().color(colors.primary));
        ui.add_space(8.0);
        
        let mut should_delete_route = false;
        
        if let Some(route_id) = self.active_route_id {
            if let Some(route) = self.routes.iter_mut().find(|r| r.id == route_id) {
                // Route info card
                ui.group(|ui| {
                    ui.set_min_width(ui.available_width());
                    ui.label(egui::RichText::new(&route.name).size(16.0).strong().color(colors.text_primary));
                    ui.label(egui::RichText::new(format!("{} → {}", route.start_city_name, route.end_city_name))
                        .size(13.0)
                        .color(colors.text_secondary));
                });
                
                ui.add_space(12.0);
                ui.separator();
                ui.add_space(12.0);
                
                // ScrollArea теперь охватывает ВСЁ содержимое панели (товары + кнопки)
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                    let mut to_remove = None;
                    
                    // Список товаров
                    for (idx, item) in route.items.iter_mut().enumerate() {
                        ui.group(|ui| {
                            ui.set_min_width(ui.available_width() - 20.0);
                            
                            ui.label(egui::RichText::new(texts.item).size(13.0).color(colors.text_secondary));
                            ui.add_space(4.0);
                            let text_edit = egui::TextEdit::singleline(&mut item.name)
                                .desired_width(ui.available_width())
                                .margin(Vec2::new(8.0, 6.0));
                            ui.add(text_edit);
                            
                            ui.add_space(8.0);
                            
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(texts.buy_markup).size(13.0).color(colors.text_secondary));
                                ui.add(egui::DragValue::new(&mut item.buy_markup)
                                    .speed(0.5)
                                    .suffix("%"));
                            });
                            
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(texts.sell_markup).size(13.0).color(colors.text_secondary));
                                ui.add(egui::DragValue::new(&mut item.sell_markup)
                                    .speed(0.5)
                                    .suffix("%"));
                            });
                            
                            ui.add_space(8.0);
                            
                            ui.horizontal(|ui| {
                                ui.checkbox(&mut item.hold, egui::RichText::new(texts.hold).size(13.0));
                                ui.add_space(10.0);
                                ui.checkbox(&mut item.sell, egui::RichText::new(texts.sell).size(13.0));
                                
                                ui.add_space(ui.available_width() - 40.0);
                                if ui.add(egui::Button::new(egui::RichText::new("X").size(14.0).color(Color32::WHITE))
                                    .fill(colors.danger)
                                    .rounding(Rounding::same(6.0))
                                    .min_size(Vec2::new(32.0, 32.0)))
                                    .on_hover_text("Удалить товар")
                                    .clicked() {
                                    to_remove = Some(idx);
                                }
                            });
                        });
                        
                        ui.add_space(8.0);
                    }
                    
                    if let Some(idx) = to_remove {
                        route.items.remove(idx);
                    }
                    
                    // Кнопки теперь ВНУТРИ ScrollArea
                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(12.0);
                    
                    if ui.add(egui::Button::new(egui::RichText::new(texts.add_item).size(14.0).color(Color32::WHITE))
                        .fill(colors.success)
                        .rounding(Rounding::same(8.0))
                        .min_size(Vec2::new(ui.available_width(), 40.0)))
                        .clicked() {
                        route.items.push(TradeItem {
                            name: String::new(),
                            buy_markup: 0.0,
                            sell_markup: 0.0,
                            hold: false,
                            sell: false,
                        });
                    }
                    
                    ui.add_space(8.0);
                    
                    if ui.add(egui::Button::new(egui::RichText::new(texts.delete_route).size(14.0).color(Color32::WHITE))
                        .fill(colors.danger)
                        .rounding(Rounding::same(8.0))
                        .min_size(Vec2::new(ui.available_width(), 40.0)))
                        .clicked() {
                        should_delete_route = true;
                    }
                });
            }
        }
        
        // Удаляем маршрут после выхода из заимствования
        if should_delete_route {
            if let Some(route_id) = self.active_route_id {
                self.routes.retain(|r| r.id != route_id);
                self.show_trade_panel = false;
                self.active_route_id = None;
                return;
            }
        }
        
        ui.add_space(12.0);
        ui.separator();
        ui.add_space(12.0);
        
        if ui.add(egui::Button::new(egui::RichText::new(texts.close).size(14.0))
            .fill(colors.bg_light)
            .rounding(Rounding::same(8.0))
            .min_size(Vec2::new(ui.available_width(), 40.0)))
            .clicked() {
            self.show_trade_panel = false;
        }
    }

    fn load_map_dialog(&mut self, ctx: &egui::Context) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Images", &["png", "jpg", "jpeg"])
            .pick_file()
        {
            if let Ok(img) = image::open(&path) {
                self.load_map_texture(ctx, img);
            }
        }
    }

    fn load_map_texture(&mut self, ctx: &egui::Context, img: DynamicImage) {
        let size = [img.width() as usize, img.height() as usize];
        let img_buffer = img.to_rgba8();
        let pixels = img_buffer.as_flat_samples();
        
        let color_image = egui::ColorImage::from_rgba_unmultiplied(
            size,
            pixels.as_slice(),
        );
        
        self.map_texture = Some(ctx.load_texture("map", color_image, Default::default()));
        self.map_size = Vec2::new(size[0] as f32, size[1] as f32);
        
        self.camera.zoom = 0.3;
        self.camera.target_zoom = 0.3;
        self.camera.offset = Vec2::ZERO;
    }

    fn load_watermark(&mut self, ctx: &egui::Context) {
        // Загружаем встроенную ватермарку из байтов (встроена в исполняемый файл)
        // Это изображение нельзя удалить или изменить без перекомпиляции приложения
        if let Ok(img) = image::load_from_memory(WATERMARK_BYTES) {
            let size = [img.width() as usize, img.height() as usize];
            let img_buffer = img.to_rgba8();
            let pixels = img_buffer.as_flat_samples();
            
            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                size,
                pixels.as_slice(),
            );
            
            self.watermark_texture = Some(ctx.load_texture("watermark", color_image, Default::default()));
        }
    }

    fn save_data(&self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("JSON", &["json"])
            .set_file_name("kenshi_data.json")
            .save_file()
        {
            #[derive(Serialize)]
            struct SaveData {
                cities: Vec<City>,
                routes: Vec<TradeRoute>,
            }

            let data = SaveData {
                cities: self.cities.clone(),
                routes: self.routes.clone(),
            };

            if let Ok(json) = serde_json::to_string_pretty(&data) {
                let _ = std::fs::write(path, json);
            }
        }
    }

    fn load_data(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("JSON", &["json"])
            .pick_file()
        {
            if let Ok(content) = std::fs::read_to_string(path) {
                #[derive(Deserialize)]
                struct SaveData {
                    cities: Vec<City>,
                    routes: Vec<TradeRoute>,
                }

                if let Ok(mut data) = serde_json::from_str::<SaveData>(&content) {
                    self.cities = data.cities;
                    
                    // Reconstruct points from city names with saved offset
                    for route in &mut data.routes {
                        if let Some(start_city) = self.cities.iter().find(|c| c.name == route.start_city_name) {
                            route.start_point = Pos2::new(
                                start_city.x + route.offset_x,
                                start_city.y + route.offset_y
                            );
                        }
                        if let Some(end_city) = self.cities.iter().find(|c| c.name == route.end_city_name) {
                            route.end_point = Pos2::new(
                                end_city.x + route.offset_x,
                                end_city.y + route.offset_y
                            );
                        }
                    }
                    self.routes = data.routes;
                    
                    // Восстанавливаем счётчики ID на основе максимальных ID + 1
                    self.next_city_id = self.cities.iter().map(|c| c.id).max().unwrap_or(0) + 1;
                    self.next_route_id = self.routes.iter().map(|r| r.id).max().unwrap_or(0) + 1;
                }
            }
        }
    }
}
