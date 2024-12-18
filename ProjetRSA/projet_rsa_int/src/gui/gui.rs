// Modules iced
use iced::theme::Theme;
use iced::widget::button;
use iced:: { Sandbox, Element, Background, Shadow, Vector, Border};
//Nos modules 
use super::check_enc_page;
use super::safe_enc_page;
use super::home_page;
use crate::rsa::keygen::generate_rsa_private_key;
use crate::rsa::keygen::generate_rsa_public_key;


// Structure principale de l'application
pub struct App{
    current_page: Page, // Page 1,2 ,3 etc 
    home_page : home_page::HomePage,
    valid_rsa_chif_page: check_enc_page::ValidRsaChifPage,
    secu_rsa_chif_page : safe_enc_page::SafeRsaChifPage,
    theme: Theme,
}

impl Sandbox for App{
    type Message = Message;

    // Initialisation de l'application
    fn new() -> Self {
        Self {
            current_page: Page::Home, // Page 1,2 ,3 etc 
            home_page : home_page::HomePage::new(),
            valid_rsa_chif_page: check_enc_page::ValidRsaChifPage::new(),
            secu_rsa_chif_page : safe_enc_page::SafeRsaChifPage::new(),
            theme: Theme::Dark, 
        }
    }

    // Méthode update pour gérer les événements
    fn update(&mut self, message: Message) {
        match message {

            // Changement de page
            Message::Router(page)=> {
                self.current_page = page;
            }

            // Mise à jour des champs pour la page "Validité RSA Chiffrement"
            Message::FieldChangedRsaChiff(n_val,p_val ,q_val ,e_val ,d_val ) => {
                self.valid_rsa_chif_page.update(n_val,p_val ,q_val ,e_val ,d_val );
            }

            // Mise à jour des champs pour la page "Sécurité RSA Chiffrement"
            Message::FieldChangedRsaChiffSecu(n_val,e_val ,ct_val ) => {
                self.secu_rsa_chif_page.update(n_val,e_val,ct_val);
            }

            // Vérification des valeurs pour la page "Validité RSA Chiffrement"
            Message::CheckButtonPressedRsaChiff =>{
                self.valid_rsa_chif_page.check_values();
            }

            // Vérification des valeurs pour la page "Sécurité RSA Chiffrement"
            Message::CheckButtonPressedRsaChiffSecu =>{
                self.secu_rsa_chif_page.remove_display_message();
                self.secu_rsa_chif_page.check_values();
            }

            // Génération de nouvelles valeurs pour la page "Validité RSA Chiffrement" 
            Message::NewValuesRsaEnc => {
                let key = generate_rsa_private_key(2048);
                self.valid_rsa_chif_page.reset_status(); // Fonction qui va appeler all_status_to_false, mais faites pour ne pas avoir a passer ALL_TEST_STATUS_VALID_RSA ici 
                self.valid_rsa_chif_page.remove_all_error_message(); // On mets de nouvelles valeurs donc on remet les status de tests a false
                self.valid_rsa_chif_page.update(
                    key[0].to_string(),
                    key[2].to_string(),
                    key[3].to_string(),
                    key[1].to_string(),
                    key[4].to_string(),
                );
            }

            // Génération de nouvelles valeurs pour la page "Sécurité RSA Chiffrement"
            Message::NewValuesRsaEncSecu => {
                let key = generate_rsa_public_key(2048); // Génere une clé publique valide et un ct valide
                self.secu_rsa_chif_page.reset_status(); // Fonction qui va appeler all_status_to_false, mais faites pour ne pas avoir a passer ALL_TEST_STATUS_VALID_RSA ici 
                self.secu_rsa_chif_page.remove_all_error_message(); // On mets de nouvelles valeurs donc on remet les status de tests a false
                self.secu_rsa_chif_page.remove_display_message();
                self.secu_rsa_chif_page.update(
                    key[0].to_string(),
                    key[1].to_string(),
                    key[2].to_string(),
                );
            }
        }
    }


    //Méthode view -> permet d'afficher la page actuelle
    fn view(&self) -> Element<Message> {
        match self.current_page {
            Page::Home => self.home_page.view(),
            Page::ValiditeRSAChiffrement => self.valid_rsa_chif_page.view(),
            Page::SecuriteRsaChiffrement => self.secu_rsa_chif_page.view(),
        }
    }

    // Titre de l'application
    fn title(&self) -> String {
        String::from("Projet Rust RSA")
    }

    fn theme(&self) ->Theme {
        self.theme.clone()
    }
}




// Enum pour les pages (router)
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum Page {
    Home,
    ValiditeRSAChiffrement,
    SecuriteRsaChiffrement,
    //Rajouter des pages si on en créer de nouvelles
} 


// Enum pour les événements/callback (ces callback sont définis au dessus)
#[derive(Debug,Clone)]
pub enum Message{
    Router(Page), // Change la page en fonction de la route indique
    FieldChangedRsaChiff(String, String, String, String, String),
    FieldChangedRsaChiffSecu(String,String,String),
    CheckButtonPressedRsaChiff,
    CheckButtonPressedRsaChiffSecu,
    NewValuesRsaEnc,
    NewValuesRsaEncSecu,
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

