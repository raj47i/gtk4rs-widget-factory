use gtk::prelude::*;
use rand::Rng;

pub fn build_ui_combobox_texts(box_margin:i32)  -> gtk::Box {
    let bx = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .hexpand(false)
        .spacing(10)
        .margin_top(box_margin)
        .margin_end(box_margin)
        .margin_bottom(box_margin)
        .margin_start(box_margin)
        .build();

    let cbs:[gtk::ComboBoxText; 4] = [gtk::ComboBoxText::with_entry(), gtk::ComboBoxText::with_entry(), gtk::ComboBoxText::new(), gtk::ComboBoxText::new()];
    for (i, e) in cbs.iter().enumerate() {
        e.append_text("Option 1");
        e.append_text("Option 2");
        e.append_text("Option 3");
        e.append_text("Option 4");
        e.append_text("Option 5");
        e.set_active(Some(rand::thread_rng().gen_range(0..5)));
        e.set_sensitive(i % 2 == 1);
        bx.append(e);
    }

    return bx
}