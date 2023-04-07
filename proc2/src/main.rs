// use ipc_channel::ipc::IpcReceiverSet;
// use ipc_channel::ipc::*;
// use ipc_channel::ipc;
use ipc_channel::ipc::{IpcReceiverSet, IpcSender, IpcSharedMemory};
fn main() {
    let (tx, rx) = ipc_channel::ipc::channel().unwrap();
    let data = [0x76, 0x69, 0x6d, 0x00];
    let shmem = IpcSharedMemory::from_bytes(&data);
    let rx_shmem = rx.recv().unwrap();
    assert_eq!(shmem, rx_shmem);
}
