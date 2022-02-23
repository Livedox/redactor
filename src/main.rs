use std::env;
use std::error::Error;
use fltk::enums::{Color, Font, CallbackTrigger, Event};
use fltk::prelude::{WidgetExt, WindowExt, GroupExt, WidgetBase, DisplayExt};
use fltk::text::{ self };
use fltk::{window, app};
use futures::executor::block_on;
use tokio;

mod db;
use crate::db::DB;

mod cursor_to_files;
use crate::cursor_to_files::cursor_to_files;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client_uri = env::var("MONGODB_URI")
        .expect("You must set the MONGODB_URI environment var!");
    let db = DB::new(client_uri, "data", "test").await;

    let files = cursor_to_files(db.get_all().await).await;

    let app = app::App::default().with_scheme(app::Scheme::Base);
    app::background(211, 211, 211);
    let mut main_win = window::Window::default()
            .with_size(800, 600)
            .center_screen()
            .with_label("Editor");

    let mut buf = text::TextBuffer::default();
    buf.set_tab_distance(4);
    buf.set_text(&files[0].data);

    let mut editor = text::TextEditor::new(0, 0, 800, 600, "");
    editor.set_buffer(Some(buf));
    editor.set_scrollbar_size(15);
    editor.set_text_font(Font::Courier);
    editor.set_linenumber_width(32);
    editor.set_linenumber_fgcolor(Color::from_u32(0x008b_8386));
    editor.set_trigger(CallbackTrigger::Changed);
    editor.handle(move |edtr, ev| match ev {
        Event::Shortcut => {
            let data = edtr.buffer().unwrap().text();
            
            block_on(db.update_one(files[0].id.unwrap(), &data[..]));
            true
        }
        _ => {
            false
        }
    });
    
    main_win.end();
    main_win.show();
    app.run().unwrap();
    Ok(())
}
