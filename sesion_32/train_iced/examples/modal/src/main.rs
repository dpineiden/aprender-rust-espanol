use iced::executor;
use iced::keyboard;
// si ocurre algo el gui monitorea los eventos
use iced::subscription::{self, Subscription};
use iced::theme;
use iced::widget::{
    self, button, column, container, horizontal_space, pick_list, row, text,
    text_input,
};

mod modal;
use modal::Modal;
use std::fmt;

use iced::{
	Alignment, 
	Application, 
	Command, 
	Element, 
	Event, 
	Length, 
	Settings
};


pub fn main() -> iced::Result {
	App::run(Settings::default())
}

#[derive(Default)]
struct App {
	show_modal: bool,
	email: String,
	password: String,
	plan: Plan // ??
}

#[derive(Debug, Clone)]
enum Message {
	ShowModal,
	HideModal,
	Email(String),
	Password(String),
	Plan(Plan),
	Submit,
	Event(Event)
}

impl Application for App {
	type Executor = executor::Default;
	type Message = Message;
	type Theme = iced::Theme;
	type Flags = ();


	fn new(_flags:())->(Self, Command<Message>){
		(App::default(), Command::none())
	}

	fn title(&self)->String {
		String::from("Modal - Iced")
	}

	fn subscription(&self)-> Subscription<Self::Message> {
		//subscription::events().map(|event |Message::Event(event))
		subscription::events().map(Message::Event)
	}

	fn update(&mut self, message:Message) -> Command<Message>{
		match message {
			Message::ShowModal => {
				self.show_modal = true;
				widget::focus_next()//??
			}
			Message::HideModal => {
				self.hide_modal();
				Command::none()
			}
			Message::Email(email)=>{
				self.email = email;
				Command::none()
			}
			Message::Password(password)=>{
				self.password = password;
				Command::none()
			}
			Message::Plan(plan)=>{
				self.plan = plan;
				Command::none()
			}
			Message::Submit => {
				if !self.email.is_empty() && !self.password.is_empty()
				{
					self.hide_modal()
				}
				Command::none()
			}
            Message::Event(event) => match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code: keyboard::KeyCode::Tab,
                    modifiers,
                }) => {
                    if modifiers.shift() {
                        widget::focus_previous()
                    } else {
                        widget::focus_next()
                    }
                }
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code: keyboard::KeyCode::Escape,
                    ..
                }) => {
                    self.hide_modal();
                    Command::none()
                }
                _ => Command::none(),
            },

			
		}
	}


    fn view(&self) -> Element<Message> {
        let content = container(
            column![
                row![
                    text("Top Left"),
                    horizontal_space(Length::Fill),
                    text("Top Right")
                ]
                .align_items(Alignment::Start)
                .height(Length::Fill),
                container(
                    button(text("Show Modal")).on_press(Message::ShowModal)
                )
                .center_x()
                .center_y()
                .width(Length::Fill)
                .height(Length::Fill),
                row![
                    text("Bottom Left"),
                    horizontal_space(Length::Fill),
                    text("Bottom Right")
                ]
                .align_items(Alignment::End)
                .height(Length::Fill),
            ]
            .height(Length::Fill),
        )
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fill);

        if self.show_modal {
            let modal = container(
                column![
                    text("Sign Up").size(24),
                    column![
                        column![
                            text("Email").size(12),
                            text_input("abc@123.com", &self.email,)
                                .on_input(Message::Email)
                                .on_submit(Message::Submit)
                                .padding(5),
                        ]
                        .spacing(5),
                        column![
                            text("Password").size(12),
                            text_input("", &self.password)
                                .on_input(Message::Password)
                                .on_submit(Message::Submit)
                                .password()
                                .padding(5),
                        ]
                        .spacing(5),
                        column![
                            text("Plan").size(12),
                            pick_list(
                                Plan::ALL,
                                Some(self.plan),
                                Message::Plan
                            )
                            .padding(5),
                        ]
                        .spacing(5),
                        button(text("Submit")).on_press(Message::HideModal),
                    ]
                    .spacing(10)
                ]
                .spacing(20),
            )
            .width(300)
            .padding(10)
            .style(theme::Container::Box);

            Modal::new(content, modal)
                .on_blur(Message::HideModal)
                .into()
        } else {
            content.into()
        }
    }

}


impl App {
    fn hide_modal(&mut self) {
        self.show_modal = false;
        self.email.clear();
        self.password.clear();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Plan {
    #[default]
    Basic,
    Pro,
    Enterprise,
}

impl Plan {
    pub const ALL: &[Self] = &[Self::Basic, Self::Pro, Self::Enterprise];
}

impl fmt::Display for Plan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Plan::Basic => "Basic",
            Plan::Pro => "Pro",
            Plan::Enterprise => "Enterprise",
        }
        .fmt(f)
    }
}
