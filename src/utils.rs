use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use gtk::*;
use sourceview::*;
use std::path::PathBuf;

pub fn set_title(header_bar: &HeaderBar, path: &PathBuf, changed: bool) {
    if let Some(file_name) = path.file_name() {
        let file_name = &file_name.to_string_lossy();
        let title = if changed { format!("{} *", file_name) } else { file_name.to_string() };
        header_bar.set_title(Some(title.as_str()));

        if let Some(parent) = path.parent() {
            let subtitle: &str = &parent.to_string_lossy();
            header_bar.set_subtitle(Some(subtitle));
        }
    }
}

pub fn buffer_to_string(buffer: &Buffer) -> String {
    let (start, end) = buffer.get_bounds();
    buffer.get_text(&start, &end, false).unwrap().to_string()
}

pub fn open_file(filename: &PathBuf) -> String {
    let file = File::open(&filename).expect("Couldn't open file");

    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    let _ = reader.read_to_string(&mut contents);

    contents
}

pub fn save_file(filename: &PathBuf, text_buffer: &Buffer) {
    let contents = buffer_to_string(text_buffer);
    let mut file = File::create(filename).expect("Couldn't save file");
    file.write_all(contents.as_bytes()).expect("File save failed");
}

pub fn configure_sourceview(buff: &Buffer) {
    LanguageManager::new()
        .get_language("markdown")
        .map(|markdown| buff.set_language(Some(&markdown)));

    let manager = StyleSchemeManager::new();
    manager
        .get_scheme("classic")
        .map(|theme| buff.set_style_scheme(Some(&theme)));
    
    buff.set_max_undo_levels(100);
}

// http://gtk-rs.org/tuto/closures
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}
