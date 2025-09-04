mod vigenere {
    pub fn encrypt(plaintext: &str, key: &str) -> String {
        let mut encrypted = String::new();
        let alphabet = ('a'..='z').collect::<String>();
        // Construct the encryption key to match the length of the text
        let mut key_rep = key.repeat(plaintext.len() / key.len()).to_string();
        key_rep.push_str(&key[..(plaintext.len() % key.len())]);
        // Go through all the chars in the text by index
        for (idx, ch) in plaintext.to_lowercase().chars().enumerate() {
            // Get the char in the plaintext that correspond to that index
            if let Some(p) = alphabet.find(ch) {
                // Get the index from the repeated key
                let k_char = key_rep.to_lowercase().chars().nth(idx)
                    .expect("unexpected error while retrieving the corresponding index from the repeated key");
                // Get the char from the alphabet that correspond to the repeated key index
                let k = alphabet.find(k_char).unwrap();
                let c = (p + k) % alphabet.len();
                encrypted.push(
                    alphabet
                        .chars()
                        .nth(c)
                        .expect("Error getting nth char from alphabet"),
                );
            }
        }
        encrypted
    }

    pub fn decrypt(ciphertext: &str, key: &str) -> String {
        let mut decrypted = String::new();
        let alphabet = ('a'..='z').collect::<String>();
        // Construct the decryption key to match the length of the text
        let mut key_rep = key.repeat(ciphertext.len() / key.len()).to_string();
        key_rep.push_str(&key[..(ciphertext.len() % key.len())]);
        // Go through all the chars in the encrypted text by index
        // Make sure we take only the alphabetic chars
        let clean_ciphertext = ciphertext.chars().filter(|x| x.is_alphabetic()).collect::<String>();
        for (idx, ch) in clean_ciphertext.to_lowercase().chars().enumerate() {
            // Get the char in the encrypted text that correspond to that index
                if let Some(p) = alphabet.find(ch) {
                    // Get the index from the repeated key
                    let k_char = key_rep.to_lowercase().chars().nth(idx%key_rep.len())
                        .expect("unexpected error while retrieving the corresponding index from the repeated key");
                    // Get the char from the alphabet that correspond to the repeated key index
                    let k = alphabet.find(k_char).unwrap();
                    let c_tmp = p as isize - k as isize;
                    let mut c = match c_tmp < 0 {
                        true => (c_tmp + 26) as usize,
                        false => c_tmp as usize,
                    };
                    c %= alphabet.len();
                    decrypted.push(
                        alphabet
                            .chars()
                            .nth(c)
                            .expect("Error getting nth char from alphabet"),
                    );
                }
        }
        decrypted
    }
}

fn main() {
    let key = "WHYRUST";
    let ciphertext = "
    PVCDJG
    PAYCMY
    JR KUC
    ";
    let ciphertext2 = "PVCDJGPAYCMYJRKUC";
    let plaintext = vigenere::decrypt(ciphertext, key);
    println!("decrypted: {}", plaintext);

    let plaintext2 = vigenere::decrypt(ciphertext2, key);
    println!("decrypted: {}", plaintext2);

    let encrypted = vigenere::encrypt("toempowereveryone", key);
    println!("encrypted: {}", encrypted);
}
