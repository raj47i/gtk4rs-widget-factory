use gtk::prelude::*;

pub fn build_all_labels(box_margin:i32)  -> gtk::Box {
    let bx = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .hexpand(true)
        .vexpand(true)
        .spacing(10)
        .margin_top(box_margin)
        .margin_end(box_margin)
        .margin_bottom(box_margin)
        .margin_start(box_margin)
        .build();
    let ls:[gtk::Label; 9] = [
        gtk::Label::new(Some("Large Title")),
        gtk::Label::new(None),
        gtk::Label::new(None),
        gtk::Label::new(None),
        gtk::Label::new(None),
        gtk::Label::new(Some("Heading")),
        gtk::Label::new(Some("Body")),
        gtk::Label::new(Some("Caption Heading")),
        gtk::Label::new(Some("Caption"))
    ];
    ls[0].style_context().add_class("large-title");
    for i in 1..5{
        ls[i].set_label(&format!("Title {n}", n=i)[..]);
        ls[i].style_context().add_class(&format!("title-{n}", n=i)[..]);
    }
    ls[5].style_context().add_class("heading");
    ls[6].style_context().add_class("body");
    ls[7].style_context().add_class("caption-heading");
    ls[8].style_context().add_class("caption");

    // ls[0].set_markup("<span style=\"large-title\">Large Title</span>");

    for (i,l) in ls.iter().enumerate(){
        bx.append(l);
    }
    return bx
}

pub fn build_all_textviews(box_margin:i32)  -> gtk::Box {
    let bx = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .hexpand(true)
        .vexpand(true)
        .spacing(10)
        .margin_top(box_margin)
        .margin_end(box_margin)
        .margin_bottom(box_margin)
        .margin_start(box_margin)
        .build();
    let buf1 = gtk::TextBuffer::builder()
        .text("Lorem ipsum dolor sit amet, consectetur adip	iscing elit. Nullam fringilla, est ut feugiat ultrices, elit lacus ultricies nibh, id commodo tortor nisi id elit. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Morbi vel elit erat. Maecenas dignissim, dui et pharetra rutrum, tellus lectus rutrum mi, a convallis libero nisi quis tellus. Nulla facilisi. Nullam eleifend lobortis nisl, in porttitor tellus malesuada vitae. Aenean lacus tellus, pellentesque quis molestie quis, fringilla in arcu. Duis elementum, tellus sed tristique semper, metus metus accumsan augue, et porttitor augue orci a libero. Ut sed justo ac felis placerat laoreet sed id sem. Proin mattis tincidunt odio vitae tristique. Morbi massa libero, congue vitae scelerisque vel, ultricies vel nisl. Vestibulum in tortor diam, quis aliquet quam. Praesent ut justo neque, tempus rutrum est. Duis eu lectus quam. Vivamus eget metus a mauris molestie venenatis pulvinar eleifend nisi. Nulla facilisi. Pellentesque at dolor sit amet purus dapibus pulvinar molestie quis neque.Suspendisse feugiat quam quis dolor accumsan cursus.")
        .build();
    let t1 = gtk::TextView::builder()
        .hexpand(true)
        .vexpand(true)
        .top_margin(box_margin)
        .right_margin(box_margin)
        .bottom_margin(box_margin)
        .left_margin(box_margin)
        .wrap_mode(gtk::WrapMode::Word)
        .buffer(&buf1)
        .tooltip_text("TextView 1")
        .build();
    bx.append(&t1);

    return bx
}