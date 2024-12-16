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
        name: "Est ce que N = p² ? ",
        is_valid: false,
    },
    TestStatus {
        name : "Est ce que N est factorisable car trop petit ? ",
        is_valid: false,
    },
    TestStatus {
        name : "Est ce que N est premiers (et on peut donc déchiffrer le ct)? ",
        is_valid: false,
    },
    TestStatus {
        name : "Autre test à implémenter dans le futur...",
        is_valid: false,
    },
    ])
);

//Fonction utilisé pour trouvé le message et renvoyer le ct re chiffré a partir de phi_n,n,e,ct.
//La fonction affiche également le résultat sur la page safe_enc_page
fn find_message(phi_n : &RsaBigUint, n: &RsaBigUint, e : &RsaBigUint, ct:&RsaBigUint,safe_enc_page: &mut SafeRsaChifPage) -> RsaBigUint {
    let d = super::utils::inverse(e, phi_n);
    let message = ct.modpow(&d,n);
    safe_enc_page.display_message(&String::from_utf8_lossy(&message.to_bytes_be()));
    let ct_test = message.modpow(e,n);
    return ct_test
}

fn test_n_facteur_carre(n: &RsaBigUint,e:&RsaBigUint,ct : &RsaBigUint,safe_enc_page: &mut SafeRsaChifPage) -> bool{
    //Vérifie si n est un carré
    let p: RsaBigUint = n.sqrt();
    if &p * &p == *n {
        let phi_n = &p*(&p-RsaBigUint::from(1u8));
        if super::utils::pgcd(e,&phi_n) == RsaBigUint::from(1u8) { // Pour etre sur que l'inverse existe
            let d = super::utils::inverse(e, &phi_n);
            let message = ct.modpow(&d,n);
            safe_enc_page.display_message(&String::from_utf8_lossy(&message.to_bytes_be()));
            let ct_test = message.modpow(e,n);
            return &ct_test==ct
        }else {
            return false;
        }
    }
    false
}

fn test_is_factorisable_too_small(n : &RsaBigUint,safe_enc_page: &mut SafeRsaChifPage) -> bool {
    if n.bits() < 2048{
        //Implémenter un algorithme de factorisation, ou se servir de "reqwest", pour envoyer une requete a factordb
        //Pour cela il faut apprendre a bien utiliser async, et voir si c'est possible.
        //Dans cette version de l'application (rendu final), on regarde juste si N est trop petit, auquel cas l'utilisateur
        //Peut aller voir sur factordb
        safe_enc_page.display_message(&String::from("N est factorisable car il est trop petit, par manque de temps nous n'avons pas implémenté cette fonctionnalité, mais n'hésitez pas a vous rendre sur factordb.com"));
        return true;
    }
    false
}


use num_primes::Verification;
fn n_is_prime(n: &RsaBigUint,e:&RsaBigUint,ct : &RsaBigUint,safe_enc_page: &mut SafeRsaChifPage) -> bool {
    let bytes_n = n.to_bytes_be();
    let n_prime = num_primes::BigUint::from_bytes_be(&bytes_n);
    if Verification::is_prime(&n_prime){
        let phi_n = n - RsaBigUint::from(1u8);
        let ct_test = find_message(&phi_n,&n,&e,&ct,safe_enc_page);
        return &ct_test==ct
    }
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
    let ct = RsaBigUint::from_str(&ct_value).expect("Conversion échouée");
    let _p = RsaBigUint::from_str(&ct_value).expect("Conversion échouée");
    let _pub_key : RsaPublicKey = match create_public_key(&n,&e){
        Some(pub_key) => pub_key,
        None => {
            safe_enc_page.add_error_message("Il y a eu une erreur lors de la création de la clé publique");
            return;
          }  //si erreur : on met un message d'erreur et on sort complétement de la fonction calc_all_safety_status
    };
    update_test_status(&mut ALL_TEST_STATUS_SECU_RSA.lock().unwrap(), 0, test_n_facteur_carre(&n,&e, &ct,safe_enc_page));
    update_test_status(&mut ALL_TEST_STATUS_SECU_RSA.lock().unwrap(), 1, test_is_factorisable_too_small(&n,safe_enc_page));
    update_test_status(&mut ALL_TEST_STATUS_SECU_RSA.lock().unwrap(), 2, n_is_prime(&n,&e, &ct,safe_enc_page));
} 