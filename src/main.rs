use eframe::egui::{self, CentralPanel, Context, TextEdit};
use eframe::egui::{Button, Label};
use reqwest::blocking::Client;
use serde_json::json;
use std::process::Command;
use std::thread;
use std::time::Duration;
use sysinfo::{ProcessExt, System, SystemExt};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "NM Launcher",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

#[derive(Default)]
struct MyApp {
    username: String,
    password: String,
    status: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("NM Launcher");

            ui.add(TextEdit::singleline(&mut self.username).hint_text("Username"));
            ui.add(TextEdit::singleline(&mut self.password).hint_text("Password").password(true));

            if ui.button("Login & Launch").clicked() {
                if self.try_login() {
                    self.status = "✅ Login success. Waiting for Roblox...".to_string();

                    // Thread qui attend Roblox et injecte
                    thread::spawn(move || {
                        wait_for_roblox_and_inject();
                    });
                } else {
                    self.status = "❌ Invalid credentials.".to_string();
                }
            }

            ui.add_space(10.0);
            ui.label(&self.status);
        });
    }
}

impl MyApp {
    fn try_login(&self) -> bool {
        let client = Client::new();
        let res = client
            .post("https://nm-api-five.vercel.app/api/check")
            .json(&json!({
                "username": self.username.trim(),
                "password": self.password.trim()
            }))
            .send();

        match res {
            Ok(r) => {
                let json: serde_json::Value = r.json().unwrap_or_default();
                json.get("success")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false)
            }
            Err(_) => false,
        }
    }
}

fn wait_for_roblox_and_inject() {
    let mut sys = System::new_all();
    loop {
        sys.refresh_all();
        let found = sys
            .processes_by_name("RobloxPlayerBeta.exe")
            .next()
            .is_some();
        if found {
            // 🧠 Appelle ici ton injecteur/dll
            let _ = Command::new("injector.exe")
                .arg("RobloxPlayerBeta.exe")
                .arg("cheat.dll")
                .spawn();
            break;
        }
        thread::sleep(Duration::from_secs(1));
    }
}
