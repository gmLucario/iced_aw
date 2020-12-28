
use iced::{button, Align, Button, Column, Element, Length, Text};
use crate::Section;

pub struct ModalSection {
    open_button: button::State,
}

impl ModalSection {
    pub fn new() -> Self {
        Self {
            open_button: button::State::new(),
        }
    }
}

impl Section for ModalSection {
    
    type Message = crate::Message;

    fn header(&self) -> String {
        String::from("Modal")
    }

    fn content(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .align_items(Align::Center)
            .width(Length::Fill)
            .push(
                Button::new(&mut self.open_button, Text::new("Open modal!"))
                    .on_press(crate::Message::OpenPrimaryModal)
            )
            .into()
    }
        
}