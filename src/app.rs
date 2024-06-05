use crate::central;
use crate::Navbar;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Default)]
pub struct Website {
    // Navbar
    navbar: Navbar,
    // Central
    central: central::Panel,
}

impl Website {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        egui_extras::install_image_loaders(&cc.egui_ctx);

        cc.egui_ctx.style_mut(|style| {
            style.url_in_tooltip = true;
        });

        egui_thumbhash::register(&cc.egui_ctx);

        Default::default()
    }
}

impl eframe::App for Website {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.navbar.update(ctx, frame);
        self.central.update(ctx, frame, &mut self.navbar);
    }
}
