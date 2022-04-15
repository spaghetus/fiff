use eframe::{
	egui::{
		self,
		plot::{Plot, PlotImage, Value},
	},
	epaint::{ColorImage, TextureHandle, TextureId, Vec2},
	epi::{self, Storage},
};
use fiff::FromFiff;
use image::RgbaImage;

fn main() {
	let image = std::fs::read(
		std::env::args()
			.skip(1)
			.next()
			.expect("Missing target file"),
	)
	.expect("Failed to read target file");
	let image = fiff::Image::from_fiff(&mut std::io::Cursor::new(&image)).unwrap();
	let a = Box::new(App(image, None));
	eframe::run_native(a, Default::default())
}

pub struct App(fiff::Image, Option<TextureHandle>);

impl epi::App for App {
	fn name(&self) -> &str {
		"fiffview"
	}
	fn setup(
		&mut self,
		ctx: &eframe::egui::Context,
		frame: &epi::Frame,
		_storage: Option<&dyn epi::Storage>,
	) {
		let i: RgbaImage = self.0.clone().into();
		let i: ColorImage =
			ColorImage::from_rgba_unmultiplied([self.0.width as usize, self.0.height as usize], &i);
		self.1 = Some(ctx.load_texture("shown", i));
	}
	fn update(&mut self, ctx: &eframe::egui::Context, frame: &epi::Frame) {
		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.label("fiffview");
				ui.separator();
				ui.label(format!("{} blocks", self.0.blocks.len()));
				ui.separator();
				ui.label(format!("{}x{}", self.0.width, self.0.height));
			})
		});
		egui::CentralPanel::default().show(ctx, |ui| {
			Plot::new("image_plot").data_aspect(1.0).show(ui, |ui| {
				if self.1.is_some() {
					ui.image(PlotImage::new(
						self.1.as_ref().unwrap(),
						Value { x: 0.0, y: 0.0 },
						Vec2::new(self.0.width as f32, self.0.height as f32),
					))
				}
			});
		});
	}
}
