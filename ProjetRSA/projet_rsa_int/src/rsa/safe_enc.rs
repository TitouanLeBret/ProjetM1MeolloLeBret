use super::utils::TestStatus;
use super::utils::update_test_status;
use once_cell::sync::Lazy;
use std::str::FromStr;
use crate::gui::safe_enc_page::SafeRsaChifPage;
use rsa::{BigUint as RsaBigUint, RsaPublicKey};
use std::sync::Mutex;
//Ce vecteur a un Mutex, pour pouvoir etre une variable globale mutable,
//On s'en sert dans check_enc_page.rs, mais on pourrais récréer une autre page qui s'en sert sans problème
pub static ALL_TEST_STATUS_SECU_RSA : Lazy<Mutex<Vec<TestStatus>>> = Lazy::new(||
    Mutex::new(vec![
    TestStatus {
        name: "Test de sécurité sur RSA",
        is_valid: false,
    },
    TestStatus {
        name: "Est ce que N = p² ? ",
        is_valid: false,
    },
    TestStatus {
        name : "Est ce que N est factorisable car trop petit ? ",
        is_valid: false,
    },
    TestStatus {
        name : "Est ce que N = p*q avec p et q non premiers ? ",
        is_valid: false,
    },
    TestStatus {
        name : "Est ce que e est trop petit ? ",
        is_valid: false,
    },
    TestStatus {
        name : "Autre test ",
        is_valid: false,
    },
    ])
);

fn test_qui_fait_rien() -> bool{
    return true
}

fn test_n_facteur_carre(n : &RsaBigUint) -> bool{
    //Vérifie si n est un carré
    let sqrt_n = n.sqrt();
    &sqrt_n * &sqrt_n == *n
}

fn test_is_factorisable_too_small(n : &RsaBigUint) -> bool {
    if n.bits() < 2048{
        /* 
        Implémenter ici l'algorithme qui test rapidement la factorisation, car N peut etre petit mais pas factorisable facilement pour autant
        On test ici si N est petit et qu'on arrive raisonnablement rapidement a le factoriser
         */
        return true;
    }
    false
}

fn test_is_factorisable_not_prime_factors(n : &RsaBigUint) -> bool {
    println!("{}",n);
    false
}

fn e_is_to_small(n : &RsaBigUint,e : &RsaBigUint) -> bool {
    println!("{}",n);
    println!("{}",e);
    false
}

//Fonction pour créer la clé
fn create_public_key(
    n: &RsaBigUint,
    e: &RsaBigUint,
) -> Option<RsaPublicKey> {
    match RsaPublicKey::new(n.clone(), e.clone()) {
        Ok(pub_key) => Some(pub_key),//clé publique si elle est créee   
        Err(_) => None//None si erreur
    }
}

pub fn calc_all_safety_status(safe_enc_page: &mut SafeRsaChifPage, n_value : String , e_value: String, ct_value: String) {
    //Return une énum avec les différents tests associés a leur validité ou non
    let n = RsaBigUint::from_str(&n_value).expect("Conversion échouée");
    let e = RsaBigUint::from_str(&e_value).expect("Conversion échouée");
    let _p = RsaBigUint::from_str(&ct_value).expect("Conversion échouée");
    let _pub_key : RsaPublicKey = match create_public_key(&n,&e){
        Some(pub_key) => pub_key,
        None => {
            safe_enc_page.add_error_message("Il y a eu une erreur lors de la création de la clé publique");
            return;
          }  //si erreur : on met un message d'erreur et on sort complétement de la fonction calc_all_safety_status
    };
    update_test_status(&mut ALL_TEST_STATUS_SECU_RSA.lock().unwrap(),0,test_qui_fait_rien());
    update_test_status(&mut ALL_TEST_STATUS_SECU_RSA.lock().unwrap(), 1, test_n_facteur_carre(&n));
    update_test_status(&mut ALL_TEST_STATUS_SECU_RSA.lock().unwrap(), 2, test_is_factorisable_too_small(&n));
    update_test_status(&mut ALL_TEST_STATUS_SECU_RSA.lock().unwrap(), 3, test_is_factorisable_not_prime_factors(&n));
    update_test_status(&mut ALL_TEST_STATUS_SECU_RSA.lock().unwrap(), 4, e_is_to_small(&n, &e));
} 