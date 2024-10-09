use app::App;
mod app;
mod config;
mod cpu;

fn main() -> cosmic::iced::Result {
    cosmic::applet::run::<App>(true, ())
}
