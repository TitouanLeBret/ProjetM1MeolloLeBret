// Rust UI - Iced

// Modules ... 
use iced::theme::{Theme};
use iced::widget::{button, container, text, Button, Column, TextInput};
use iced:: { Alignment, Sandbox, Settings, Element, Background, Shadow, Vector, Border, Padding,Length};
use iced::alignment::{Horizontal, Vertical};

//Nos propres modules 
use super::check_enc_page;
use super::components;
use super::home_page;

pub fn main() -> iced::Result {
    RustUI::run(Settings::default())
}

pub struct RustUI{
    //Définir les variable principale
    theme: Theme, // Noir ou blanc
    page: Page, // Page 1,2 ,3 etc 
}


//enum pour les pages, chaque variable dans Page va créer une nouvelle view/page
#[derive(Debug,Clone,PartialEq,Eq)]
enum Page{Accueil,ValiditeRSAChiffrement, ValiditeRSASignature, SecuriteRSAChiffrement, SecuriteRSASignature} //Peut importe les noms de variable

//callback events
#[derive(Debug,Clone)]
pub enum Message {
    ToggleTheme, //light/dark
    Router(String), // Change la page en fonctio nde la route indique
}

//Sandbox pour RustUI

impl Sandbox for RustUI{
    type Message = Message;

    fn new() -> Self {
        Self {
            theme: Theme::Dark, // Drak theme
            page: Page::Accueil,// Page de login
        }
    }

    //definir le titre de l'app
    fn title(&self) -> String {
        String::from("Rust UI - Iced")
    }

    fn theme(&self) ->Theme {
        self.theme.clone() //retourn une copie du theme
    }

    //Définir la méthode update
    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleTheme => {
                self.theme = if self.theme == Theme::Light {
                    Theme::Dark
                }else {
                    Theme::Light
                }
            }
            /*
            Message::LoginFieldChanged(email, password) => {
                self.login_field.email = email;
                self.login_field.password = password;
            }*/
            Message::Router(route)=> {
                if route == "accueil" {
                    self.page = Page::Accueil;
                } else if route == "check_enc_page" {
                    self.page = Page::ValiditeRSAChiffrement;
                } 
                /*else if route == "validRsaSign" {
                    self.page = Page::ValiditeRSASignature;
                }else if route == "secuRsaChif" {
                    self.page = Page::SecuriteRSAChiffrement;
                } else if route == "secuRsaSign" {
                    self.page = Page::SecuriteRSASignature;
                }*/
            }
            
        }
    }

    //Méthode view -> c'est ou l'UI va chercher la page
    fn view(&self) -> Element<Message> {

        let content = match self.page {
            Page::Accueil => home_page::page_accueil(), //page_accueil(),
            Page::ValiditeRSAChiffrement => check_enc_page::check_enc_page_fn(),
            Page::ValiditeRSASignature => check_enc_page::check_enc_page_fn(),
            Page::SecuriteRSAChiffrement => check_enc_page::check_enc_page_fn(),
            Page::SecuriteRSASignature => check_enc_page::check_enc_page_fn(),
        };

        let wrapper = Column::new()
            .spacing(50)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(content)
            .push(
                match self.page {
                    Page::Accueil => components::page_footer(
                        vec![
                        button("Validite RSA Chiffrement")
                            .on_press(Message::Router("check_enc_page".to_string()))
                            .style(iced::theme::Button::Custom(Box::new(ButtonStyle::ThemeButton))),
                        button("Validite RSA Signature")
                            .on_press(Message::Router("validRsaSign".to_string()))
                            .style(iced::theme::Button::Custom(Box::new(ButtonStyle::ThemeButton))),
                        button("Securite RSA Chiffrement")
                            .on_press(Message::Router("secuRsaChif".to_string()))
                            .style(iced::theme::Button::Custom(Box::new(ButtonStyle::ThemeButton))),
                        button("Securite RSA Signature")
                            .on_press(Message::Router("secuRsaSign".to_string()))
                        .style(iced::theme::Button::Custom(Box::new(ButtonStyle::ThemeButton)))
                        ]
                    ),
                    Page::ValiditeRSAChiffrement | Page::ValiditeRSASignature | Page::SecuriteRSAChiffrement | Page::SecuriteRSASignature => components::page_footer(
                        vec![
                        button("Page d'accueil")
                            .on_press(Message::Router("accueil".to_string()))
                            .style(iced::theme::Button::Custom(Box::new(ButtonStyle::ThemeButton)))
                        ]
                    ),
                }
            );

        //Text::new("Hello, World!").into();
        //let btn = submit_btn("Boutton de test", Message::ToggleTheme);

        container(wrapper)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(Padding::from(20))
            .center_x()
            .center_y()
            .style(iced::theme::Container::Custom(Box::new(ContainerStyle)))
            .into()
    }
}















//Style des champs d'entrées

fn input_field( _placeholder:&str,_value : &str,) -> TextInput<'static, Message> {
    TextInput::new(_placeholder,_value)
        .width(Length::Fixed(500.0))
        .padding(Padding::from(10))
        .line_height(text::LineHeight::Relative(1.75))
}

//Style du boutton

pub enum ButtonStyle {Standard, ThemeButton}

impl button::StyleSheet for ButtonStyle {
    type Style = Theme;

    //definir comportement actif 
    fn active(&self, theme : &Self::Style) -> button::Appearance{
        button::Appearance {
            background: Some(Background::Color(match self {
                Self::Standard => iced::Color::from_rgb(0.059,0.463,0.702),
                Self::ThemeButton => iced::Color::default(),
            })),
            border : match self {
                Self::Standard => Border::with_radius(5),
                Self::ThemeButton => Border::default(),
            },
            shadow: match self {
                Self::Standard => Shadow {
                    color : iced::Color::BLACK,
                    offset: Vector::new(0.0,4.0),
                    blur_radius:20.0,
                },
                Self::ThemeButton => Shadow::default(),
            },
            text_color : {
                if theme == &Theme::Light {
                     match self {
                        Self::Standard => iced::Color::WHITE,
                        Self::ThemeButton => iced::Color::BLACK,
                     }
                } else {
                     match self {
                        Self::Standard => iced::Color::BLACK,
                        Self::ThemeButton => iced::Color::WHITE,
                     }
                }
            },
            ..Default::default()
        }
    }
}

//Summit du boutton

fn submit_btn(name: &str, event:Message) -> Button<Message> {
    Button::new(
        text(name)
        .horizontal_alignment(Horizontal::Center)
        .vertical_alignment(Vertical::Center)
        .size(21)
    )
    .on_press(event)
    .width(Length::Fixed(500.0))
    .height(Length::Fixed(45.0))
    .style(iced::theme::Button::Custom(Box::new(ButtonStyle::Standard)))
}

//définir le style du conteneur 
struct ContainerStyle;

impl container::StyleSheet for ContainerStyle {
    type Style = Theme;

    fn appearance(&self , _theme: &Self::Style) -> container::Appearance {
        container::Appearance{
            text_color : Default::default(),
            border: Border::with_radius(5),
            background: None,
            shadow: Shadow {
                color: iced::Color::BLACK,
                offset: Vector::new(0.0,2.0),
                blur_radius: 40.0,
            },
        }
    }
}