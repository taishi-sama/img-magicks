use egui::{mutex::Mutex, CentralPanel, DragValue, FontId, ScrollArea, Slider};

use std::{ops::Deref, sync::Arc};

use self::algs::{img2br, RandomDeviation};

mod algs;

#[derive(Clone)]
pub struct ImgToBraille {
    pub image: Arc<Mutex<Option<(egui::TextureHandle, image::DynamicImage)>>>,
    pub setting1: u8,
    pub setting2: u8,
    pub output_width: u32,
    pub output_height: u32,
    pub result: String,
    pub result_len: usize,
    pub font_size: f32,
}
impl ImgToBraille {
    pub fn new(image: Arc<Mutex<Option<(egui::TextureHandle, image::DynamicImage)>>>) -> Self {
        Self {
            image,
            setting1: 0,
            setting2: 0,
            output_width: 100,
            output_height: 100,
            result: "".into(),
            result_len: 0,
            font_size: 15.0
        }
    }
}

impl eframe::App for ImgToBraille {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let _sliders = ui.horizontal_wrapped(|ui| {
                ui.label("Color threshold:");
                let sl1 = ui.add(Slider::new(&mut self.setting1, 0..=255));
                ui.label("Noise threshold:");
                let sl2 = ui.add(Slider::new(&mut self.setting2, 0..=(255 - self.setting1)));
                ui.label("Width:");
                let dw1 = ui.add(DragValue::new(&mut self.output_width).clamp_range(10..=2000));
                ui.label("Height:");
                let dw2 = ui.add(DragValue::new(&mut self.output_height).clamp_range(10..=2000));
                ui.label(format!("Length: {}", self.result_len));
                ui.label("Font size:");
                let _sl = ui.add(Slider::new(&mut self.font_size, 1.0..=25.0).clamp_to_range(false).step_by(0.5).drag_value_speed(0.5));
                if sl1.changed() || sl2.changed() || dw1.changed() || dw2.changed() {
                    let image = self.image.clone();
                    let image = image.lock();
                    if let Some(a) = &image.deref() {
                        if let Some(p) = RandomDeviation::new(self.setting1, self.setting2) {
                            self.result = img2br(
                                &a.1,
                                self.output_width,
                                self.output_height,
                                algs::GrayscaleToMono::RandomDeviation(p),
                            );
                            self.result_len = self.result.chars().count();
                        }
                    }
                }
            });

            let mut c: &str = &self.result;
            ScrollArea::both().show(ui, |ui| {
                let t = egui::TextEdit::multiline(&mut c)
                    .font(FontId::new( self.font_size.max(1.0), egui::FontFamily::Proportional));
                ui.add_sized(ui.available_size(), t);
            })
        });
    }
}
