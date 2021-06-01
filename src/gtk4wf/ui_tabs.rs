use gtk::prelude::*;

pub fn build_all_tabs(box_margin:i32)  -> gtk::Box {
    let bx = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .vexpand(false)
        .spacing(10)
        .margin_top(box_margin)
        .margin_end(box_margin)
        .margin_bottom(box_margin)
        .margin_start(box_margin)
        .build();
    let ns:[gtk::Notebook;4] = [
        gtk::Notebook::builder()
            .tab_pos(gtk::PositionType::Top)
            .show_border(true)
            .build(),
        gtk::Notebook::builder()
            .tab_pos(gtk::PositionType::Right)
            .build(),
        gtk::Notebook::builder()
            .tab_pos(gtk::PositionType::Bottom)
            .build(),
        gtk::Notebook::builder()
            .tab_pos(gtk::PositionType::Left)
            .build()
    ];
    for n in ns.iter(){
        n.set_margin_bottom(box_margin);
        for i in 1..6{
            n.append_page(&build_page_frame(box_margin, &format!("Page {no} Frame", no=i)[..]), Some(&gtk::Label::builder().label(&format!("Page {no}", no=i)[..]).build()));
        }
        bx.append(n);
    }

    return bx
}

fn build_page_frame(box_margin:i32, label:&str) -> gtk::Frame{
    let f = gtk::Frame::builder()
        .label(label)
        .margin_top(box_margin)
        .margin_end(box_margin)
        .margin_bottom(box_margin)
        .margin_start(box_margin)
        .height_request(200)
        .build();
    let content_text = format!("This text is in: {}!", label);
    let l= gtk::Label::builder()
        .label(&content_text[..])
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();
    f.set_child(Some(&l));
    return f
}