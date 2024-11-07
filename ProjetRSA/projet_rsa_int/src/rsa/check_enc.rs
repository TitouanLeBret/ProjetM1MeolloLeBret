fn main() {
    println!("Hello, world!");
}

//docs : https://docs.rs/rsa/latest/rsa/
use rsa::{RsaPrivateKey, RsaPublicKey };
use rsa::traits::PrivateKeyParts; 
fn generate_rsa_private_key(bits: usize) -> Vec<BigUint> {
    //On créer une clef RSA 
    let mut rng = rand::thread_rng();
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    //On récupère les facteurs premiers de la clef privée
    let p = priv_key.primes()[0].clone();
    let q = priv_key.primes()[1].clone();
    //On renvoie n,p,q
    let n = BigUint::from_bytes_be(&p.to_bytes_be())*BigUint::from_bytes_be(&p.to_bytes_be());
    vec![n,BigUint::from_bytes_be(&p.to_bytes_be()),BigUint::from_bytes_be(&p.to_bytes_be())]
}

//Fonction de vérification de RSA : 
use num_bigint::BigUint;
fn all_security_tests(/*Clé publique, privé, exposant etc... */) /*-> enum<bool>*/ {
    //Return une énum avec les différents tests associés a leur validité ou non
}

fn bits_pub_key(n : &BigUint) -> bool {
    let n_bits = n.bits(); // Renvoie le nombre de bits de n
    n_bits >= 2048 //Vrai si >=2048 faut sinon
}

fn is_prime(p : &BigUint) -> bool {
    true
}

fn are_prime_factors(p : &BigUint, q : &BigUint) -> bool {
    is_prime(p) && is_prime(q)
}

fn is_valid_factorisation(n : &BigUint, p : &BigUint, q : &BigUint) -> bool {
    let n_calc = p*q;
    n.eq(&n_calc) && are_prime_factors(p, q)
}

fn is_valid_encryption_decryption(n : &BigUint, pub_key : &RsaPublicKey, priv_key : &RsaPrivateKey) -> bool {
    true
}


// Module de test
#[cfg(test)] //N'est compîlé que si cargo test
mod tests {
    use super::*; // Import les elts du code principale

    #[test]//Test pour bits_pub_key
    fn test_bits_pub_key(){
        let mut a = BigUint::from(2u32); // 256 octect = 2048 bits
        a = a.pow(2047);//C'est le plus petit nombre de 2048 bits (C'est 1 suivi de 2047 0)
        assert!(bits_pub_key(&a));

        let mut b = BigUint::from(2u32); // 256 octect = 2048 bits
        b = b.pow(1024);
        assert!(bits_pub_key(&b)==false);
    }


    #[test]//Test pour is_valid_factorisation
    fn test_is_valid_factorisation(){
        let priv_key1 = generate_rsa_private_key(2048);
        let n1 = priv_key1[0].clone();//C'est le plus petit nombre de 2048 bits (C'est 1 suivi de 2047 0)
        let p1 = priv_key1[1].clone();
        let q1 = priv_key1[2].clone();
        assert!(is_valid_factorisation(&n1,&p1,&q1));

        let mut n2 = BigUint::from(2u32); // 256 octect = 2048 bits
        let mut p2 = BigUint::from(2u32);
        let mut q2 = BigUint::from(2u32);
        n2 = n2.pow(2047);//C'est le plus petit nombre de 2048 bits (C'est 1 suivi de 2047 0)
        p2 = p2.pow(1024);
        q2 = q2.pow(1023);
        assert!(is_valid_factorisation(&n2,&p2,&q2)==false); // Doit renvoyer false car p et q ne sont pas premiers

        let mut n3 = BigUint::from(2u32); // 256 octect = 2048 bits
        let mut p3 = BigUint::from(2u32);
        let mut q3 = BigUint::from(2u32);
        n3 = n3.pow(2047);//C'est le plus petit nombre de 2048 bits (C'est 1 suivi de 2047 0)
        p3 = p3.pow(1024);
        q3 = q3.pow(1023);
        assert!(is_valid_factorisation(&n3,&p3,&q3)==false); // Doit renvoyer false car p et q sont premier mais ne sont pas facteurs de N
    }
}