pub mod shmem_impl {

    use libshmem::datastructs::*;
    use rnglib::{Language, RNG};
    use serde::{Deserialize, Serialize};
    use serde_with::serde_as;
    use serde_with::Bytes;
    use shared_memory::*;
    use shared_memory::*;
    use std::mem;
    use std::path::Path;
    pub fn read_shmem(n_msg: u32) {
        let root_path = project_root::get_project_root().unwrap();
        let shmem_flink = Path::new(&root_path).join("server").join("file1");

        let sizeofstruct = mem::size_of::<InterData<512>>();
        let memsize: usize = 1048576;
        let shmem = ShmemConf::new()
            .size(memsize)
            .flink(&shmem_flink)
            .open()
            .unwrap();

        let shm_ptr_beg = shmem.as_ptr();
        let mut ptr_cpy = shm_ptr_beg.clone();
        unsafe {
            while std::ptr::read_volatile(ptr_cpy) != 1 {
                println!("not ready");
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
        let mut n_msg_t: u32 = 0;
        unsafe {
            ptr_cpy = ptr_cpy.add(1); // add one byte
            let readed_n_msg_bytes: [u8; 4] = std::ptr::read(ptr_cpy as *const _);
            n_msg_t = u32::from_ne_bytes(readed_n_msg_bytes);
            println!("n msg: {}", n_msg_t); // transferred n msg
            ptr_cpy = ptr_cpy.add(mem::size_of::<u32>()); // add offset
        }

        //         let mut buffer: Vec<u8> = Vec::with_capacity(4096);
        //         let mut buffer_ptr = buffer.as_ptr();
        let mut buf_struct = InterData::new(); // dummy
        let sizeofstruct = bincode::serialized_size(&buf_struct).unwrap() as usize;
        unsafe {
            for _i in 0..n_msg {
                let data = InterData::deserialize(ptr_cpy, sizeofstruct);
                println!(" addr: {:?} data is {:?}", ptr_cpy, data);
                let data1 = std::str::from_utf8(&data.bytes1).unwrap();
                println!("---->>>>{}", data1);
                ptr_cpy = ptr_cpy.add(sizeofstruct);
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    }
}
