use gtk::prelude::*;

pub fn build_all_bars(box_margin:i32)  -> gtk::Box {
    let bx = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .vexpand(false)
        .spacing(10)
        .margin_top(box_margin)
        .margin_end(box_margin)
        .margin_bottom(box_margin)
        .margin_start(box_margin)
        .build();
    bx.append(&build_all_hbars(box_margin));
    bx.append(&build_all_vbars(box_margin));
    bx.append(&build_ui_spinners(box_margin));
    return bx
}


pub fn build_ui_spinners(box_margin:i32) -> gtk::Box {
    let bx = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .spacing(box_margin)
        .margin_top(box_margin)
        .margin_end(box_margin)
        .margin_bottom(box_margin)
        .margin_start(box_margin)
        .build();

    let s1 = gtk::Spinner::builder()
        .tooltip_text("Spinner 1")
        .spinning(true)
        .build();

    let s2 = gtk::Spinner::builder()
        .tooltip_text("Spinner 2: Disabled")
        .spinning(true)
        .sensitive(false)
        .build();

    bx.append(&s1);
    bx.append(&s2);
    return bx
}

pub fn build_all_vbars(box_margin:i32)  -> gtk::Box {
    let bx = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .vexpand(false)
        .build();
    for b in build_progress_bars(box_margin,gtk::Orientation::Vertical).iter(){
        bx.append(b)
    }
    for b in build_scales(box_margin,gtk::Orientation::Vertical).iter(){
        bx.append(b)
    }
    for b in build_levels(box_margin,gtk::Orientation::Vertical, gtk::LevelBarMode::Continuous).iter(){
        bx.append(b)
    }
    for b in build_levels(box_margin,gtk::Orientation::Vertical, gtk::LevelBarMode::Discrete).iter(){
        bx.append(b)
    }
    return bx
}

pub fn build_all_hbars(box_margin:i32)  -> gtk::Box {
    let bx = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .vexpand(false)
        .build();

    for b in build_progress_bars(box_margin,gtk::Orientation::Horizontal).iter(){
        bx.append(b)
    }
    for b in build_scales(box_margin,gtk::Orientation::Horizontal).iter(){
        bx.append(b)
    }
    for b in build_levels(box_margin,gtk::Orientation::Horizontal, gtk::LevelBarMode::Continuous).iter(){
        bx.append(b)
    }
    for b in build_levels(box_margin,gtk::Orientation::Horizontal, gtk::LevelBarMode::Discrete).iter(){
        bx.append(b)
    }
    return bx
}

fn build_progress_bars(box_margin:i32, orientation:gtk::Orientation) -> [gtk::ProgressBar; 2]{
    let pbs:[gtk::ProgressBar; 2] = [
        gtk::ProgressBar::builder()
            .orientation(orientation)
            .margin_top(box_margin)
            .margin_end(box_margin)
            .margin_bottom(box_margin)
            .margin_start(box_margin)
            .fraction(0.5)
            .tooltip_text("ProgressBar 1")
            .build(),
        gtk::ProgressBar::builder()
            .orientation(orientation)
            .inverted(true)
            .fraction(0.5)
            .tooltip_text("ProgressBar 2")
            .build()
    ];
    if orientation == gtk::Orientation::Vertical{
        pbs[0].set_height_request(250);
        pbs[1].set_height_request(250);
    }
    return pbs
}

fn build_scales(box_margin:i32, orientation:gtk::Orientation) -> [gtk::Scale; 2]{
    let pbs:[gtk::Scale; 2] = [
        gtk::Scale::with_range(orientation,1.0, 100.0, 1.0),
        gtk::Scale::with_range(orientation,1.0, 100.0, 1.0)
    ];
    pbs[1].set_inverted(true);
    for (i, b) in pbs.iter().enumerate(){
        b.set_margin_top(box_margin);
        b.set_margin_end(box_margin);
        b.set_margin_bottom(box_margin);
        b.set_margin_start(box_margin);
        b.set_tooltip_text(Some(&format!("ProgressBar {no}", no= i + 1)[..]));
        b.set_draw_value(false);
        b.set_value(50.0);
    }
    return pbs
}
fn build_levels(box_margin:i32, orientation:gtk::Orientation, mode:gtk::LevelBarMode) -> [gtk::LevelBar; 2]{
    let pbs:[gtk::LevelBar; 2] = [
        gtk::LevelBar::new(),
        gtk::LevelBar::new()
    ];
    pbs[1].set_inverted(true);
    for (i, b) in pbs.iter().enumerate(){
        b.set_orientation(orientation);
        b.set_mode(mode);
        b.set_margin_top(box_margin);
        b.set_margin_end(box_margin);
        b.set_margin_bottom(box_margin);
        b.set_margin_start(box_margin);
        b.set_value(3.0);
        b.set_max_value(10.0);
        b.set_min_value(1.0);
        b.set_tooltip_text(Some(&format!("LevelBar {no}", no= i + 1)[..]));
    }
    return pbs
}
