pub mod memoryprocessor {
    use hex::encode;
    use libshmem::datastructs::*;
    use rnglib::{Language, RNG};
    use serde::{Deserialize, Serialize};
    use serde_with::serde_as;
    use serde_with::Bytes;
    use shared_memory::*;
    use shared_memory::*;
    use std::mem;
    use std::path::Path;
    // WRITE!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    pub fn fill_sh_memory(key: String, value: Vec<u8>, serial_msg: u32) {
        let root_path = project_root::get_project_root().unwrap();
        let shmem_flink = Path::new(&root_path).join(MEMFILE);
        let memsize: usize = MEMSIZE;
        let shmem = ShmemConf::new()
            .size(memsize)
            .flink(shmem_flink)
            .open()
            .unwrap();
        // opened shmem
        tracing::info!("connected to memory file");
        let ptr_shm_beg_original = shmem.as_ptr(); // beginning of memory
        let mut raw_ptr = shmem.as_ptr();
        let mut ptr_cpy = shmem.as_ptr();
        let mut buf_struct = InterData::new(); // assign struct
        let sizeofstruct = bincode::serialized_size(&buf_struct).unwrap() as usize; // get size
        unsafe {
            *ptr_shm_beg_original = 0; // write not ready
            ptr_cpy = ptr_cpy.add(1); // add one byte
            let readed_n_msg_bytes: [u8; 4] = std::ptr::read(ptr_cpy as *const _);
            let n_msg_t = u32::from_ne_bytes(readed_n_msg_bytes);
            tracing::info!("currently written [{}] transactions", n_msg_t);
            ptr_cpy = ptr_cpy.add(mem::size_of::<u32>()); // add offset
            ptr_cpy = ptr_cpy.add(sizeofstruct * n_msg_t as usize); // add serial offset
        }
        tracing::info!(
            "processing key {}, value {}, serial: {}",
            key,
            encode(&value),
            serial_msg
        );
        unsafe {
            let (ptr, len, cap) = key.into_raw_parts();
            let (ptr1, len1, cap1) = value.into_raw_parts();
            buf_struct.increment_serial(serial_msg);
            buf_struct.assign_bytes0((ptr, len, cap));
            buf_struct.assign_bytes1((ptr1, len1, cap1));
            let bytes = buf_struct.serialize();
            std::ptr::copy(bytes.as_ptr(), ptr_cpy, sizeofstruct);
        }
        // write add one to written
        let next_num: u32 = serial_msg + 1;
        unsafe {
            raw_ptr = raw_ptr.add(1); // added one byte to pointer
            let n_msg_as_bytes: [u8; mem::size_of::<u32>()] = next_num.to_ne_bytes();
            std::ptr::copy(n_msg_as_bytes.as_ptr(), raw_ptr, mem::size_of::<u32>()); //write n msg as
            raw_ptr.add(mem::size_of::<u32>()); // add 4 bytes
        }
        tracing::info!("written a transaction");
    }
    pub fn write_complite() {
        let root_path = project_root::get_project_root().unwrap();
        let shmem_flink = Path::new(&root_path).join(MEMFILE);
        let memsize: usize = MEMSIZE;
        tracing::warn!("write ready!");
        let shmem = ShmemConf::new()
            .size(memsize)
            .flink(shmem_flink)
            .open()
            .unwrap();
        // opened shmem
        let ptr_shm_beg_original = shmem.as_ptr(); // beginning of memory
        unsafe {
            *ptr_shm_beg_original = 1; // write ready
        }
    }
}
