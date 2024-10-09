use std::time::Duration;

use cosmic::app::{Command, Core};
use cosmic::iced::wayland::popup::{destroy_popup, get_popup};
use cosmic::iced::window::Id;
use cosmic::iced::{time, Limits};
use cosmic::iced_style::application;
use cosmic::iced_widget::{row, Column};
use cosmic::widget::{self};
use cosmic::{Application, Element, Theme};

use crate::config::Config;
use crate::cpu::{get_cpu_percentage, get_cpus, Cpu};

#[derive(Default)]
pub struct App {
    core: Core,
    popup: Option<Id>,
    config: Config,
    cpus: Vec<Cpu>,
}

#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    PopupClosed(Id),
    UpdateCpu,
}

impl Application for App {
    type Executor = cosmic::executor::Default;

    type Flags = ();

    type Message = Message;

    const APP_ID: &'static str = "another.lusitano.AppletCpu";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn subscription(&self) -> cosmic::iced::Subscription<Self::Message> {
        let seconds = self.config.refresh_time;

        time::every(Duration::from_secs(seconds)).map(|_| Message::UpdateCpu)
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let app = App {
            core,
            cpus: get_cpus(),
            ..Default::default()
        };

        (app, Command::none())
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn view(&self) -> Element<Self::Message> {
        let percentage = get_cpu_percentage(&self.cpus);

        let percentage = format!("{}%", percentage);

        row![
            self.core.applet.text(percentage),
            self.core
                .applet
                .icon_button("utilities-system-monitor-symbolic")
                .on_press(Message::TogglePopup)
        ]
        .align_items(cosmic::iced::Alignment::Center)
        .into()
    }

    fn view_window(&self, _id: Id) -> Element<Self::Message> {
        let mut cpu_list: Vec<cosmic::iced_core::Element<'_, _, _, _>> = vec![];
        for cpu in &self.cpus {
            let percentage = format!("{}%", cpu.usage as u8);
            cpu_list.push(
                widget::row()
                    .push(
                        widget::text(&cpu.name)
                            .vertical_alignment(cosmic::iced::alignment::Vertical::Center)
                            .horizontal_alignment(cosmic::iced::alignment::Horizontal::Center)
                            .width(40),
                    )
                    .push(widget::progress_bar(0.0..=100.0, cpu.usage))
                    .push(
                        widget::text(percentage)
                            .vertical_alignment(cosmic::iced::alignment::Vertical::Center)
                            .horizontal_alignment(cosmic::iced::alignment::Horizontal::Center)
                            .width(40),
                    )
                    .into(),
            );
        }

        let content = Column::with_children(cpu_list);

        self.core.applet.popup_container(content).into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::TogglePopup => {
                return if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    let new_id = Id::unique();
                    self.popup.replace(new_id);
                    let mut popup_settings =
                        self.core
                            .applet
                            .get_popup_settings(Id::MAIN, new_id, None, None, None);
                    popup_settings.positioner.size_limits = Limits::NONE
                        .max_width(372.0)
                        .min_width(300.0)
                        .min_height(200.0)
                        .max_height(1080.0);
                    get_popup(popup_settings)
                }
            }
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
            Message::UpdateCpu => {
                self.cpus = get_cpus();
            }
        }
        Command::none()
    }

    fn style(&self) -> Option<<Theme as application::StyleSheet>::Style> {
        Some(cosmic::applet::style())
    }
}
