pub const INTERRUPT_POLL_MILLISECONDS: u8 = 5;

pub const PACKET_SIZE: usize = 64;

<<<<<<< Updated upstream
// 7609
pub const MESSAGE_SIZE: usize = PACKET_SIZE - 7 + 128 * (PACKET_SIZE - 5);
=======
// TODO: find the actual minimum Dilithium sizes
pub const MESSAGE_SIZE: usize = (if cfg!(feature = "backend-dilithium5") {
    20000
} else if cfg!(feature = "backend-dilithium3") {
    15000
} else if cfg!(feature = "backend-dilithium2") {
    10000
} else {
    3072
});
>>>>>>> Stashed changes
