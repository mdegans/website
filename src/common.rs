//! Common content to multiple pages.

use egui_commonmark::CommonMarkCache;

/// Recent projects.
pub fn projects(ui: &mut egui::Ui, commonmark_cache: &mut CommonMarkCache) {
    egui::CollapsingHeader::new("Projects")
        .default_open(false)
        .show(ui, |ui| {
            projects_inner(ui, commonmark_cache);
        })
        .header_response
        .on_hover_text_at_pointer("My recent projects.");
}

/// Macro to generate a project section.
macro_rules! project {
    ($ui:ident, $commonmark_cache:ident, $f:ident, $title:expr, $markdown_file:expr, $desc:expr) => {
        fn $f(ui: &mut egui::Ui, commonmark_cache: &mut CommonMarkCache) {
            egui::CollapsingHeader::new($title)
                .default_open(false)
                .show(ui, |ui| {
                    egui_commonmark::commonmark_str!(
                        $markdown_file,
                        ui,
                        commonmark_cache,
                        $markdown_file
                    );
                })
                .header_response
                .on_hover_text_at_pointer($desc);
        }

        $f($ui, $commonmark_cache);
    };
    () => {};
}

/// Projects, but without the [`egui::CollapsingHeader`].
pub fn projects_inner(
    ui: &mut egui::Ui,
    commonmark_cache: &mut CommonMarkCache,
) {
    project!(
        ui,
        commonmark_cache,
        weave,
        "Weave",
        "assets/weave.md",
        "A tree-based story writing tool featuring generative AI."
    );
    project!(
        ui,
        commonmark_cache,
        website,
        "Drama Llama",
        "assets/drama_llama.md",
        "A safe and ergonomic Rust wrapper for `llama.cpp`."
    );
}
