pub struct Pari {
    channel: u32,
    bus: u32,
    value: u32,
}

impl Pari {
    pub fn new(channel: u32, bus: u32, value: u32) -> Self {
        Self { channel, bus, value }
    }
    
    pub fn to_bytes(&self) -> [u8; 12] {
        let mut buf = [0u8; 12];
        
        buf[0 .. 3].copy_from_slice(&self.channel.to_le_bytes());
        buf[4 .. 7].copy_from_slice(&self.bus.to_le_bytes());
        buf[8 .. 11].copy_from_slice(&self.value.to_le_bytes());
        
        return buf;
    }
}