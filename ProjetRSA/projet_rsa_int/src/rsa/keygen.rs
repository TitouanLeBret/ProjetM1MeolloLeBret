use rsa::{RsaPrivateKey, RsaPublicKey ,Pkcs1v15Encrypt};
use rsa::BigUint as RsaBigUint;
use rsa::traits:: {PrivateKeyParts,PublicKeyParts}; 


//Fonction de génération de la clef RSA (publique et privée) :
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

//Fonction de génération de la clef (publique) RSA et d'un chiffré pour "Hello, world!": 
pub fn generate_rsa_public_key(bits: usize) -> Vec<RsaBigUint> {
    //On créer une clef RSA 
    let mut rng = rand::thread_rng();
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a public key");
    let pub_key = RsaPublicKey::from(&priv_key);
    let e = pub_key.e().clone();
    let p = priv_key.primes()[0].clone();
    let q = priv_key.primes()[1].clone();
    let n = p.clone()*q.clone();
    let data = b"Hello, world!";
    let ct = RsaBigUint::from_bytes_be(
            &pub_key
                .encrypt(&mut rng, Pkcs1v15Encrypt, &data[..])
                .expect("failed to encrypt")
    );

    //Test pour être sur que l'on puisse retrouver la data a partir du ct qui est en BigUInt et pas en Vec!(u8)
    try_to_decrypt(&ct,&priv_key);
    //be pour Big Endian
    vec![n,e,ct]
}

//Fonction qui essaye de déchiffré le ct a partir de la sk
fn try_to_decrypt(ct : &RsaBigUint, priv_key : &RsaPrivateKey) {
    let encrypted_bytes = ct.to_bytes_be(); // Convertir BigUint en Vec<u8>
    let _decrypted_data = priv_key
        .decrypt(Pkcs1v15Encrypt, &encrypted_bytes)
        .expect("Échec du déchiffrement");
}