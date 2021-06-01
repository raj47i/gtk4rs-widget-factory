use gtk::prelude::*;

pub fn build_all_buttons(box_margin:i32) -> gtk::Box {
    let bx = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(box_margin)
        .halign(gtk::Align::Baseline)
        .margin_top(box_margin)
        .margin_end(box_margin)
        .margin_bottom(box_margin)
        .margin_start(box_margin)
        .build();

    bx.append(&build_ui_buttons(box_margin));
    bx.append(&build_ui_toggle_buttons(box_margin));
    bx.append(&build_ui_font_buttons(box_margin));
    bx.append(&build_ui_spin_buttons(box_margin));

    let b = gtk::Box::new(gtk::Orientation::Horizontal, box_margin);
    b.set_halign(gtk::Align::Center);
    b.append(&build_ui_switches(box_margin, true));
    b.append(&build_ui_switches(box_margin, false));
    bx.append(&b);

    let b = gtk::Box::new(gtk::Orientation::Horizontal, box_margin);
    b.set_halign(gtk::Align::Center);
    b.append(&build_ui_color_buttons(box_margin));
    b.append(&build_ui_volume_buttons(box_margin));
    bx.append(&b);

    bx.append(&build_ui_labels(box_margin));

    bx.append(&build_ui_link_buttons(box_margin));
    return bx
}

pub fn build_ui_labels(box_margin:i32) -> gtk::Box {
    let bx = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .valign(gtk::Align::Center)
        .spacing(box_margin)
        .margin_top(box_margin)
        .margin_end(box_margin)
        .margin_bottom(box_margin)
        .margin_start(box_margin)
        .build();

    let b1 = gtk::Label::builder()
        .label("Label 1")
        .tooltip_text("Label 1")
        .build();

    let b2 = gtk::Label::builder()
        .label("Label 2")
        .tooltip_text("Label 2: Disabled")
        .sensitive(false)
        .build();

    bx.append(&b1);
    bx.append(&b2);
    return bx
}

pub fn build_all_check_radios(box_margin:i32) -> gtk::Box {
    let bx = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(box_margin)
        // .halign(gtk::Align::Center)
        .margin_top(box_margin)
        .margin_end(box_margin)
        .margin_bottom(box_margin)
        .margin_start(box_margin)
        .build();

    bx.append(&build_ui_check_buttons(box_margin));
    bx.append(&build_ui_radio_buttons(box_margin));
    return bx
}

fn build_ui_check_buttons(box_margin:i32) -> gtk::Box {
    let bx = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .valign(gtk::Align::Start)
        .spacing(box_margin)
        .margin_top(box_margin)
        .margin_end(box_margin)
        .margin_bottom(box_margin)
        .margin_start(box_margin)
        .build();
    let cbs: [gtk::CheckButton; 6] = [
        gtk::CheckButton::with_label("Checkbox 1"),
        gtk::CheckButton::with_label("Checkbox 2"),
        gtk::CheckButton::with_label("Checkbox 3"),
        gtk::CheckButton::with_label("Checkbox 4"),
        gtk::CheckButton::with_label("Checkbox 5"),
        gtk::CheckButton::with_label("Checkbox 6")
    ];

    for (i, e) in cbs.iter().enumerate() {
        let row = (i / 2) as i32;
        e.set_sensitive(i % 2 == 1);
        e.set_inconsistent(row == 1);
        e.set_active(row == 2);
        bx.append(e);
    }

    return bx
}

fn build_ui_radio_buttons(box_margin:i32) -> gtk::Box {
    let bx = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .valign(gtk::Align::Start)
        .spacing(box_margin)
        .margin_top(box_margin)
        .margin_end(box_margin)
        .margin_bottom(box_margin)
        .margin_start(box_margin)
        .build();
    let cbs: [gtk::CheckButton; 6] = [
        gtk::CheckButton::with_label("Radio 1"),
        gtk::CheckButton::with_label("Radio 2"),
        gtk::CheckButton::with_label("Radio 3"),
        gtk::CheckButton::with_label("Radio 4"),
        gtk::CheckButton::with_label("Radio 5"),
        gtk::CheckButton::with_label("Radio 6")
    ];

    for (i, e) in cbs.iter().enumerate() {
        let row = (i / 2) as i32;
        e.set_sensitive(i % 2 == 1);
        e.set_inconsistent(row == 1);
        e.set_active(row == 2);
        bx.append(e);
        if i > 0{
            e.set_group(Some(&cbs[i-1]));
        }
    }

    return bx
}

pub fn build_ui_spin_buttons(box_margin:i32) -> gtk::Box {
    let bx = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .valign(gtk::Align::Start)
        .spacing(box_margin)
        .margin_top(box_margin)
        .margin_end(box_margin)
        .margin_bottom(box_margin)
        .margin_start(box_margin)
        .build();

    let b:[gtk::SpinButton; 2] = [
        gtk::SpinButton::with_range(-100.00, 100.00, 1.00),
        gtk::SpinButton::with_range(-100.00, 100.00, 1.00)
    ];
    b[0].set_tooltip_text(Some("SpinButton 1"));
    b[0].set_value(1.00);
    b[1].set_value(1.00);
    b[1].set_tooltip_text(Some("SpinButton 2: Disabled"));
    b[1].set_sensitive(false);

    bx.append(&b[0]);
    bx.append(&b[1]);
    return bx
}

pub fn build_ui_buttons(box_margin:i32) -> gtk::Box {
    let bx = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(box_margin)
        .valign(gtk::Align::Start)
        .margin_top(box_margin)
        .margin_end(box_margin)
        .margin_bottom(box_margin)
        .margin_start(box_margin)
        .build();

    let b1 = gtk::Button::builder()
        .label("Button 1")
        .tooltip_text("Button 1")
        .build();

    let b2 = gtk::Button::builder()
        .label("Button 2")
        .sensitive(false)
        .tooltip_text("Button 2: Disabled")
        .build();
    bx.append(&b1);
    bx.append(&b2);
    return bx
}

pub fn build_ui_toggle_buttons(box_margin:i32) -> gtk::Box {
    let bx = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(box_margin)
        .valign(gtk::Align::Start)
        .margin_top(box_margin)
        .margin_end(box_margin)
        .margin_bottom(box_margin)
        .margin_start(box_margin)
        .build();

    let b1 = gtk::ToggleButton::builder()
        .active(true)
        .label("ToggleButton 1")
        .tooltip_text("ToggleButton 1")
        .build();

    let b2 = gtk::ToggleButton::builder()
        .active(true)
        .sensitive(false)
        .label("ToggleButton 2")
        .tooltip_text("ToggleButton 2: Active,Disabled")
        .build();
    bx.append(&b1);
    bx.append(&b2);
    return bx
}

pub fn build_ui_font_buttons(box_margin:i32) -> gtk::Box {
    let bx = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(box_margin)
        .valign(gtk::Align::Start)
        .margin_top(box_margin)
        .margin_end(box_margin)
        .margin_bottom(box_margin)
        .margin_start(box_margin)
        .build();

    let b1 = gtk::FontButton::builder()
        .tooltip_text("FontButton 1")
        .build();
    bx.append(&b1);

    let b1 = gtk::FontButton::builder()
        .sensitive(false)
        .tooltip_text("FontButton 2: Disabled")
        .build();
    bx.append(&b1);
    return bx
}

pub fn build_ui_color_buttons(box_margin:i32) -> gtk::Box {
    let bx = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .valign(gtk::Align::Center)
        .spacing(box_margin)
        .margin_top(box_margin)
        .margin_end(box_margin)
        .margin_bottom(box_margin)
        .margin_start(box_margin)
        .build();

    let color_red = gdk::RGBA{
        red: 255.0,
        green: 0.0,
        blue: 0.0,
        alpha: 100.0
    };

    let b1 = gtk::ColorButton::builder()
        .rgba(&color_red)
        .tooltip_text("ColorButton 1")
        .halign(gtk::Align::Center)
        .build();
    bx.append(&b1);

    let b2 = gtk::ColorButton::builder()
        .rgba(&color_red)
        .tooltip_text("ColorButton 2: Disabled")
        .halign(gtk::Align::Center)
        .sensitive(false)
        .build();

    bx.append(&b2);
    return bx
}

pub fn build_ui_switches(box_margin:i32, is_active:bool) -> gtk::Box {
    let bx = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .valign(gtk::Align::Center)
        .spacing(box_margin)
        .margin_top(box_margin)
        .margin_end(box_margin)
        .margin_bottom(box_margin)
        .margin_start(box_margin)
        .build();

    let ss:[gtk::Switch; 2] = [
        gtk::Switch::new(),
        gtk::Switch::new()
    ];
    ss[0].set_tooltip_text(Some("Switch"));
    if is_active{
        ss[1].set_tooltip_text(Some("Switch: On, Disabled"));
    }else{
        ss[1].set_tooltip_text(Some("Switch: Off, Disabled"));
    }
    for (i, e) in ss.iter().enumerate(){
        bx.append(e);
        e.set_halign(gtk::Align::Center);
        e.set_active(is_active);
        e.set_sensitive(i == 0);
    }
    return bx
}

pub fn build_ui_link_buttons(box_margin:i32) -> gtk::Box {
    let bx = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .valign(gtk::Align::Center)
        .spacing(box_margin)
        .margin_top(box_margin)
        .margin_end(box_margin)
        .margin_bottom(box_margin)
        .margin_start(box_margin)
        .build();

    let b1 = gtk::LinkButton::builder()
        .label("github.com")
        .uri("https://github.com/raj47i/gtk4rs-widget-factory")
        .tooltip_text("LinkButton 1")
        .build();

    let b2 = gtk::LinkButton::builder()
        .label("arunRaj.in")
        .sensitive(false)
        .uri("https://arunRaj.in")
        .tooltip_text("LinkButton 2: Disabled")
        .build();

    bx.append(&b1);
    bx.append(&b2);
    return bx
}

pub fn build_ui_volume_buttons(box_margin:i32) -> gtk::Box {
    let bx = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .valign(gtk::Align::Center)
        .spacing(box_margin)
        .margin_top(box_margin)
        .margin_end(box_margin)
        .margin_bottom(box_margin)
        .margin_start(box_margin)
        .build();

    let b1 = gtk::VolumeButton::builder()
        .tooltip_text("VolumeButton 1")
        .halign(gtk::Align::Center)
        .value(0.5)
        .build();

    let b2 = gtk::VolumeButton::builder()
        .tooltip_text("VolumeButton 2: Disabled")
        .halign(gtk::Align::Center)
        .sensitive(false)
        .value(0.0)
        .build();

    bx.append(&b1);
    bx.append(&b2);
    return bx
}
