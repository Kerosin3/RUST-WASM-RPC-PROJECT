pub mod implement {

    use crate::{Answer, TEST_MODE};
    use k256::schnorr::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    };

    use console::Style;
    use libinteronnect::serdes::*;
    use libshmem::datastructs::MESSAGES_NUMBER;
    use libshmem::datastructs::*;
    use std::io::{Error, ErrorKind};
    use std::time::Instant;
    use tonic::codegen::http::header::TE;

    pub fn verify_message_natively_schoor(
        recv_sig_msg: crossbeam_channel::Receiver<String>,
        recv_ver_key: crossbeam_channel::Receiver<Vec<u8>>,
        right_messages: Vec<Answer>,
    ) -> Result<(), wapc::errors::Error> {
        println!("-------VERIFYING SCHOOR NATIVELY-----------");
        if TEST_MODE >= 2 {
            panic!("RANDOM VERIFYING IS NOT IMPLEMENTED IN NATIVE RUNTIME");
        }
        let yellow = Style::new().yellow();
        let magenta = Style::new().magenta();
        let mut right_messages: Vec<Answer> = right_messages.into_iter().collect();
        let mut r_answers: usize = 0;
        let now = Instant::now();
        for _i in 0..MESSAGES_NUMBER {
            let mut encoded_signed_msg = recv_sig_msg.recv().unwrap();
            let r_msg_struct = right_messages.pop().unwrap();
            let _testmessage = r_msg_struct.msg;
            encoded_signed_msg.truncate(r_msg_struct.e_len); //adjust msg len
            let mut encoded_vkey = recv_ver_key.recv().unwrap();
            encoded_vkey.truncate(SIGN_SIZE - 1);
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
            let Ok(restored_signed_message) = Signature::try_from(&restored_signed_message[..]) else {
                return Err(wapc::errors::Error::InitFailed("failed signatured msg recovery".to_string()));
            };
            let Ok(ver_key ) = VerifyingKey::from_bytes(&encoded_vkey) else {
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
        let elapsed = now.elapsed();
        println!(
            "------------>>TOTAL MESSAGES:{}, VALID: {}, USED METHOD: [SCHOOR], elapsed: {:.2?}<<--------------------",
            MESSAGES_NUMBER, r_answers, elapsed
        );
        Ok(())
    }
    #[allow(dead_code)]
    pub fn test_validity(
        encoded_vkey: &[u8],
        encoded_signed_msg: &str,
        _testmessage: &str,
    ) -> Result<(), std::io::Error> {
        println!("message is {}", _testmessage);
        //----restore signe msg (to Signature)
        let restored_signed_message = hex::decode(encoded_signed_msg).unwrap();
        let restored_signed_message = Signature::try_from(&restored_signed_message[..]).unwrap();
        // restore verification key
        let ver_key = VerifyingKey::from_bytes(encoded_vkey).unwrap();
        if ver_key
            .verify(_testmessage.as_bytes(), &restored_signed_message)
            .is_ok()
        {
            println!("PASSED!");
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "verification failed"))
        }
    }
}
