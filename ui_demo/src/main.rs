use iced::{
    button, executor, Align, Application, Button, Column, Command, Element, Font,
    HorizontalAlignment, Length, Row, Settings, Subscription, Text,
};

#[derive(Debug, Clone)]
pub enum Message {
    Start,
    Stop,
    Reset,
}

pub enum TickState {
    Stopped,
    Ticking,
}

const FONT: Font = Font::External {
    name: "PixelMplus12-Regular",
    bytes: include_bytes!("../rsc/PixelMplus12-Regular.ttf"),
};

struct GUI {
    tick_state: TickState,
    start_stop_button_state: button::State,
    reset_button_state: button::State,
}

impl Application for GUI {
    type Executor = executor::Null;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (GUI, Command<Self::Message>) {
        (
            GUI {
            tick_state: TickState::Stopped,
            start_stop_button_state: button::State::new(),
            reset_button_state: button::State::new(),
        }, 
        Command::none())
    }
    
    fn title(&self) -> String {
        String::from("Demo")
    }
    
    fn update(&mut self, _message:Self::Message) -> Command<Self::Message>{
        Command::none()
    }
    
    fn view(&mut self) -> Element<Self::Message> {
        // init widgets
        //prepare duration text
        let duration_text = "00:00:00.00";

        let start_stop_text = match self.tick_state {
            TickState::Stopped => Text::new("Start")
            .horizontal_alignment(HorizontalAlignment::Center)
            .font(FONT),
            TickState::Ticking => Text::new("Stop")
            .horizontal_alignment(HorizontalAlignment::Center)
            .font(FONT),
        };
        
        let start_stop_message = match self.tick_state {
            TickState::Stopped => Message::Start,
            TickState::Ticking => Message::Stop,
        };
        
        let tick_text = Text::new(duration_text).font(FONT).size(60);

        let start_stop_button = Button::new(
            &mut self.start_stop_button_state,
            Text::new("Start")
            .horizontal_alignment(HorizontalAlignment::Center)
            .font(FONT),
        )
        .min_width(80);

        let reset_button = Button::new(
            &mut self.reset_button_state,
            Text::new("Reset")
            .horizontal_alignment(HorizontalAlignment::Center)
            .font(FONT),
        )
        .min_width(80);

        // prepare column
        Column::new()
        .push(tick_text)
        .push(
            Row::new()
            .push(start_stop_button)
            .push(reset_button)
            .spacing(10),
        )
        .spacing(10)
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_items(Align::Center)
        .into()
    }
}
fn main() {
    let mut settings = Settings::default();
    settings.window.size = (400u32, 120u32);
    GUI::run(settings);
}
