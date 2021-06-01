use gtk::prelude::*;

pub fn build_all_frames(box_margin:i32)  -> gtk::Box {
    let bx = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .vexpand(false)
        .spacing(10)
        .margin_top(box_margin)
        .margin_end(box_margin)
        .margin_bottom(box_margin)
        .margin_start(box_margin)
        .build();

    let f1 = gtk::Frame::builder()
        .label("Frame 1")
        .width_request(300)
        .height_request(100)
        .valign(gtk::Align::Start)
        .build();
    bx.append(&f1);

    let f2 = gtk::Frame::builder()
        .label("Frame 1")
        .width_request(300)
        .height_request(100)
        .valign(gtk::Align::Start)
        .sensitive(false)
        .build();
    bx.append(&f2);

    return bx
}