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
    let vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(3)
        .margin_start(10)
        .margin_top(10)
        .margin_end(10)
        .margin_bottom(10)
        .halign(gtk4::Align::Center)
        .build();

    let current_volume = get_volume().to_string();
    println!("{}", current_volume);
    vbox.append(&create_slider(
        "audio-volume-high-symbolic".into(),
        &current_volume,
    ));

    vbox.append(&create_slider(
        "display-brightness-symbolic".into(),
        &current_volume,
    ));

    vbox.append(&create_toggle_button_and_dropdown(
        "Wifi".to_string(),
        "network-wireless-symbolic".into(),
    ));
    vbox.append(&create_toggle_button_and_dropdown(
        "Bluetooth".to_string(),
        "bluetooth-symbolic".into(),
    ));

    // Final Window Setup
    let window = ApplicationWindow::builder()
        .application(app)
        // .default_height(180)
        // .default_width(180)
        // .resizable(false)
        .title("ControlNix")
        .child(&vbox)
        .build();

    window.present();
}

// Functions for system controls as shown above...
fn set_volume(volume: u8) {
    use std::io::{self, Write};

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "pamixer -u; pamixer --allow-boost --set-volume {}; kill \"$(pidof sleep)\"",
            volume
        ))
        .output()
        .expect("set_volume: Failed to execute command");

    io::stderr().write_all(&output.stderr).unwrap();
}

fn get_wifi_networks() -> Vec<String> {
    println!("Called");
    // Run the `nmcli` command with additional shell utilities
    let output = Command::new("sh")
        .arg("-c")
        .arg("nmcli -t -f ACTIVE,SSID device wifi list | sed '/^$/d' | sort -u")
        .output()
        .expect("Failed to execute nmcli command");

    let output_str = String::from_utf8_lossy(&output.stdout);

    let mut connected_network = None;
    let mut wifi_networks = Vec::new();

    for line in output_str.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 {
            let is_active = parts[0] == "yes";
            let ssid = parts[1].to_string();

            if is_active {
                connected_network = Some(ssid);
            } else {
                wifi_networks.push(ssid);
            }
        }
    }

    // Prepend the connected network to the list if it exists
    if let Some(connected) = connected_network {
        wifi_networks.insert(0, connected);
    }

    wifi_networks
}

fn toggle_wifi(enable: bool) {
    let state = if enable { "on" } else { "off" };
    let _ = Command::new("sh")
        .arg("-c")
        .arg(format!("nmcli radio wifi {}", state))
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

fn get_volume() -> String {
    use std::io::{self, Write};

    let output = Command::new("sh")
        .arg("-c")
        .arg("pamixer --get-volume")
        .output()
        .expect("get_volume: Failed to execute command");

    let mut output_str = String::from_utf8(output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    // Trim any whitespace, including newline characters
    output_str = output_str.trim().to_string();
    output_str
}

fn mute_volume() {
    use std::io::{self, Write};
    let output = Command::new("sh")
        .arg("-c")
        .arg("pamixer -t; kill \"$(pidof sleep)\"")
        .output()
        .expect("get_volume: Failed to execute command");
    io::stderr().write_all(&output.stderr).unwrap();
}

fn create_toggle_button_and_dropdown(button_label: String, icon: String) -> gtk4::Box {
    let hbox = Box::new(Orientation::Horizontal, 2);
    let hbox1 = Box::new(Orientation::Horizontal, 2);
    let vbox1 = Box::new(Orientation::Vertical, 5);
    let button_icon = Image::from_icon_name(&icon);
    let label = Label::new(Some(&button_label));
    let network_name_label = Label::new(Some("Not connected"));

    vbox1.append(&label);
    vbox1.append(&network_name_label);
    hbox1.append(&button_icon);
    hbox1.append(&vbox1);

    let vbox1_clone = vbox1.clone();
    let button_icon_clone = button_icon.clone();
    let icon_name_clone = icon.clone();
    let button_label_clone = button_label.clone();

    // WiFi Toggle and Dropdown Split Button
    let toggle_button = ToggleButton::builder().build();
    toggle_button.connect_clicked(move |button| {
        if button.is_active() {
            if &button_label == "Wifi" {
                button_icon.set_icon_name(Some("network-wireless-offline-symbolic"));
            } else {
                button_icon.set_icon_name(Some("bluetooth-disabled-symbolic"));
            }
            vbox1_clone.hide();
        } else {
            button_icon.set_icon_name(Some(&icon));
            vbox1_clone.show();
        }
        toggle_wifi(true);
        toggle_bluetooth(true);
    });

    let toggle_menu_button = MenuButton::builder()
        .direction(gtk4::ArrowType::Right)
        .halign(gtk4::Align::Start)
        .build();
    
    let listbox = ListBox::builder().show_separators(true).build();
    let listbox_clone = listbox.clone();
    let popover = gtk4::Popover::builder().build();
    popover.connect_realize(move |_| {
        let networks: Vec<String>;
        if &button_label_clone == "Wifi" {
            networks = get_wifi_networks();
            //networks = vec!["Wifi1".into(), "Wifi2".into()];
        } else {
            networks = vec!["Bluetooth1".into(), "Bluetooth2".into()];
        }
        for network in networks {
            let row = ListBoxRow::builder().margin_top(5).margin_bottom(5).build();
            let label = Label::builder().label(network).build();
            row.set_child(Some(&label));
            listbox_clone.append(&row);
        }
    });

    let toggle_button_clone = toggle_button.clone();
    let popover_clone = popover.clone();
    let network_name_label_clone = network_name_label.clone();
    listbox.connect_row_activated(move |_, lbr| {
        if let Some(child) = lbr.child() {
            if let Ok(label) = child.downcast::<Label>() {
                toggle_button_clone.set_active(false);
                // Successfully downcasted, we can use label as a Label now
                network_name_label_clone.set_text(&label.text());
                button_icon_clone.set_icon_name(Some(&icon_name_clone));
                vbox1.show();
                popover_clone.popdown();
            }
        }
    });

    popover.set_child(Some(&listbox));
    toggle_menu_button.set_popover(Some(&popover));

    toggle_button.set_child(Some(&hbox1));

    hbox.append(&toggle_button);
    hbox.append(&toggle_menu_button);

    hbox
}

fn create_slider(icon_name: String, current_value: &String) -> gtk4::Box {
    let hbox = gtk4::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(2)
        .build();

    // Slider
    let slider = Scale::with_range(gtk4::Orientation::Horizontal, 0.0, 100.0, 5.0);
    slider.set_draw_value(true);
    slider.set_value_pos(PositionType::Right);
    slider.set_hexpand(true);

    println!("{}", current_value);
    let current_value: f64 = current_value.parse::<f64>().unwrap_or(60.0);
    slider.set_value(current_value);

    let icon_name_clone = icon_name.clone();
    slider.connect_value_changed(move |s| {
        let value = s.value() as u8;
        if icon_name_clone == "audio-volume-high-symbolic" {
            set_volume(value); // Call the function to adjust volume
        } else {
            set_brightness(value);
        }
    });

    if icon_name == "audio-volume-high-symbolic" {
        let toggle_button = ToggleButton::new();
        toggle_button.set_icon_name(&icon_name);
        let toggle_button_clone = toggle_button.clone();
        toggle_button.connect_clicked(move |b| {
            if b.is_active() {
                toggle_button_clone.set_icon_name("audio-volume-muted-symbolic");
            } else {
                toggle_button_clone.set_icon_name("audio-volume-high-symbolic");
            }
            mute_volume();
        });
        hbox.append(&toggle_button);
    } else {
        let button_icon = Image::from_icon_name(&icon_name);
        hbox.append(&button_icon);
    }

    hbox.append(&slider);

    hbox
}
