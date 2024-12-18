//Page sur validite RSA Chiffrement
use iced::widget::{button, container, text, Button, Checkbox, Column, Row, Text};
use iced:: { Alignment,  Element, Length};
use iced::alignment::Horizontal;
use iced::Color;

use super::gui;
use crate::rsa::check_enc;
use super::components::MyTextInput;

use crate::rsa::utils::all_status_to_false; //Pour remettre tous les status à false
use crate::rsa::check_enc::ALL_TEST_STATUS_VALID_RSA; //Pour utiliser la lsite de check_enc.rs

/// Gère l'état de la page "Validité Chiffrement RSA".
/// Cette structure contient les valeurs de clé publique et privée, 
/// ainsi que les éléments d'interface nécessaires (boutons, messages d'erreur, etc.).
#[derive(Default,Debug,Clone,PartialEq,Eq)]
pub struct ValidRsaChifPage {
    /// Valeur de N (clé publique).
    n_value: String,
    /// Valeur de p (clé privée, premier facteur premier).
    p_value: String,
    /// Valeur de q (clé privée, second facteur premier).
    q_value: String,
    /// Valeur de e (exposant public).
    e_value: String,
    /// Valeur de d (exposant privé).
    d_value: String,
    /// État du bouton pour déclencher la vérification de validité des clés.
    check_button: button::State,

    /// Liste des messages d'erreur affichés à l'utilisateur.
    error_messages: Vec<String>,
}


/// Initialise une nouvelle page "Validité Chiffrement RSA".
/// Les champs de clés sont vides par défaut.
impl ValidRsaChifPage {
    pub fn new() -> Self {
        Self {
            n_value: String::new(),
            p_value: String::new(),
            q_value: String::new(),
            e_value: String::new(),
            d_value: String::new(),
            check_button: button::State::new(),
            error_messages: Vec::new(),
        }
    }



    /// Met à jour les valeurs de clé publique et privée.
    ///
    /// # Arguments
    ///
    /// * `n_val` - Nouvelle valeur pour N.
    /// * `p_val` - Nouvelle valeur pour p.
    /// * `q_val` - Nouvelle valeur pour q.
    /// * `e_val` - Nouvelle valeur pour e.
    /// * `d_val` - Nouvelle valeur pour d.
    pub fn update(&mut self ,n_val: String,p_val: String ,q_val: String ,e_val: String ,d_val: String) {
        self.n_value = n_val.clone();
        self.p_value = p_val.clone();
        self.q_value = q_val.clone();
        self.e_value = e_val.clone();
        self.d_value = d_val.clone();
    }


    /// Génère la vue de la page en affichant les champs de saisie, les boutons, et les résultats des tests.
    /// Organisation : 8 gros élements : 
    /// -Boutton de retour a la page d'accueil
    /// -Titre
    /// -Boutton génération de valeurs
    /// -Section avec champs pour clé publique
    /// -Section avec champs pour clé privée
    /// -Boutton de validation
    /// -Section pour les résultats des tests
    /// -Section d'affichage des messages d'erreur
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
                        //Réinitialise tous les tests a False, car une case a était modifiée
                        all_status_to_false(&mut ALL_TEST_STATUS_VALID_RSA.lock().unwrap());  

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
                        //Réinitialise tous les tests a False, car une case a était modifiée
                        all_status_to_false(&mut ALL_TEST_STATUS_VALID_RSA.lock().unwrap());
                        
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
                        //Réinitialise tous les tests a False, car une case a était modifiée
                        all_status_to_false(&mut ALL_TEST_STATUS_VALID_RSA.lock().unwrap());

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
                        //Réinitialise tous les tests a False, car une case a était modifiée
                        all_status_to_false(&mut ALL_TEST_STATUS_VALID_RSA.lock().unwrap());

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
                        //Réinitialise tous les tests a False, car une case a était modifiée
                        all_status_to_false(&mut ALL_TEST_STATUS_VALID_RSA.lock().unwrap());

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
        let test_results = ALL_TEST_STATUS_VALID_RSA.lock().unwrap().iter()
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
        
        // Section des messages d'erreur
        let error_message_section: Column<'_, gui::Message> = self.error_messages.iter().fold(
            Column::new().spacing(5),
            |column, error| {
                column.push(Text::new(error.clone()).style(Color::from_rgb(0.8, 0.2, 0.2)))
            },
        );

        //Wrapper qui va contenir tous les élements de la page
        let wrapper = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(Button::new(Text::new("Retour a la page d'accueil"))
                .on_press(gui::Message::Router(gui::Page::Home)))
            .push(title)
            .push(new_values_button)
            .push(pub_key_section)
            .push(priv_key_section)
            .push(check_button)
            .push(test_results)
            .push(error_message_section);

        //On ajoute notre wrapper a notre page
        container(wrapper)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y() // Fond standard
            .into()
    }


    //Getter et setter :
    //Ajouter un message d'erreur a error_messages, qui est un vecteur auquel est lié : error_message_section
    fn add_error_message(&mut self, msg: &str) {
        self.error_messages.push(msg.to_string());
    }

    //fonction qui vide le vecteur error_messages (et donc aussi error_message_section )
    pub fn remove_all_error_message(&mut self){
        self.error_messages.clear();
    }

    //Remet tous les status de la liste a faux 
        pub fn reset_status(&mut self){//Permet a gui d'appeler all_status_to_false, sans connaitre ALL_TEST_STATUS_VCALID_RSA
        all_status_to_false(&mut ALL_TEST_STATUS_VALID_RSA.lock().unwrap());
    }
    

    /// Vérifie la validité de la clé RSA en exécutant tous les tests de sécurité.
    /// Met à jour les résultats des tests dans l'état de la page.
    pub fn check_values(&mut self){
        //Test pour voir si toutes les cases sont bien convertibles en int
        self.remove_all_error_message();
        match validate_inputs(&self.n_value.clone(), &self.e_value.clone(), &self.p_value.clone(), &self.q_value.clone(), &self.d_value.clone()) {
            Ok(_) => {
                check_enc::calc_all_security_tests_status(self.n_value.clone(), self.e_value.clone(), self.p_value.clone(), self.q_value.clone(), self.d_value.clone());
            }
            Err(errors) => {
                // Affiche une popup ou un message d'erreur
                let error_message = format!(
                    "Attention, les erreurs suivantes ont été détectées :\n{}",
                    errors.join("\n")
                );
                self.add_error_message(&error_message);
            }
        }
    }



}


use std::str::FromStr;
/// Fonction de validation des entré (vérif que ce sont bien des entiers valide sinon ajout un message d'erreur pour chacun des invalides)
fn validate_inputs(n_value: &str, e_value: &str, p_value: &str, q_value: &str, d_value: &str) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    // Essayer de convertir chaque valeur
    if num_bigint::BigUint::from_str(n_value).is_err() {
        errors.push("N n'est pas un entier valide.".to_string());
    }
    if num_bigint::BigUint::from_str(e_value).is_err() {
        errors.push("E n'est pas un entier valide.".to_string());
    }
    if num_bigint::BigUint::from_str(p_value).is_err() || num_bigint::BigUint::from_str(p_value).unwrap() <= num_bigint::BigUint::from(1u32) {
        errors.push("P n'est pas un entier valide (doit etre > 1).".to_string());
    }
    if num_bigint::BigUint::from_str(q_value).is_err() || num_bigint::BigUint::from_str(q_value).unwrap() <= num_bigint::BigUint::from(1u32) {
        errors.push("Q n'est pas un entier valide (doit etre > 1).".to_string());
    }
    if num_bigint::BigUint::from_str(d_value).is_err() {
        errors.push("D n'est pas un entier valide.".to_string());
    }

    if errors.is_empty() {
        Ok(()) // Pas d'erreurs
    } else {
        Err(errors) // Renvoie les erreurs
    }
}