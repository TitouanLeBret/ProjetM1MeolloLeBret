//Page sur validite RSA Chiffrement
use iced::theme::{Theme};
use iced::widget::{button, container, text, Button, Column, Container, Row, Text, TextInput,text_input};
use iced:: { Alignment, Sandbox, Settings, Element, Background, Shadow, Vector, Border, Padding,Length};
use iced::alignment::{Horizontal, Vertical};
use iced_core::text::Paragraph;

use super::gui;


#[derive(Default,Debug,Clone,PartialEq,Eq)]
pub struct ValidRsaChifPage {
    n_value: String,
    p_value: String,
    q_value: String,
    e_value: String,
    d_value: String,
    check_button: button::State,
}

impl ValidRsaChifPage {
    pub fn new() -> Self {
        Self {
            n_value: String::new(),
            p_value: String::new(),
            q_value: String::new(),
            e_value: String::new(),
            d_value: String::new(),
            check_button: button::State::new(),
        }
    }
    pub fn update(&mut self ,n_val: String,p_val: String ,q_val: String ,e_val: String ,d_val: String) {
        self.n_value = n_val.clone();
        self.p_value = p_val.clone();
        self.q_value = q_val.clone();
        self.e_value = e_val.clone();
        self.d_value = d_val.clone();
    }

    pub fn view(&self) -> Element<gui::Message> {
        let wrapper = Column::new()
            .push(text("Validité Chiffrement RSA").size(64))
            .push(input_field("N : ", &self.n_value)
                .on_input(
                    | n_value | {
                        gui::Message::FieldChangedRsaChiff(
                            n_value, 
                            self.p_value.clone(),
                            self.q_value.clone(),
                            self.e_value.clone(),
                            self.d_value.clone(),
                            )
                        }
                    )
                )
            .push(input_field("e : ", &self.e_value)
                .on_input(
                    | e_value | {
                        gui::Message::FieldChangedRsaChiff(
                            self.n_value.clone(), 
                            self.p_value.clone(),
                            self.q_value.clone(),
                            e_value,
                            self.d_value.clone(),
                            )
                        }
                    )
                )
            .push(input_field("p : ", &self.p_value)
                .on_input(
                    | p_value | {
                        gui::Message::FieldChangedRsaChiff(
                            self.n_value.clone(), 
                            p_value,
                            self.q_value.clone(),
                            self.e_value.clone(),
                            self.d_value.clone(),
                            )
                        }
                    )
            )
            .push(input_field("q : ", &self.q_value))
            .push(input_field("d : ", &self.d_value));
            //.push(chek_button);

        container(wrapper)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

}
/*
impl gui::PageContent for ValidRsaChifPage {
    fn view(&self) -> Element<gui::Message> {
        let wrapper = Column::new()
            .push(text("Validité Chiffrement RSA").size(64))
            .push(input_field("N : ", &self.n_value)
                .on_input(
                    | n_value | {
                        gui::Message::FieldChangedRsaChiff(
                            n_value, 
                            self.p_value.clone(),
                            self.q_value.clone(),
                            self.e_value.clone(),
                            self.d_value.clone(),
                            )
                        }
                    )
                )
            .push(input_field("e : ", &self.e_value))
            .push(input_field("p : ", &self.p_value))
            .push(input_field("q : ", &self.q_value))
            .push(input_field("d : ", &self.d_value));
            //.push(chek_button);

        container(wrapper)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    
    fn update(&mut self, message: gui::PageMessage) {
        match message {
            gui::PageMessage::FieldChanged(value) => {
                self.n_value = value;
            }
            gui::PageMessage::ButtonPressed => {
                // Action lorsque le bouton est pressé
            }
        }
    }
}
*/


//A mettre dans component : 
fn input_field( _placeholder:&str,_value : &str,) -> TextInput<'static, gui::Message> {
    TextInput::new(_placeholder,_value)
        .width(Length::Fixed(500.0))
        .padding(Padding::from(10))
        .line_height(text::LineHeight::Relative(1.75))
}