use std::sync::mpsc;

use crate::{cli::TaggerCommand, tray_icon::TaggerTrayIcon};
use log::{debug, info};
use tray_icon::TaggerTrayEvent;
use winit::event_loop::EventLoopBuilder;

mod cli;
mod i18n;
mod tray_icon;

fn main() {
    env_logger::init();

    let args = cli::parse();

    match args.command {
        TaggerCommand::Edit { .. } => {
            debug!("{:?}", args);
        }
        // User can use this command to pop up a window allowing drag'n'droping
        // the files to edit. This command starts the tray icon, binds global
        // shortcuts then keep everything alive.
        TaggerCommand::Daemon {} => {
            let tray_icon = TaggerTrayIcon::new().unwrap();

            // Proper exit when there is a signal from the user to do so.
            let (tx, rx) = mpsc::channel();
            ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
                .expect("Error setting Ctrl-C handler");

            // Keep the application alive and wait for events.
            let event_loop = EventLoopBuilder::new().build().unwrap();
            event_loop
                .run(move |_event, event_loop| {
                    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

                    if let Ok(()) = rx.try_recv() {
                        event_loop.exit()
                    }

                    for event in tray_icon.events() {
                        match event {
                            TaggerTrayEvent::ClickOnTagFiles => {
                                info!("{}", i18n::fl!("tag-some-files"));
                            }
                            TaggerTrayEvent::DoubleClickOnTrayIcon => {
                                info!("{}", i18n::fl!("double-click"))
                            }
                            TaggerTrayEvent::Exit => {
                                event_loop.exit();
                            }
                        }
                    }
                })
                .unwrap();
        }
    };
}
