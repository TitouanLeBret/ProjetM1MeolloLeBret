//Page sur validite RSA Chiffrement
use iced::theme::{Theme};
use iced::widget::{button, container, text, text_input, Button, Checkbox, Column, Container, Row, Text, TextInput};
use iced:: { Alignment, Sandbox, Settings, Element, Background, Shadow, Vector, Border, Padding,Length};
use iced::alignment::{Horizontal, Vertical};
use iced_core::text::Paragraph;

use super::gui;
use crate::rsa::check_enc;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TestStatus {
    pub name: &'static str,
    pub is_valid: bool,
}

#[derive(Default,Debug,Clone,PartialEq,Eq)]
pub struct ValidRsaChifPage {
    n_value: String,
    p_value: String,
    q_value: String,
    e_value: String,
    d_value: String,
    check_button: button::State,
    tests: Vec::<TestStatus>,
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
            tests: vec![
                TestStatus {
                    name: "Test de sécurité complet (faire une version ou on voit le résultat de chaque test)",
                    is_valid: false,
                },
            ],
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
        let title = Text::new("Validité Chiffrement RSA")
            .size(48)
            .horizontal_alignment(Horizontal::Center)
            .style(Color::from_rgb(0.2, 0.2, 0.6));

        //Boutton pour générer une nouvelle clé RSA valide
        let new_values_button = button(text("Générer une clé RSA valide "))
            .padding(10)
            .on_press(gui::Message::NewValuesRsaEnc);

        //Section pour la partie de la clé publique
        let pub_key_section = Column::new()
            .spacing(10)
            .push(Text::new("Clé publique :"))
            .push(Row::new()// N : text field et e : text field
                .spacing(15)
                .push(Text::new("N :"))
                .push(MyTextInput::new("N", &self.n_value).width(Length::Fill).on_input(
                    | n_value | {
                        gui::Message::FieldChangedRsaChiff(
                            n_value, 
                            self.p_value.clone(),
                            self.q_value.clone(),
                            self.e_value.clone(),
                            self.d_value.clone(),
                            )
                        
                        }
                    ))
                .push(Text::new("e :"))
                .push(MyTextInput::new("e", &self.e_value).width(Length::Fill).on_input(
                    | e_value | {
                        gui::Message::FieldChangedRsaChiff(
                            self.n_value.clone(), 
                            self.p_value.clone(),
                            self.q_value.clone(),
                            e_value,
                            self.d_value.clone(),
                            )
                        }
                    ))
            );

        //Section pour la partie de la clé privée
        let priv_key_section = Column::new()
            .spacing(10)
            .push(Text::new("Clé privée :"))
            .push(Row::new()
                .spacing(15)
                .push(Text::new("p :"))
                .push(MyTextInput::new("p", &self.p_value).width(Length::Fill).on_input(
                    | p_value | {
                        gui::Message::FieldChangedRsaChiff(
                            self.n_value.clone(), 
                            p_value,
                            self.q_value.clone(),
                            self.e_value.clone(),
                            self.d_value.clone(),
                            )
                        }
                    ))
                .push(Text::new("q :"))
                .push(MyTextInput::new("q", &self.q_value).width(Length::Fill).on_input(
                    | q_value | {
                        gui::Message::FieldChangedRsaChiff(
                            self.n_value.clone(), 
                            self.p_value.clone(),
                            q_value,
                            self.e_value.clone(),
                            self.d_value.clone(),
                            )
                        }
                    ))
                .push(Text::new("d :"))
                .push(MyTextInput::new("d", &self.d_value).width(Length::Fill).on_input(
                    | d_value | {
                        gui::Message::FieldChangedRsaChiff(
                            self.n_value.clone(), 
                            self.p_value.clone(),
                            self.q_value.clone(),
                            self.e_value.clone(),
                            d_value,
                            )
                        }
                    ))
            );

        //Boutton pour vérifier la validité des clés RSA
        let check_button = button(text("Vérifier la validité"))
            .padding(10)
            .on_press(gui::Message::CheckButtonPressedRsaChiff);

        // Section des tests de sécurité avec les cases validées ou non
        let test_results = self.tests.iter()
            .fold(Column::new()
                .spacing(10), |column: Column<'_, gui::Message>, test| { column
                    .push(
                        Row::new()
                            .spacing(10)
                            .push(Text::new(test.name))
                            .push(
                                Checkbox::new(
                                    "",
                                    test.is_valid,
                                )
                                //.style(if test.is_valid { ValidStyle::Checked } else { ValidStyle::Unchecked })
                            )
                    )
                }
            );


        let wrapper = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(title)
            .push(new_values_button)
            .push(pub_key_section)
            .push(priv_key_section)
            .push(check_button)
            .push(test_results);

        container(wrapper)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y() // Fond standard
            .into()
    }

    //Getter et setter : 
    pub fn get_tests_status(&self) -> Vec<TestStatus> {
        self.tests.clone()
    }


    //Méthode qui affiche les valeurs entrées dans les champs
    pub fn display_values(&self) {
        let p : num_bigint::BigUint = self.p_value.parse().expect("Echec conversion");
        let q : num_bigint::BigUint = self.q_value.parse().expect("Echec conversion");
        println!("Values are: N:{}\n, E:{}\n, P:{}\n, Q:{}\n, D:{}\n, p*q = {}\n",self.n_value.clone(), self.e_value.clone(),self.p_value.clone(),self.q_value.clone(),self.d_value.clone(),p*q);
    }

    pub fn check_values(&mut self) -> Vec<TestStatus>{
        let all_test_status = check_enc::all_security_tests_status(self.n_value.clone(), self.e_value.clone(), self.p_value.clone(), self.q_value.clone(), self.d_value.clone());
        for i in 0..all_test_status.len() {
            self.tests[i].is_valid = all_test_status[i].is_valid;
        }
        all_test_status
    }

}
/*
impl gui::PageContent for ValidRsaChifPage {
    fn view(&self) -> Element<gui::Message> {
        let wrapper = Column::new()
            .push(text("Validité Chiffrement RSA").size(64))
            .push(MyTextInput::new("N : ", &self.n_value)
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
            .push(MyTextInput::new("e : ", &self.e_value))
            .push(MyTextInput::new("p : ", &self.p_value))
            .push(MyTextInput::new("q : ", &self.q_value))
            .push(MyTextInput::new("d : ", &self.d_value));
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

//      Text input field
use iced::Color;
use iced::widget::text_input::StyleSheet;
use iced::widget::text_input::Appearance;
struct MyTextInput ;

impl MyTextInput {
    fn create() -> Self{
        MyTextInput
    }

    pub fn new( _placeholder:&str,_value : &str,) -> TextInput<'static, gui::Message> {
        TextInput::new(_placeholder,_value)
            .line_height(text::LineHeight::Relative(1.75))
            .padding(Padding::from(12))
            .width(Length::Fixed(300.0))
            .style(iced::theme::TextInput::Custom(Box::new(MyTextInput::create())))
    }
}

impl StyleSheet for MyTextInput {
    type Style = Theme;

    fn active(&self, _: &Self::Style) -> Appearance {
        text_input::Appearance {
            background: Background::Color(iced::Color::from_rgb8(5, 11, 31)),
            border: Border::with_radius(10),
            icon_color: iced::Color::from_rgb(0.8, 0.8, 0.8),
        }
    }

    fn focused(&self, _: &Self::Style) -> Appearance {
        text_input::Appearance {
            background: Background::Color(iced::Color::from_rgb8(5, 11, 51)),
            border: Border::with_radius(10),
            icon_color: iced::Color::from_rgb(0.8, 0.8, 0.8),
        }
    }

    fn placeholder_color(&self, _: &Self::Style) -> Color {
        Color::from_rgb8(70, 70, 70)
    }

    fn value_color(&self, _: &Self::Style) -> Color {
        Color::from_rgb8(255, 255, 255)
    }

    fn selection_color(&self, _: &Self::Style) -> Color {
        Color::from_rgb(0.8, 0.8, 1.0)
    }

    fn disabled_color(&self, _: &Self::Style) -> Color {
        Color::from_rgb(0.6, 0.6, 0.6)
    }

    fn disabled(&self, _: &Self::Style) -> Appearance { //Mettre tout en gris
        text_input::Appearance {
            background: Background::Color(iced::Color::from_rgb(0.9, 0.9, 0.9)),
            border: Border::with_radius(5),
            icon_color: iced::Color::from_rgb(1., 1., 1.),
        }
    }
}