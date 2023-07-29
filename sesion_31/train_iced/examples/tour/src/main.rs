use iced::alignment;
use iced::theme;
use iced::widget::{
    checkbox, column, container, horizontal_space, image, radio, row,
    scrollable, slider, text, text_input, toggler, vertical_space,
};
use iced::widget::{
	Button, Column, Container, Slider
};

use iced::{
	Color, 
	Element, 
	Font, 
	Length, 
	Renderer, 
	Sandbox, 
	Settings};



pub fn main() -> iced::Result  {
	env_logger::init();
	Tour::run(Settings::default())
}

pub struct Tour {
	steps: Steps,
	debug: bool
}

impl Sandbox for Tour {
	type Message = Message;

	fn new() -> Tour {
		Tour {
			steps: Steps::new(),
			debug: false
		}
	}

	fn title(&self)->String {
		format!("{} - Iced", self.steps.title())
		//String::from("Tour Iced")
	}

	fn update(&mut self, event: Message) {
		match event {
			Message::BackPressed =>{
				self.steps.go_back();
			}
			Message::NextPressed =>{
				self.steps.advance();
			}
			Message::StepMessage(step_msg) => {
				self.steps.update(step_msg, &mut self.debug);
			}
		}
	}

	fn view(&self)->Element<Message>{
		let Tour {steps, .. } = self;

		let mut controls = row![];
		
		if steps.has_previous() {
			controls = controls.push(
				button("Back")
					.on_press(Message::BackPressed)
					.style(theme::Button::Secondary)
			);
		}

		controls = controls.push(horizontal_space(Length::Fill));


		if steps.can_continue() {
			controls = controls.push(button("Next").on_press(Message::NextPressed));
		}

		let content: Element<_> = column![
			steps.view(self.debug).map(Message::StepMessage),
			controls
		]
			.max_width(540)
			.spacing(20)
			.padding(20)
			.into();

		let scrollable = scrollable(
			container(
				if self.debug {
					content.explain(Color::BLACK)
				} else{
					content
				}
			)
				.width(Length::Fill)
				.center_x()
		);

		container(scrollable).height(Length::Fill).center_y().into()
	}

}

#[derive(Debug, Clone)]
pub enum Message {
    BackPressed,
    NextPressed,
    StepMessage(StepMessage),
}



struct Steps {
	steps: Vec<Step>,
	current: usize
}

impl Steps {
	fn new()-> Steps {
		Steps {
			steps: vec![
				Step::Welcome,
				Step::Slider {value:50},
				Step::RowsAndColumns {
					layout: Layout::Row,
					spacing: 20
				},
                Step::Text {
                    size: 30,
                    color: Color::BLACK,
                },
			],
			current: 0
		}
	}

	fn update(&mut self, msg: StepMessage, debug: &mut bool){
		self.steps[self.current].update(msg, debug);
	}

	fn view(&self, debug:bool)-> Element<StepMessage> {
		self.steps[self.current].view(debug)
	}

	fn advance(&mut self) {
		if self.can_continue() {
			self.current += 1; 
		}
	}


    fn go_back(&mut self) {
        if self.has_previous() {
            self.current -= 1;
        }
    }

    fn has_previous(&self) -> bool {
        self.current > 0
    }

    fn can_continue(&self) -> bool {
        self.current + 1 < self.steps.len()
            && self.steps[self.current].can_continue()
    }

    fn title(&self) -> &str {
        self.steps[self.current].title()
    }	

}


enum Step {
	Welcome,
    Slider {
        value: u8,
    },
	RowsAndColumns {
		layout: Layout,
		spacing: u16
	},
    Text {
        size: u16,
        color: Color,
    },
}


#[derive(Debug, Clone)]
pub enum StepMessage {
    SliderChanged(u8),
    LayoutChanged(Layout),
    SpacingChanged(u16),
    TextSizeChanged(u16),
    TextColorChanged(Color),
    //LanguageSelected(Language),
    ImageWidthChanged(u16),
    InputChanged(String),
    ToggleSecureInput(bool),
    ToggleTextInputIcon(bool),
    DebugToggled(bool),
    TogglerChanged(bool),
}


impl<'a> Step {
    fn update(&mut self, msg: StepMessage, debug: &mut bool) {
        match msg {
            StepMessage::DebugToggled(value) => {
                // if let Step::Debugger = self {
                //     *debug = value;
                // }
            }
            //StepMessage::LanguageSelected(language) => {
                // if let Step::Radio { selection } = self {
                //     *selection = Some(language);
                // }
            //}
            StepMessage::SliderChanged(new_value) => {
                if let Step::Slider { value, .. } = self {
                    *value = new_value;
                }
            }
            StepMessage::TextSizeChanged(new_size) => {
                if let Step::Text { size, .. } = self {
                    *size = new_size;
                }
            }
            StepMessage::TextColorChanged(new_color) => {
                if let Step::Text { color, .. } = self {
                    *color = new_color;
                }
            }
            StepMessage::LayoutChanged(new_layout) => {
                if let Step::RowsAndColumns { layout, .. } = self {
                    *layout = new_layout;
                }
            }
            StepMessage::SpacingChanged(new_spacing) => {
                if let Step::RowsAndColumns { spacing, .. } = self {
                    *spacing = new_spacing;
                }
            }
            StepMessage::ImageWidthChanged(new_width) => {
                // if let Step::Image { width, .. } = self {
                //     *width = new_width;
                // }
            }
            StepMessage::InputChanged(new_value) => {
                // if let Step::TextInput { value, .. } = self {
                //     *value = new_value;
                // }
            }
            StepMessage::ToggleSecureInput(toggle) => {
                // if let Step::TextInput { is_secure, .. } = self {
                //     *is_secure = toggle;
                // }
            }
            StepMessage::TogglerChanged(value) => {
                // if let Step::Toggler { can_continue, .. } = self {
                //     *can_continue = value;
                // }
            }
            StepMessage::ToggleTextInputIcon(toggle) => {
                // if let Step::TextInput {
                //     is_showing_icon, ..
                // } = self
                // {
                //     *is_showing_icon = toggle
                // }
            }
        };
    }

    fn title(&self) -> &str {
        match self {
            Step::Welcome => "Welcome",
            Step::Slider { .. } => "Slider",
            Step::RowsAndColumns { .. } => "Rows and columns",
            Step::Text { .. } => "Text",

            // Step::Radio { .. } => "Radio button",
            // Step::Toggler { .. } => "Toggler",
            // Step::Image { .. } => "Image",
            // Step::Scrollable => "Scrollable",
            // Step::TextInput { .. } => "Text input",
            // Step::Debugger => "Debugger",
            // Step::End => "End",
        }
    }
    fn can_continue(&self) -> bool {
        match self {
            Step::Welcome => true,
            Step::Slider { .. } => true,
            Step::RowsAndColumns { .. } => true,
            Step::Text { .. } => true,
            // Step::Radio { selection } => *selection == Some(Language::Rust),
            // Step::Toggler { can_continue } => *can_continue,
            // Step::Image { .. } => true,
            // Step::Scrollable => true,
            // Step::TextInput { value, .. } => !value.is_empty(),
            // Step::Debugger => true,
            // Step::End => false,
        }
    }

    fn view(&self, debug: bool) -> Element<StepMessage> {
        match self {
            Step::Welcome => Self::welcome(),
            Step::Slider { value } => Self::slider(*value),
            Step::RowsAndColumns { layout, spacing } => {
                Self::rows_and_columns(*layout, *spacing)
            }
            Step::Text { size, color } => Self::text(*size, *color),
            // Step::Radio { selection } => Self::radio(*selection),
            // Step::Toggler { can_continue } => Self::toggler(*can_continue),
            // Step::Image { width } => Self::image(*width),
            // Step::Scrollable => Self::scrollable(),
            // Step::TextInput {
            //     value,
            //     is_secure,
            //     is_showing_icon,
            // } => Self::text_input(value, *is_secure, *is_showing_icon),
            // Step::Debugger => Self::debugger(debug),
            // Step::End => Self::end(),
        }
        .into()
    }

    fn container(title: &str) -> Column<'a, StepMessage> {
        column![text(title).size(50)].spacing(20)
    }	

    fn welcome() -> Column<'a, StepMessage> {
        Self::container("Welcome!")
            .push(
                "This is a simple tour meant to showcase a bunch of widgets \
                 that can be easily implemented on top of Iced.",
            )
            .push(
                "Iced is a cross-platform GUI library for Rust focused on \
                 simplicity and type-safety. It is heavily inspired by Elm.",
            )
            .push(
                "It was originally born as part of Coffee, an opinionated \
                 2D game engine for Rust.",
            )
            .push(
                "On native platforms, Iced provides by default a renderer \
                 built on top of wgpu, a graphics library supporting Vulkan, \
                 Metal, DX11, and DX12.",
            )
            .push(
                "Additionally, this tour can also run on WebAssembly thanks \
                 to dodrio, an experimental VDOM library for Rust.",
            )
            .push(
                "You will need to interact with the UI in order to reach the \
                 end!",
            )
    }


    fn slider(value: u8) -> Column<'a, StepMessage> {
        Self::container("Slider")
            .push(
                "A slider allows you to smoothly select a value from a range \
                 of values.",
            )
            .push(
                "The following slider lets you choose an integer from \
                 0 to 100:",
            )
            .push(slider(0..=100, value, StepMessage::SliderChanged))
            .push(
                text(value.to_string())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center),
            )
    }

    fn rows_and_columns(
        layout: Layout,
        spacing: u16,
    ) -> Column<'a, StepMessage> {
        let row_radio =
            radio("Row", Layout::Row, Some(layout), StepMessage::LayoutChanged);

        let column_radio = radio(
            "Column",
            Layout::Column,
            Some(layout),
            StepMessage::LayoutChanged,
        );

        let layout_section: Element<_> = match layout {
            Layout::Row => {
                row![row_radio, column_radio].spacing(spacing).into()
            }
            Layout::Column => {
                column![row_radio, column_radio].spacing(spacing).into()
            }
        };

        let spacing_section = column![
            slider(0..=80, spacing, StepMessage::SpacingChanged),
            text(format!("{spacing} px"))
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Center),
        ]
        .spacing(10);

        Self::container("Rows and columns")
            .spacing(spacing)
            .push(
                "Iced uses a layout model based on flexbox to position UI \
                 elements.",
            )
            .push(
                "Rows and columns can be used to distribute content \
                 horizontally or vertically, respectively.",
            )
            .push(layout_section)
            .push("You can also easily change the spacing between elements:")
            .push(spacing_section)
    }
	
    fn text(size: u16, color: Color) -> Column<'a, StepMessage> {
        let size_section = column![
            "You can change its size:",
            text(format!("This text is {size} pixels")).size(size),
            slider(10..=70, size, StepMessage::TextSizeChanged),
        ]
        .padding(20)
        .spacing(20);

        let color_sliders = row![
            color_slider(color.r, move |r| Color { r, ..color }),
            color_slider(color.g, move |g| Color { g, ..color }),
            color_slider(color.b, move |b| Color { b, ..color }),
        ]
        .spacing(10);

        let color_section = column![
            "And its color:",
            text(format!("{color:?}")).style(color),
            color_sliders,
        ]
        .padding(20)
        .spacing(20);

        Self::container("Text")
            .push(
                "Text is probably the most essential widget for your UI. \
                 It will try to adapt to the dimensions of its container.",
            )
            .push(size_section)
            .push(color_section)
    }


}


/*
Funciones utilitarias
*/

fn button<'a, Message: Clone>(label: &str) -> Button<'a, Message> {
    iced::widget::button(
        text(label).horizontal_alignment(alignment::Horizontal::Center),
    )
    .padding(12)
    .width(100)
}

fn color_slider<'a>(
    component: f32,
    update: impl Fn(f32) -> Color + 'a,
) -> Slider<'a, f64, StepMessage, Renderer> {
    slider(0.0..=1.0, f64::from(component), move |c| {
        StepMessage::TextColorChanged(update(c as f32))
    })
    .step(0.01)
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Row,
    Column,
}
