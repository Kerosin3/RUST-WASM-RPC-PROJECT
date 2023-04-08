pub mod shmem_impl {

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

    pub fn read_shmem(n_msg: u32) {
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
        unsafe {
            for _i in 0..n_msg {
                let data = InterData::deserialize(ptr_cpy, sizeofstruct);
                //                 println!(" addr: {:?} data is {:?}", ptr_cpy, data);
                let data1 = std::str::from_utf8(&data.bytes1).unwrap();
                let data2 = std::str::from_utf8(&data.bytes2).unwrap();
                info!("readed key [{}], value [{}]", data1, data2);
                ptr_cpy = ptr_cpy.add(sizeofstruct);
            }
        }
        warn!("read complete!");
        unsafe {
            *shmem.as_ptr() = 0;
        }
        warn!("clear write ready flag");
    }
}
