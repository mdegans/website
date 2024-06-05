use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};

// https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust
fn some_kind_of_uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// Generate a Rust file with a list of blog entries. These are all built into
/// the binary since we don't have many blog entries. They're loaded lazily,
/// however, so all the ui elements aren't created until the user scrolls down.
/// This is fine for now.
fn gen_blog_entries_rs() {
    const CRATE_ROOT: &str = env!("CARGO_MANIFEST_DIR");
    const BLOG_PATH: &str = "assets/blog";

    const PREFIX: &str = r#"
//! Blog entries.
use time::{Date, Month};

// A map of numeric months to their string representations.
const MONTHS: &[Month] = &[
    Month::January, // 0
    Month::January,
    Month::February,
    Month::March,
    Month::April,
    Month::May,
    Month::June,
    Month::July,
    Month::August,
    Month::September,
    Month::October,
    Month::November,
    Month::December,
];

/// A blog entry.
///     
/// # Fields
/// - `date`: The date the entry was published.
/// - `title`: The title of the entry.
/// - `content`: The content of the entry
#[derive(Debug)]
pub struct Entry {
    pub date: Date,
    pub title: &'static str,
    pub content: &'static str,
}

impl Entry {
    /// Create a new blog entry.
    pub const fn new(
        year: i32,
        month: u8,
        date:u8,
        title: &'static str,
        content: &'static str) -> Self {

        Self {
            date: match Date::from_calendar_date(year, MONTHS[month as usize], date) {
                Ok(date) => date,
                Err(_) => panic!("Invalid date"),
            },
            title,
            content,
        }
    }

    /// Entry UI.
    pub fn ui(
        &self,
        ui: &mut egui::Ui,
        commonmark_cache: &mut egui_commonmark::CommonMarkCache,
    ) -> egui::CollapsingResponse<egui::InnerResponse<()>> {
        let title = format!("{} - {}", self.date, self.title);

        egui::CollapsingHeader::new(title)
            .default_open(false)
            .show(ui, |ui| {
                egui_commonmark::CommonMarkViewer::new(self.title).show(
                    ui,
                    commonmark_cache,
                    self.content,
                )
            })
    }
}

pub const ENTRIES: &[&Entry] = &[
"#;
    let mut entries = String::from(PREFIX);

    let in_dir = Path::new(CRATE_ROOT).join(BLOG_PATH);
    let out_file = PathBuf::from(CRATE_ROOT)
        .join("src")
        .join("blog")
        .join("entries.rs");

    println!("cargo:rerun-if-changed={}/*", in_dir.display());
    let paths: BTreeSet<_> = in_dir
        .read_dir()
        .unwrap()
        .filter_map(Result::ok)
        .map(|e| e.path())
        .collect();

    for path in paths.iter().rev() {
        if path.extension().map_or(false, |ext| ext == "md") {
            let filename = path.file_name().unwrap().to_string_lossy();
            let mut fn_parts = filename.split('-');
            let year: u16 = fn_parts.next().unwrap().parse().unwrap();
            let month: u8 = fn_parts.next().unwrap().parse().unwrap();
            let day: u8 = fn_parts.next().unwrap().parse().unwrap();
            let title: String = fn_parts
                .map(some_kind_of_uppercase_first_letter)
                .collect::<Vec<_>>()
                .join(" ")
                .trim_end_matches(path.extension().unwrap().to_str().unwrap())
                .trim_end_matches('.')
                .to_string();
            let content = std::fs::read_to_string(&path).unwrap();

            entries.push_str(&format!(
                r##"    &Entry::new({}, {}, {}, r#"{}"#, r#"{}"#),"##,
                year, month, day, title, content,
            ));
        }
    }
    entries.push_str("];\n");
    std::fs::write(&out_file, entries)
        .expect(format!("Failed to write to {:?}", out_file).as_str());
    // rustfmt the file
    std::process::Command::new("rustfmt")
        .arg(&out_file)
        .output()
        .expect(format!("Failed to rustfmt {:?}", out_file).as_str());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    gen_blog_entries_rs();
    Ok(())
}
