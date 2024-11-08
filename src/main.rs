use gtk4::{
    glib, Application, ApplicationWindow, Box, Button, Image, Label, ListBox, ListBoxRow,
    MenuButton, Orientation, PasswordEntry, Scale,
};
use gtk4::{prelude::*, PositionType, ToggleButton};
use std::io::{self, Write};
use std::process::{Command, Output};

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

    let current_brightness = get_brightness();
    vbox.append(&create_slider(
        "display-brightness-symbolic".into(),
        &current_brightness,
    ));

    vbox.append(&create_toggle_button_and_dropdown(
        "Wifi".to_string(),
        "network-wireless-symbolic".into(),
        true,
    ));
    vbox.append(&create_toggle_button_and_dropdown(
        "Bluetooth".to_string(),
        "bluetooth-symbolic".into(),
        is_bluetooth_enabled(),
    ));

    // Final Window Setup
    let window = ApplicationWindow::builder()
        .application(app)
        .default_height(180)
        .default_width(180)
        .resizable(false)
        .title("ControlNix")
        .child(&vbox)
        .build();

    window.present();
}

fn create_toggle_button_and_dropdown(
    button_label: String,
    icon: String,
    enabled: bool,
) -> gtk4::Box {
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
    let button_label_clone = button_label.clone();
    let button_label_clone2 = button_label.clone();

    // WiFi Toggle and Dropdown Split Button
    let toggle_button = ToggleButton::builder().active(!enabled).build();
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
        enable_bluetooth(true);
    });

    let toggle_menu_button = MenuButton::builder()
        .direction(gtk4::ArrowType::Right)
        .halign(gtk4::Align::Start)
        .build();

    let listbox = ListBox::builder()
        .selection_mode(gtk4::SelectionMode::Single)
        .show_separators(true)
        .build();
    let listbox_clone = listbox.clone();
    let popover = gtk4::Popover::builder().build();
    popover.connect_realize(move |_| {
        let networks: Vec<String>;
        if &button_label_clone == "Wifi" {
            networks = get_wifi_networks();
            //networks = vec!["Wifi1".into(), "Wifi2".into()];
        } else {
            networks = get_bluetooth_networks();
        }
        for network in networks {
            let row = ListBoxRow::builder().margin_top(5).margin_bottom(5).build();
            if &button_label_clone == "Bluetooth" {
                let parts: Vec<&str> = network.split_whitespace().collect();
                if parts.len() >= 3 {
                    let vbox = Box::new(Orientation::Vertical, 2);
                    let label = Label::builder().label(parts[2]).build();
                    let mac = Label::builder().visible(false).label(parts[0]).build();
                    vbox.append(&label);
                    vbox.append(&mac);
                    row.set_child(Some(&vbox));
                }
            } else {
                let label = Label::builder().label(network).build();
                row.set_child(Some(&label));
            }
            listbox_clone.append(&row);
        }
    });

    let popover_clone = popover.clone();
    let network_name_label_clone = network_name_label.clone();

    listbox.connect_row_activated(move |_, lbr| {
        let mut selected_network: String = String::new();
        let mut selected_mac: String = String::new();
        if button_label_clone2 == "Wifi" {
            if let Some(child) = lbr.child() {
                if let Ok(l) = child.downcast::<Label>() {
                    println!("Here 1");
                    selected_network = l.text().to_string();
                }
            }
        } else {
            if let Some(child) = lbr.child() {
                if let Ok(b) = child.downcast::<Box>() {
                    if let Some(name_widget) = b.first_child() {
                        if let Ok(name) = name_widget.downcast::<Label>() {
                            selected_network = name.text().to_string();
                        }
                    }
                    if let Some(mac_widget) = b.first_child() {
                        if let Ok(mac) = mac_widget.downcast::<Label>() {
                            selected_mac = mac.text().to_string();
                            println!("{}", selected_mac);
                        }
                    }
                }
            }
        }

        if selected_network.is_empty() {
            return;
        }
        if button_label_clone2 == "Bluetooth" && selected_mac.is_empty() {
            return;
        }
        println!("{}", selected_network);

        let row_label = Label::new(Some(&selected_network));
        let vbox = Box::new(Orientation::Vertical, 2);
        let hbox = Box::new(Orientation::Horizontal, 2);
        let connect_button = Button::with_label("Connect");
        let forget_button = Button::with_label("Forget");
        let pass_entry = PasswordEntry::builder().show_peek_icon(true).build();
        pass_entry.set_visible(false);

        let selected_network_clone = selected_network.clone();
        let selected_network_clone2 = selected_network.clone();
        let network_label_name_clone2 = network_name_label_clone.clone();
        let vbox_clone = vbox.clone();
        let hbox_clone = hbox.clone();
        let popover_clone2 = popover_clone.clone();
        let pass_entry_clone = pass_entry.clone();
        let button_label_clone3 = button_label_clone2.clone();
        let button_label_clone4 = button_label_clone2.clone();
        let selected_mac_clone = selected_mac.clone();

        connect_button.connect_clicked(move |b| {
            if button_label_clone3 == "Wifi" {
                if connect_wifi(&selected_network_clone, None) {
                    // If connection succeeds without a password, set label and exit.
                    hbox_clone.remove(b);
                    popover_clone2.popdown();
                    network_label_name_clone2.set_text(&selected_network_clone);
                    return;
                }

                // If connection without password fails, show password entry.
                pass_entry_clone.set_visible(true);

                // Check if password is provided.
                let password = pass_entry_clone.text();
                if password.is_empty() {
                    pass_entry_clone.grab_focus();
                    return;
                }

                // Try connecting with the provided password.
                if connect_wifi(&selected_network_clone, Some(password.to_string())) {
                    vbox_clone.remove(&pass_entry_clone);
                    hbox_clone.remove(b);
                    popover_clone2.popdown();
                    network_label_name_clone2.set_text(&selected_network_clone);
                } else {
                    // Focus on the password field if connection with password fails.
                    pass_entry_clone.grab_focus();
                }
            } else {
                if connect_bluetooth(&selected_mac) {
                    //if connection is successfull
                    hbox_clone.remove(b);
                    popover_clone2.popdown();
                    network_label_name_clone2.set_text(&selected_network_clone);
                    return;
                } else {
                    network_label_name_clone2.set_text("Failed to connect");
                }
            }
        });

        forget_button.connect_clicked(move |_| {
            if button_label_clone4 == "Wifi" {
                forget_network(&button_label_clone4, &selected_network_clone2);
            } else {
                forget_network(&button_label_clone4, &selected_mac_clone);
            }
        });

        hbox.append(&forget_button);
        hbox.append(&connect_button);

        vbox.append(&row_label);
        vbox.append(&pass_entry);
        vbox.append(&hbox);
        lbr.set_child(Some(&vbox));
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
            toggle_audio();
        });
        hbox.append(&toggle_button);
    } else {
        let button_icon = Image::from_icon_name(&icon_name);
        hbox.append(&button_icon);
    }

    hbox.append(&slider);

    hbox
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

fn connect_wifi(wifi_name: &String, password: Option<String>) -> bool {
    println!("Inside connect_wif()");
    let output: Output;
    if let Some(pass) = password {
        println!("{pass}");
        output = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "nmcli dev wifi connect '{}' password '{}'",
                wifi_name, pass
            ))
            .output()
            .expect("connect_wifi: failed to execute command");
    } else {
        println!("no pass");
        output = Command::new("sh")
            .arg("-c")
            .arg(format!("nmcli dev wifi connect {}", wifi_name))
            .output()
            .expect("connect_wifi: failed to execute command");
    }
    io::stderr().write_all(&output.stderr).unwrap();
    io::stdout().write_all(&output.stdout).unwrap();
    output.status.success()
}

fn enable_bluetooth(enable: bool) -> bool {
    let state = if enable { "power on" } else { "power off" };
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("bluetoothctl {}", state))
        .output()
        .expect("toggle_bluetooth: failed to execute command");

    io::stderr().write_all(&output.stderr).unwrap();
    io::stdout().write_all(&output.stdout).unwrap();

    output.status.success()
}

fn is_bluetooth_enabled() -> bool {
    let output = Command::new("sh")
        .arg("-c")
        .arg("bluetoothctl show | grep PowerState | awk {'print $2'}")
        .output()
        .expect("toggle_bluetooth: failed to execute command");

    io::stderr().write_all(&output.stderr).unwrap();
    io::stdout().write_all(&output.stdout).unwrap();

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        return stdout.trim() == "on";
    }
    false
}

fn get_bluetooth_networks() -> Vec<String> {
    if !enable_bluetooth(true) {
        return Vec::new();
    }
    let output = Command::new("sh")
        .arg("-c")
        .arg("bluetoothctl scan on")
        .output()
        .expect("get_bluetooth_networks: failed to execute command");

    if !output.status.success() {
        return Vec::new();
    }

    println!("getting list of bluetooth networks");
    // Wait a few seconds to gather devices
    std::thread::sleep(std::time::Duration::from_secs(5));

    // Stop scanning
    Command::new("sh")
        .arg("-c")
        .arg("bluetoothctl scan off")
        .output()
        .expect("Failed to stop scanning");

    // Get the list of devices
    let output = Command::new("sh")
        .arg("-c")
        .arg("bluetoothctl devices")
        .output()
        .expect("Failed to list devices");

    io::stderr().write_all(&output.stderr).unwrap();
    io::stdout().write_all(&output.stdout).unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Collect each line as a "Device" entry in format "Address - Name"
    stdout
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 && parts[0] == "Device" {
                let address = parts[1];
                let name = parts[2..].join(" ");
                Some(format!("{} - {}", address, name))
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
}

fn connect_bluetooth(mac: &String) -> bool {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("bluetoothctl connect {}", mac))
        .output()
        .expect("Failed to connect");

    io::stderr().write_all(&output.stderr).unwrap();
    io::stdout().write_all(&output.stdout).unwrap();

    output.status.success()
}

fn forget_network(what: &String, name: &String) -> bool {
    let name_clone = name.clone();
    let mut command = Command::new("sh");
    command.arg("-c");

    if what == "Wifi" {
        command.arg(format!("nmcli connection delete '{}'", name));
    } else {
        command.arg(format!("bluetoothctl remove {}", name_clone));
    }
    let output = command
        .output()
        .expect("forget_network: error foregetting network");

    io::stderr().write_all(&output.stderr).unwrap();
    io::stdout().write_all(&output.stdout).unwrap();

    output.status.success()
}

fn set_brightness(level: u8) {
    let output = Command::new("sh")
        .arg("-c")
        .arg("xbacklight")
        .arg("-set")
        .arg(format!("{}%", level))
        .output()
        .expect("set-brightness: failed to execute command");

    io::stderr().write_all(&output.stderr).unwrap();
    io::stdout().write_all(&output.stdout).unwrap();
}

fn get_brightness() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("xbacklight -get")
        .output()
        .expect("get_brightness: failed to execute command");

    io::stderr().write_all(&output.stderr).unwrap();
    io::stdout().write_all(&output.stdout).unwrap();

    let output_str = String::from_utf8_lossy(&output.stdout);
    output_str.trim().to_string()
}

// Functions for system controls as shown above...
fn set_volume(volume: u8) {
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

fn get_volume() -> String {
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

fn toggle_audio() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("pamixer -t; kill \"$(pidof sleep)\"")
        .output()
        .expect("toggle_audio: Failed to execute command");

    io::stderr().write_all(&output.stderr).unwrap();
    io::stdout().write_all(&output.stdout).unwrap();
}
