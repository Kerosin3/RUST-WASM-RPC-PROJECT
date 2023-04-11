pub mod implement {

    use k256::schnorr::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    };

    use libinteronnect::serdes::*;
    fn verify_message_natively(
        encoded_signed_msg: &String,
        encoded_vkey: &Vec<u8>,
        _testmessage: &String,
    ) -> Result<(), StatusFromWasm> {
        let Ok(restored_signed_message) = hex::decode(&encoded_signed_msg) else {
                return Err(StatusFromWasm::Error);
            };
        let Ok(restored_signed_message) = Signature::try_from(&restored_signed_message[..]) else {
                return Err(StatusFromWasm::Error);
            };
        let Ok(ver_key ) = VerifyingKey::from_bytes(&encoded_vkey) else {
                return Err(StatusFromWasm::Error);
            };

        if ver_key
            .verify(_testmessage.as_bytes(), &restored_signed_message)
            .is_ok()
        {
            return Ok(());
        } else {
            return Err(StatusFromWasm::NotValid);
        }
    }
}
