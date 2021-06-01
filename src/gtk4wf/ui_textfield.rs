use gtk::prelude::*;

pub fn build_ui_textfields(box_margin: i32) -> gtk::Box {
    let g = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .hexpand(false)
        .spacing(box_margin)
        .margin_top(box_margin)
        .margin_end(box_margin)
        .margin_bottom(box_margin)
        .margin_start(box_margin)
        .build();

    let es:[gtk::Entry; 5] = [gtk::Entry::new(), gtk::Entry::new(), gtk::Entry::new(), gtk::Entry::new(), gtk::Entry::new()];
    es[1].set_tooltip_text(Some("Read Only"));
    for (i,e) in es.iter().enumerate() {
        e.set_placeholder_text(Some("Enter some text.."));
        e.set_text("This is some text");
        e.set_editable(i != 1);

        if [2,4].contains(&i){
            e.set_sensitive(false);
            e.set_tooltip_text(Some("Disabled"));
        }

        if [3,4].contains(&i){
            e.set_text("Text Entry with icons");
            e.set_icon_from_icon_name(gtk::EntryIconPosition::Primary, Some("gtk-clear"));
            e.set_icon_from_icon_name(gtk::EntryIconPosition::Secondary, Some("gtk-find"));

            e.set_icon_tooltip_text(gtk::EntryIconPosition::Primary, Some("gtk-clear"));
            e.set_icon_tooltip_text(gtk::EntryIconPosition::Secondary, Some("gtk-find"));
        }

        g.append(e);
    }

    return g
}