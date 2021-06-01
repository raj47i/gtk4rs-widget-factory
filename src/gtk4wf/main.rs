use gtk::prelude::*;

pub fn build_all(box_margin:i32) -> gtk::Grid {
    return build_page_1(box_margin)
}

fn build_page_1(box_margin:i32) -> gtk::Grid {
    let bx = gtk::Grid::builder()
        .row_spacing(box_margin)
        .valign(gtk::Align::Baseline)
        .column_spacing(box_margin)
        .margin_top(box_margin)
        .build();
    // col 0
    bx.attach(&super::ui_button::build_all_buttons(box_margin),0,0,1,1);

    // col 1
    let bc1 = gtk::Box::new(gtk::Orientation::Vertical, box_margin);
    bc1.append(&super::ui_combobox::build_ui_combobox_texts(box_margin));
    bc1.append(&super::ui_textfield::build_ui_textfields(box_margin));
    bc1.append(&super::ui_button::build_all_check_radios(box_margin));
    bc1.append(&super::ui_frame::build_all_frames(box_margin));

    bx.attach(&bc1,1,0,1,1);

    // col 2
    bx.attach(&super::ui_tabs::build_all_tabs(box_margin),2,0,1,1);

    // col 3
    let bc1 = gtk::Box::new(gtk::Orientation::Horizontal, box_margin);
    bc1.append(&super::ui_progressbars::build_all_vbars(box_margin));
    let bc2 = gtk::Box::new(gtk::Orientation::Vertical, box_margin);
    bc2.append(&super::ui_progressbars::build_ui_spinners(box_margin));
    bc2.append(&super::ui_textview::build_all_labels(box_margin));
    bc1.append(&bc2);

    let bc3:gtk::Box  = gtk::Box::new(gtk::Orientation::Vertical, box_margin);
    bc3.append(&super::ui_progressbars::build_all_hbars(box_margin));
    bc3.append(&bc1);
    bc3.append(&super::ui_textview::build_all_textviews(box_margin));

    bx.attach(&bc3,3,0,1,1);
    // col 4

    return bx
}