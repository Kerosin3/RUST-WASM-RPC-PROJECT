pub mod implement {

    use crate::Answer;
    use crate::TEST_MODE;
    use console::Style;
    use k256::ecdsa::signature::Verifier;
    use k256::ecdsa::{Signature, VerifyingKey};
    use libinteronnect::serdes::*;
    use libshmem::datastructs::MESSAGES_NUMBER;
    use libshmem::datastructs::*;
    use std::io::{Error, ErrorKind};
    pub fn verify_message_natively_ecdsa(
        recv_sig_msg: crossbeam_channel::Receiver<String>,
        recv_ver_key: crossbeam_channel::Receiver<Vec<u8>>,
        right_messages: Vec<Answer>,
    ) -> Result<(), wapc::errors::Error> {
        println!("-------VERIFYING ECDSA NATIVELY -----------");
        if TEST_MODE >= 2 {
            panic!("RANDOM VERIFYING IS NOT IMPLEMENTED IN NATIVE RUNTIME");
        }
        let yellow = Style::new().yellow();
        let magenta = Style::new().magenta();
        let mut right_messages: Vec<Answer> = right_messages.into_iter().collect();
        let mut r_answers: usize = 0;
        for _i in 0..MESSAGES_NUMBER {
            let mut encoded_signed_msg = recv_sig_msg.recv().unwrap();
            let r_msg_struct = right_messages.pop().unwrap();
            let _testmessage = r_msg_struct.msg;
            encoded_signed_msg.truncate(r_msg_struct.e_len); //adjust msg len
            let mut encoded_vkey = recv_ver_key.recv().unwrap();
            encoded_vkey.truncate(SIGN_SIZE);
            println!(
                "[{}]\nsigned message is [{}]\nver key is {}\nmessage:{}",
                _i,
                yellow.apply_to(&encoded_signed_msg),
                magenta.apply_to(hex::encode(&encoded_vkey)),
                &_testmessage
            );

            let Ok(restored_signed_message) = hex::decode(&encoded_signed_msg) else {
                return Err(wapc::errors::Error::InitFailed("bad encoding".to_string()));
            };

            let Ok(restored_signed_message) = Signature::from_der(&restored_signed_message) else {
                return Err(wapc::errors::Error::InitFailed("failed signatured msg recovery".to_string()));
            };
            let Ok(ver_key ) = VerifyingKey::from_sec1_bytes(&encoded_vkey) else {
                return Err(wapc::errors::Error::InitFailed("failed ver key recovery".to_string()));
            };
            if ver_key
                .verify(_testmessage.as_bytes(), &restored_signed_message)
                .is_ok()
            {
                r_answers += 1;
            } else {
                return Err(wapc::errors::Error::General(
                    "failed ver key recovery".to_string(),
                ));
            }
        }
        println!(
            "------------>>TOTAL MESSAGES:{}, VALID: {}, USED METHOD: [ECDSA]<<--------------------",
            MESSAGES_NUMBER, r_answers
        );
        Ok(())
    }
}
