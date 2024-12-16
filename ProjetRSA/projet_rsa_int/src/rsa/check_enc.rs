//docs : https://docs.rs/rsa/latest/rsa/
use rsa::{RsaPrivateKey, RsaPublicKey };
use rsa::BigUint as RsaBigUint;
use rsa::traits:: {PrivateKeyParts,PublicKeyParts}; 


use once_cell::sync::Lazy;
use std::sync::Mutex;

use super::utils::TestStatus;
use super::utils::update_test_status;

//Ce vecteur a un Mutex, pour pouvoir etre une variable globale mutable,
//On s'en sert dans check_enc_page.rs, mais on pourrais récréer une autre page qui s'en sert sans problème
pub static ALL_TEST_STATUS_VALID_RSA : Lazy<Mutex<Vec<TestStatus>>> = Lazy::new(||
    Mutex::new(vec![
    TestStatus {
        name: "Test de la taille de N (>= 2048 bits)",
        is_valid: false,
    },
    TestStatus {
        name: "Test de factorisation (N=p*q)",
        is_valid: false,
    },
    TestStatus {
        name: "Test de chiffrement / déchiffrement",
        is_valid: false,
    },
    TestStatus {
        name: "Test de sécurité sur e et d",
        is_valid: false,
    },
    TestStatus {
        name: "Test de sécurité complet",
        is_valid: false,
    },
    TestStatus {
        name: "Rajouter test dans ALL_TEST_STATUS",
        is_valid: false,
    },
    ])
);





use std::str::FromStr;

pub fn calc_all_security_tests_status(n_value : String , e_value: String, p_value: String, q_value: String , d_value: String) {
    //Return une énum avec les différents tests associés a leur validité ou non
    let n = RsaBigUint::from_str(&n_value).expect("Conversion échouée");
    let e = RsaBigUint::from_str(&e_value).expect("Conversion échouée");
    let p = RsaBigUint::from_str(&p_value).expect("Conversion échouée");
    let q = RsaBigUint::from_str(&q_value).expect("Conversion échouée");
    let d = RsaBigUint::from_str(&d_value).expect("Conversion échouée");
    let priv_key : RsaPrivateKey = RsaPrivateKey::from_components(n.clone(), e.clone(), d.clone(),vec![p.clone(), q.clone()]).expect("Conversion échouée");
    let pub_key : RsaPublicKey = RsaPublicKey::from(&priv_key);
    let validation_bits_pub_key =  bits_pub_key(&n) ;
    let validation_facto = is_valid_factorisation(&n, &p, &q) ;
    let validation_enc_dec =  is_valid_encryption_decryption( &pub_key, &priv_key) ;
    let validation_e_d = are_valide_e_d(&pub_key,&priv_key);
    let validation_complete = validation_bits_pub_key && validation_facto && validation_enc_dec && validation_e_d;
    update_test_status(&mut ALL_TEST_STATUS_VALID_RSA.lock().unwrap(),0,validation_bits_pub_key);
    update_test_status(&mut ALL_TEST_STATUS_VALID_RSA.lock().unwrap(),1,validation_facto);
    update_test_status(&mut ALL_TEST_STATUS_VALID_RSA.lock().unwrap(),2,validation_enc_dec);
    update_test_status(&mut ALL_TEST_STATUS_VALID_RSA.lock().unwrap(),3,validation_e_d);
    update_test_status(&mut ALL_TEST_STATUS_VALID_RSA.lock().unwrap(),4,validation_complete);
}


use num_primes::Verification;
//Toutes les fonctions utilisés dans all_security_tests :
fn bits_pub_key(n : &RsaBigUint) -> bool {
    let n_bits = n.bits(); // Renvoie le nombre de bits de n
    n_bits >= 2048 //Vrai si >=2048 faut sinon
}


fn primalite(p : &RsaBigUint) -> bool { // !!!!!!!!!! Implémenter MILLER RABIN tout seul
    //Pour l'instant test de num_primes, masi essayant en le faisant nous même 
    //2 lignes du dessous pour converitr RsaBigUint en num_primes::BigUint
    //!!!!!!!!!!!!!!!Très importantes, elles doivent être réutiliser quand probleme de type entre les différente BigUInt utiliser.
    let bytes_p = p.to_bytes_be();
    let p_prime = num_primes::BigUint::from_bytes_be(&bytes_p);
    Verification::is_prime(&p_prime)
}


fn are_prime(p : &RsaBigUint, q : &RsaBigUint) -> bool {
    primalite(p) && primalite(q)
}


fn is_valid_factorisation(n : &RsaBigUint, p : &RsaBigUint, q : &RsaBigUint) -> bool {
    let n_calc = p *q; // Calcul du produit des facteurs premiers
    n.eq(&n_calc) && are_prime(p, q)
}


//Voir : https://docs.rs/rsa/latest/rsa/index.html
use rsa::Pkcs1v15Encrypt; // C'est le padding Pkcs1
fn is_valid_encryption_decryption(pub_key : &RsaPublicKey, priv_key : &RsaPrivateKey) -> bool {
    let data = b"Hello world! C'est  le test de chiffrement/dechiffrement";
    let mut rng = rand::thread_rng();
    let enc_data = pub_key.encrypt(&mut rng,Pkcs1v15Encrypt, &data[..]).unwrap_or_else(|_| vec![]); // si on ne peut pas unwrap on renvoie un vecteur vide

    let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).unwrap_or_else(|_| vec![]);// si on ne peut pas unwrap on renvoie un vecteur vide
    data == &dec_data[..] // On retourne vrai si c'est égale, faux sinon

}


//use num_integer::Integer; // Pour le trait gcd() --> Mais on a préférer refaire euclide étendue nous même
use super::utils::pgcd; 
//Fonction de calcul du pgcd (PEUT ETRE A METTRE DANS UN FICHIER utils.rs, car peut reservir)
fn are_valide_e_d(pub_key : &RsaPublicKey, priv_key : &RsaPrivateKey) -> bool {
    // Récupérer les valeurs de la clé publique et privée
    let e = pub_key.e(); // Exposant public
    let d = priv_key.d(); // Exposant privé
    let _n = pub_key.n(); // Modulus (N)
    let p = priv_key.primes()[0].clone(); // Premier facteur de N
    let q = priv_key.primes()[1].clone(); // Deuxième facteur de N

    // Calculer φ(N) = (p - 1) * (q - 1)
    let phi_n = (p - RsaBigUint::from(1u8)) * (q - RsaBigUint::from(1u8));

    // Vérifier que e et φ(N) sont premiers entre eux grâce aui pgcd
    if pgcd(&e,&phi_n) != RsaBigUint::from(1u8) {
        return false; // e n'est pas valide
    };

    // Vérifier que (e * d) % φ(N) == 1
    if (e * d) % &phi_n != RsaBigUint::from(1u8) {
        return false; // d n'est pas valide
    };

    true
}


// Module de test pour les tests unitaires
#[cfg(test)] //N'est compîlé que si "cargo test" est exécuté
mod tests {
    use super::*; // Import les elts du code principale
    #[test]//Test pour bits_pub_key
    fn test_bits_pub_key(){
        let mod1 = RsaBigUint::from(1u32); // 256 octect = 2048 bits
        let mut a = RsaBigUint::from(2u32); // 256 octect = 2048 bits
        let exposant1 = RsaBigUint::from(2047u32);
        a = a.modpow(&exposant1,&mod1);//C'est le plus petit nombre de 2048 bits (C'est 1 suivi de 2047 0)
        assert!(bits_pub_key(&a));

        let mut b = RsaBigUint::from(2u32); // 256 octect = 2048 bits
        let exposant2 = RsaBigUint::from(1024u32);
        b = b.modpow(&exposant2,&mod1);
        assert!(bits_pub_key(&b)==false);
    }

    use super::super::keygen;
    #[test]//Test pour is_valid_factorisation
    fn test_is_valid_factorisation(){
        let priv_key1 = keygen::generate_rsa_private_key(2048);
        let n1 = priv_key1[0].clone();//C'est le plus petit nombre de 2048 bits (C'est 1 suivi de 2047 0)
        let p1 = priv_key1[1].clone();
        let q1 = priv_key1[2].clone();
        assert!(is_valid_factorisation(&n1,&p1,&q1));


        let mod1 = RsaBigUint::from(1u32);
        let mut n2 = RsaBigUint::from(2u32); // 256 octect = 2048 bits
        let mut p2 = RsaBigUint::from(2u32);
        let mut q2 = RsaBigUint::from(2u32);
        let exp1 = RsaBigUint::from(2047u32);
        let exp2 = RsaBigUint::from(1024u32);
        let exp3 = RsaBigUint::from(1023u32);
        n2 = n2.modpow(&exp1,&mod1);//C'est le plus petit nombre de 2048 bits (C'est 1 suivi de 2047 0)
        p2 = p2.modpow(&exp2,&mod1);
        q2 = q2.modpow(&exp3,&mod1);
        assert!(is_valid_factorisation(&n2,&p2,&q2)==false); // Doit renvoyer false car p et q ne sont pas premiers

        let mut n3 = RsaBigUint::from(2u32); // 256 octect = 2048 bits
        let mut p3 = RsaBigUint::from(2u32);
        let mut q3 = RsaBigUint::from(2u32);
        let exp4 = RsaBigUint::from(2047u32);
        let exp5 = RsaBigUint::from(1024u32);
        let exp6 = RsaBigUint::from(1023u32);
        n3 = n3.modpow(&exp4,&mod1);//C'est le plus petit nombre de 2048 bits (C'est 1 suivi de 2047 0)
        p3 = p3.modpow(&exp5,&mod1);
        q3 = q3.modpow(&exp6,&mod1);
        assert!(is_valid_factorisation(&n3,&p3,&q3)==false); // Doit renvoyer false car p et q sont premier mais ne sont pas facteurs de N
    }
}