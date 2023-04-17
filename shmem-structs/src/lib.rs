#![allow(clippy::new_without_default)]
pub mod datastructs {
    //##############################################################3
    //##############################################################3
    //##############################################################3
    pub const STRING_SIZE: usize = 512;
    pub const MESSAGES_NUMBER: u32 = 2;
    pub const MEMFILE: &str = "memshare";
    pub const MEMSIZE: usize = 52428800;
    pub const SIGN_SIZE: usize = 33; // max!
                                     //##############################################################3
                                     //##############################################################3
                                     //##############################################################3
    extern crate serde;
    extern crate serde_with;
    use self::serde::{Deserialize, Serialize};
    use self::serde_with::serde_as;
    use self::serde_with::Bytes;
    #[serde_as]
    #[derive(Serialize, Deserialize, Clone, Debug)]
    #[repr(C, packed)]
    pub struct InterData<const N: usize> {
        pub serial: u32,
        #[serde_as(as = "Bytes")]
        pub bytes1: [u8; N],
        #[serde_as(as = "Bytes")]
        pub bytes2: [u8; N],
    }
    impl InterData<STRING_SIZE> {
        pub fn new() -> Self {
            Self {
                serial: (0),
                bytes1: [0_u8; STRING_SIZE],
                bytes2: [0_u8; STRING_SIZE],
            }
        }
        pub fn increment_serial(&mut self, seriall: u32) {
            self.serial = seriall;
        }
        pub fn serialize(&self) -> Vec<u8> {
            bincode::serialize(self).unwrap()
        }

        pub fn deserialize(src: *const u8, _size: usize) -> Self {
            unsafe {
                let readed_n_msg_bytes: [u8; 1048] = std::ptr::read(src as *const _);
                let struct1: InterData<512> = bincode::deserialize(&readed_n_msg_bytes).unwrap();
                struct1
            }
        }
        pub fn assign_bytes0(&mut self, t_data: (*mut u8, usize, usize)) {
            if t_data.1 > STRING_SIZE {
                panic!()
            }
            unsafe {
                // copy t_data.1 bytes to bytes 1 field
                std::ptr::copy(t_data.0, self.bytes1.as_mut_ptr(), t_data.1);
            }
        }
        pub fn assign_bytes1(&mut self, t_data: (*mut u8, usize, usize)) {
            if t_data.1 > STRING_SIZE {
                panic!()
            }
            unsafe {
                // copy t_data.1 bytes to bytes 1 field
                std::ptr::copy(t_data.0, self.bytes2.as_mut_ptr(), t_data.1);
            }
        }
    }
}
