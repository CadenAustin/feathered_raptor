static K_MIN: u32 = 1024;
static G_MAX: u32 = 32;

pub struct ObjectTransmissionInformation {
    transfer_length: u64,
    encoding_symbol_length: u16,
    number_of_source_blocks: u16,
    number_of_sub_blocks: u8,
    allignment: u8,
}

impl ObjectTransmissionInformation {
    pub fn new() -> Self {
        Self {
            transfer_length: 0,
            encoding_symbol_length: 0,
            number_of_source_blocks: 0,
            number_of_sub_blocks: 0,
            allignment: 0,
        }
    }
}