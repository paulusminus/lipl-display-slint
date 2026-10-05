use crate::LiplDisplay;
use lipl_display_common::{Command, Message};
use login_poweroff_reboot_zbus::poweroff;
use slint::{Weak, invoke_from_event_loop, quit_event_loop};
use tracing::error;

pub(crate) async fn handle_message(ui_handle: Weak<LiplDisplay>, message: Message) {
    match message {
        Message::Part(part) => {
            if let Err(error) =
                invoke_from_event_loop(move || ui_handle.unwrap().set_part(part.into()))
            {
                error!("Error handling received part {}", error);
            };
        }
        Message::Status(status) => {
            if let Err(error) =
                invoke_from_event_loop(move || ui_handle.unwrap().set_status(status.into()))
            {
                error!("Error handling received status {}", error);
            };
        }
        Message::Command(command) => match command {
            Command::Dark => {
                if let Err(error) =
                    invoke_from_event_loop(move || ui_handle.unwrap().set_dark(true))
                {
                    error!("Error handling set theme dark {}", error);
                };
            }
            Command::Light => {
                if let Err(error) =
                    invoke_from_event_loop(move || ui_handle.unwrap().set_dark(false))
                {
                    error!("Error handling set theme light {}", error);
                };
            }
            Command::Increase => {
                if let Err(error) = invoke_from_event_loop(move || {
                    let ui = ui_handle.unwrap();
                    let length = ui.get_fontsize();
                    ui.set_fontsize(length + 2);
                }) {
                    error!("Failed to handle increase command {}", error);
                }
            }
            Command::Decrease => {
                if let Err(error) = invoke_from_event_loop(move || {
                    let ui = ui_handle.unwrap();
                    let length = ui.get_fontsize();
                    if length > 4 {
                        ui.set_fontsize(length - 2)
                    };
                }) {
                    error!("Failed to handle decrease command {error}");
                }
            }
            Command::Exit => {
                if let Err(error) = quit_event_loop() {
                    error!("Failed to handle exit command {error}");
                }
            }
            Command::Poweroff => {
                if let Err(error) = poweroff().await {
                    error!("Failed to send poweroff to systemd-logind: {error}");
                }

                if let Err(error) = quit_event_loop() {
                    error!("Failed to handle exit command {error}");
                }
            }
            Command::Wait => {
                if let Err(error) = invoke_from_event_loop(move || {
                    let screen = ui_handle.unwrap();
                    screen.set_status(lipl_display_common::WAIT_MESSAGE.into());
                    screen.set_part("".into());
                }) {
                    error!("Error handling received status {}", error);
                };
            }
        },
    }
}
