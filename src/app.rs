use std::{ops::Deref, sync::Arc};

use crate::state::AppState;
use egui::{mutex::Mutex, Context, FontData, FontDefinitions, FontFamily, Vec2, Visuals};

//#[derive(serde::Deserialize, serde::Serialize)]
//#[serde(default)]
pub struct ImgMagicksApp {
    app: AppState,
    //#[serde(skip)]
    current_image: Arc<Mutex<Option<(egui::TextureHandle, image::DynamicImage)>>>,
}

impl Default for ImgMagicksApp {
    fn default() -> Self {
        Self {
            app: AppState::None,
            // Example stuff:
            current_image: Arc::new(Mutex::new(None)),
        }
    }
}

fn setup_custom_font(ctx: &Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "NotoSansSymbols2".to_owned(),
        FontData::from_static(include_bytes!("../fonts/NotoSansSymbols2-Regular.ttf")),
    );
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .push("NotoSansSymbols2".to_owned());
    ctx.set_fonts(fonts)
}

impl ImgMagicksApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        setup_custom_font(&_cc.egui_ctx);
        _cc.egui_ctx.set_visuals(Visuals::dark());
        Default::default()
    }
}
impl ImgMagicksApp {
    fn handle_file(&mut self, ctx: &egui::Context) {
        let fd = rfd::FileDialog::new().add_filter("Images", &["jpg", "jpeg", "png", "gif"]); //TODO: extend list by adding all supported formats
        if let Some(path) = fd.pick_file() {
            let image = image::io::Reader::open(path).unwrap().decode().unwrap();
            let size = [image.width() as _, image.height() as _];
            let image_buffer = image.to_rgba8();
            let pixels = image_buffer.as_flat_samples();
            let loaded = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
            *self.current_image.lock() =
                Some((ctx.load_texture("image", loaded, Default::default()), image));
        }
    }
}

impl eframe::App for ImgMagicksApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.separator();
                if ui.button("Open image file...").clicked() {
                    self.handle_file(ctx);
                }
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Your image:");
            if let Some(txt) = self.current_image.lock().deref() {
                let w = ui.available_width();
                let h = w / txt.0.aspect_ratio();
                ui.image(&txt.0, Vec2::new(w, h));
            }
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("font used ");
                    ui.hyperlink_to("\"Noto Sans Symbols2\"", "https://notofonts.github.io/");
                    ui.label(".");
                });
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("source codes are available on ");
                    ui.hyperlink_to("GitHub", "https://github.com/taishi-sama/img-magicks");
                    ui.label(".");
                })
            });
        });
        egui::TopBottomPanel::top("indexator").show(ctx, |ui| {
            if ui.button("Open Image2Braille converter").clicked() {
                let image = self.current_image.clone();
                self.app = AppState::ImgToBraille(crate::img2br::ImgToBraille::new(image))
            }
        });
        egui::CentralPanel::default().show(ctx, |_ui| {
            match &mut self.app {
                AppState::None => {}
                AppState::ImgToBraille(i) => i.update(ctx, _frame),
            };
        });
    }
}
