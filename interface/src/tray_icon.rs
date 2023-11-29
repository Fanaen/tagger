use crate::i18n::fl;
use tray_icon::{
    menu::{Menu, MenuEvent, MenuEventReceiver, MenuId, MenuItem, Submenu},
    ClickType, Icon, TrayIcon, TrayIconBuilder, TrayIconEvent, TrayIconEventReceiver,
};

pub enum TaggerTrayEvent {
    ClickOnTagFiles,
    DoubleClickOnTrayIcon,
    Exit,
}

pub struct TaggerTrayIcon<'a> {
    // We keep the struct to keep it in the tray.
    #[allow(dead_code)]
    tray_icon: TrayIcon,

    // To recognise the events
    tag_files_item: MenuId,
    exit_item: MenuId,

    // To receive the events
    menu_channel: &'a MenuEventReceiver,
    tray_channel: &'a TrayIconEventReceiver,
}

impl<'a> TaggerTrayIcon<'a> {
    pub fn new() -> anyhow::Result<TaggerTrayIcon<'a>> {
        let tray_menu = Menu::new();

        // Buttons
        let tool_name = fl!("tray-tool-name");
        let tag_files = MenuItem::new(fl!("tray-tag-files"), true, None);
        let exit = MenuItem::new(fl!("tray-exit"), true, None);

        // Since macos does not allows item directly, we create a submenu.
        if cfg!(macos) {
            let tray_submenu = Submenu::new(&tool_name, true);

            tray_submenu.append(&tag_files)?;
            tray_submenu.append(&exit)?;

            tray_menu.append(&tray_submenu)?;
        } else {
            tray_menu.append(&tag_files)?;
            tray_menu.append(&exit)?;
        }

        Ok(TaggerTrayIcon {
            tray_icon: TrayIconBuilder::new()
                .with_menu(Box::new(tray_menu))
                .with_icon(load_logo()?)
                .with_tooltip(tool_name)
                .build()?,

            // Buttons
            tag_files_item: tag_files.into_id(),
            exit_item: exit.into_id(),

            // To receive the events
            menu_channel: MenuEvent::receiver(),
            tray_channel: TrayIconEvent::receiver(),
        })
    }

    pub fn try_recv_menu(&self) -> Option<TaggerTrayEvent> {
        match self.menu_channel.try_recv().ok().map(|event| event.id) {
            Some(id) if id == self.tag_files_item => Some(TaggerTrayEvent::ClickOnTagFiles),
            Some(id) if id == self.exit_item => Some(TaggerTrayEvent::Exit),
            _ => None,
        }
    }

    pub fn try_recv_tray_icon(&self) -> Option<TaggerTrayEvent> {
        match self.tray_channel.try_recv().ok() {
            Some(TrayIconEvent {
                click_type: ClickType::Double,
                ..
            }) => Some(TaggerTrayEvent::DoubleClickOnTrayIcon),
            _ => None,
        }
    }

    /// A way to hide tray icon useless events and format useful ones.
    pub fn events(&'a self) -> EventIterator<'a> {
        EventIterator {
            tray_icon: self,
            menu_events_exhausted: false,
        }
    }
}

/// A way to hide tray icon useless events and format useful ones.
pub struct EventIterator<'a> {
    tray_icon: &'a TaggerTrayIcon<'a>,
    menu_events_exhausted: bool,
}

impl<'a> Iterator for EventIterator<'a> {
    type Item = TaggerTrayEvent;

    fn next(&mut self) -> Option<TaggerTrayEvent> {
        if !self.menu_events_exhausted {
            match self.tray_icon.try_recv_menu() {
                Some(event) => return Some(event),
                None => self.menu_events_exhausted = true,
            }
        }

        self.tray_icon.try_recv_tray_icon()
    }
}

/// Build the tray icon with from the logo embedded in the binary
fn load_logo() -> anyhow::Result<Icon> {
    let buffer = include_bytes!("../../assets/logo-32.png");
    let logo = image::load_from_memory(buffer)?;

    let width = logo.width();
    let height = logo.height();

    Ok(Icon::from_rgba(logo.into_bytes(), width, height)?)
}
