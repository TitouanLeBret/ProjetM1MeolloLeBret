//docs : https://docs.rs/rsa/latest/rsa/
use rsa::{RsaPrivateKey, RsaPublicKey };
use rsa::BigUint as RsaBigUint;
use rsa::traits:: {PrivateKeyParts,PublicKeyParts}; 

//Fonction de génération de la clef RSA :
pub fn generate_rsa_private_key(bits: usize) -> Vec<RsaBigUint> {
    //On créer une clef RSA 
    let mut rng = rand::thread_rng();
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    let e = pub_key.e().clone();
    let p = priv_key.primes()[0].clone();
    let q = priv_key.primes()[1].clone();
    let d = priv_key.d().clone();
    let n = p.clone()*q.clone();
    vec![n,e,p,q,d]
}



use std::str::FromStr;
//Fonction de vérification de RSA : 
pub fn all_security_tests(n_value : String , e_value: String, p_value: String, q_value: String , d_value: String) -> bool/*-> enum<bool>*/ {
    //Return une énum avec les différents tests associés a leur validité ou non
    let mut validation = true;
    let n = RsaBigUint::from_str(&n_value).expect("Conversion échouée");
    let e = RsaBigUint::from_str(&e_value).expect("Conversion échouée");
    let p = RsaBigUint::from_str(&p_value).expect("Conversion échouée");
    let q = RsaBigUint::from_str(&q_value).expect("Conversion échouée");
    let d = RsaBigUint::from_str(&d_value).expect("Conversion échouée");
    let priv_key : RsaPrivateKey = RsaPrivateKey::from_components(n.clone(), e.clone(), d.clone(),vec![p.clone(), q.clone()]).expect("Conversion échouée");
    let pub_key : RsaPublicKey = RsaPublicKey::from(&priv_key);
    validation = validation && bits_pub_key(&n) && is_valid_factorisation(&n, &p, &q) && is_valid_encryption_decryption(&n, &pub_key, &priv_key) && are_valide_e_d();
    validation
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


fn are_prime_factors(p : &RsaBigUint, q : &RsaBigUint) -> bool {
    primalite(p) && primalite(q)
}


fn is_valid_factorisation(n : &RsaBigUint, p : &RsaBigUint, q : &RsaBigUint) -> bool {
    let n_calc = p *q; // Calcul du produit des facteurs premiers
    //println!("n_calc : {}", n_calc);
    //println!("n : {}", n);
    n.eq(&n_calc) && are_prime_factors(p, q)
}


fn is_valid_encryption_decryption(n : &RsaBigUint, pub_key : &RsaPublicKey, priv_key : &RsaPrivateKey) -> bool {
    true
}


fn are_valide_e_d() -> bool {
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


    #[test]//Test pour is_valid_factorisation
    fn test_is_valid_factorisation(){
        let priv_key1 = generate_rsa_private_key(2048);
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