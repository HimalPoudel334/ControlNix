use gtk4::{
    glib, Application, ApplicationWindow, Box, Image, Label, ListBox, ListBoxRow, MenuButton,
    Orientation, Scale,
};
use gtk4::{prelude::*, PositionType, ToggleButton};
use std::process::Command;

fn main() -> glib::ExitCode {
    let app = Application::new(Some("com.example.ControlNix"), Default::default());

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let vbox = Box::new(Orientation::Vertical, 5);

    // Volume Slider
    let volume_slider = Scale::with_range(gtk4::Orientation::Horizontal, 0.0, 100.0, 5.0);
    volume_slider.set_draw_value(true);
    volume_slider.set_value_pos(PositionType::Right);
    volume_slider.connect_value_changed(|s| {
        let volume = s.value() as u8;
        set_volume(volume); // Call the function to adjust volume
    });
    vbox.append(&volume_slider);

    let networks: Vec<String> = vec!["Network 1".into(), "Network 2".into(), "Network 3".into()];
    let networks_clone = networks.clone();

    vbox.append(&create_toggle_button_and_dropdown(
        "Wifi".to_string(),
        "network-wireless-symbolic".into(),
        networks,
    ));
    vbox.append(&create_toggle_button_and_dropdown(
        "Bluetooth".to_string(),
        "bluetooth-symbolic".into(),
        networks_clone,
    ));

    // Brightness Slider
    let brightness_slider = Scale::with_range(gtk4::Orientation::Horizontal, 0.0, 100.0, 5.0);
    brightness_slider.set_draw_value(true);
    brightness_slider.set_value_pos(PositionType::Right);
    brightness_slider.connect_value_changed(|s| {
        let brightness = s.value() as u8;
        set_brightness(brightness); // Call the function to adjust volume
    });
    vbox.append(&brightness_slider);

    // Final Window Setup
    let window = ApplicationWindow::builder()
        .application(app)
        .title("ControlNix")
        .child(&vbox)
        .build();

    window.present();
}

// Functions for system controls as shown above...
fn set_volume(volume: u8) {
    let _ = Command::new("pamixer")
        .arg("-u")
        .arg(";")
        .arg("pamixer")
        .arg("--allow-boost")
        .arg("-i")
        .arg(format!("{}%", volume))
        .arg(";")
        .arg("killsleep")
        .output();
}

fn toggle_wifi(enable: bool) {
    let state = if enable { "on" } else { "off" };
    let _ = Command::new("nmcli")
        .arg("radio")
        .arg("wifi")
        .arg(state)
        .output();
}

fn toggle_bluetooth(enable: bool) {
    let state = if enable { "power on" } else { "power off" };
    let _ = Command::new("bluetoothctl").arg(state).output();
}

fn set_brightness(level: u8) {
    let _ = Command::new("brightnessctl")
        .arg("set")
        .arg(format!("{}%", level))
        .output();
}

fn create_toggle_button_and_dropdown(
    button_label: String,
    icon: String,
    networks_list: Vec<String>,
) -> gtk4::Box {
    let hbox = Box::new(Orientation::Horizontal, 2);
    let hbox1 = Box::new(Orientation::Horizontal, 2);
    let vbox1 = Box::new(Orientation::Vertical, 5);
    let icon = Image::from_icon_name(&icon);
    let label = Label::new(Some(&button_label));
    let hbox1_copy = hbox1.clone();
    let network_name_label = Label::new(Some(networks_list.first().unwrap()));
    // WiFi Toggle and Dropdown Split Button
    let toggle_button = ToggleButton::builder().build();
    toggle_button.connect_clicked(move |button| {
        if button.is_active() {
            button.set_label(&format!("{button_label}: Disconnected"));
        } else {
            button.set_child(Some(&hbox1_copy));
        }
        toggle_wifi(true);
        toggle_bluetooth(true);
    });

    let toggle_menu_button = MenuButton::builder()
        .direction(gtk4::ArrowType::Right)
        .halign(gtk4::Align::Start)
        .build();

    //let networks = vec!["Network 1", "Network 2", "Network 3"];
    let popover = gtk4::Popover::builder().build();
    let listbox = ListBox::builder().show_separators(true).build();

    for network in networks_list {
        let row = ListBoxRow::builder().margin_top(5).margin_bottom(5).build();
        let label = Label::builder().label(&network).build();
        row.set_child(Some(&label));
        listbox.append(&row);
    }

    let toggle_button_clone = toggle_button.clone();
    let popover_clone = popover.clone();
    let network_name_label_clone = network_name_label.clone();
    listbox.connect_row_activated(move |_, lbr| {
        if let Some(child) = lbr.child() {
            if let Ok(label) = child.downcast::<Label>() {
                toggle_button_clone.set_active(false);
                // Successfully downcasted, we can use label as a Label now
                network_name_label_clone.set_text(&label.text());
                popover_clone.popdown();
            }
        }
    });

    popover.set_child(Some(&listbox));
    toggle_menu_button.set_popover(Some(&popover));

    vbox1.append(&label);
    vbox1.append(&network_name_label);
    hbox1.append(&icon);
    hbox1.append(&vbox1);

    toggle_button.set_child(Some(&hbox1));

    hbox.append(&toggle_button);
    hbox.append(&toggle_menu_button);

    hbox
}
