pub mod shmem_impl {

    use crate::wasm_processor::implement::*;
    use crossbeam_channel::unbounded;
    use crossbeam_queue::ArrayQueue;
    use libshmem::datastructs::*;
    use log::*;
    use rnglib::{Language, RNG};
    use serde::{Deserialize, Serialize};
    use serde_with::serde_as;
    use serde_with::Bytes;
    use shared_memory::*;
    use shared_memory::*;
    use std::mem;
    use std::path::Path;
    use std::thread;

    pub fn read_shmem(n_msg: u32, recv_right_msg: Vec<String>) {
        let root_path = project_root::get_project_root().unwrap();
        let shmem_flink = Path::new(&root_path).join("server").join(MEMFILE);
        info!("getted connected to shared memory file");
        let memsize: usize = MEMSIZE;
        let shmem = ShmemConf::new()
            .size(memsize)
            .flink(&shmem_flink)
            .open()
            .unwrap();

        let shm_ptr_beg = shmem.as_ptr();
        let mut ptr_cpy = shm_ptr_beg;
        unsafe {
            while std::ptr::read_volatile(ptr_cpy) != 1 {
                info!("waiting shared memory file write to complite");
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
        unsafe {
            ptr_cpy = ptr_cpy.add(1); // add one byte
            let readed_n_msg_bytes: [u8; 4] = std::ptr::read(ptr_cpy as *const _);
            let n_msg_t = u32::from_ne_bytes(readed_n_msg_bytes);
            info!("written message number:[{}]", n_msg_t);
            ptr_cpy = ptr_cpy.add(mem::size_of::<u32>()); // add offset
        }

        let buf_struct = InterData::new(); // dummy
        let sizeofstruct = bincode::serialized_size(&buf_struct).unwrap() as usize;
        //-------------------------------------------------------------------
        //-------------------------------------------------------------------
        let (sender_signed_msg, receiver_signed_msg) = unbounded();
        let (sender_ver_key, receiver_ver_key) = unbounded();
        let recv1: crossbeam_channel::Receiver<String> = receiver_signed_msg;
        let recv_val = receiver_ver_key;
        let handler = thread::spawn(move || {
            process_in_wasm(recv1, recv_val, recv_right_msg).unwrap();
        });
        //-------------------------------------------------------------------
        unsafe {
            for _i in 0..n_msg {
                let data = InterData::deserialize(ptr_cpy, sizeofstruct);
                let data1 = std::str::from_utf8(&data.bytes1).unwrap();
                let data2: Vec<u8> = data.bytes2.to_vec();
                //                 info!("readed key [{}], value [{}]", data1, data2);
                sender_signed_msg.send(data1.to_owned()).unwrap(); // sending to wasm module
                sender_ver_key.send(data2.to_owned()).unwrap(); // sending to wasm module
                ptr_cpy = ptr_cpy.add(sizeofstruct);
            }
        }
        warn!("read complete!");
        unsafe {
            let mut beg_m = shmem.as_ptr();
            *beg_m = 0; //flag
            beg_m = beg_m.add(1); // added one byte to pointer
            let n_msg_as_bytes: [u8; mem::size_of::<u32>()] = 0_u32.to_ne_bytes();
            std::ptr::copy(n_msg_as_bytes.as_ptr(), beg_m, mem::size_of::<u32>());
        }
        warn!("clear write ready flag");
        handler.join().unwrap();
    }
}
