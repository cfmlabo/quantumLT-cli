#[derive(Clone, Copy)]
pub struct Mixer {
    channel: u16,
    bus: u8,
    position: u8,
    value: f32,
}
impl Mixer {
    pub fn new(channel: u16, bus: u8, position: u8, value: f32) -> Self {
        Self {
            channel: channel,
            bus: bus,
            position: position,
            value: value,
        }
    }

    fn to_bytes(&self) -> [u8; 8] {
        let mut buf = [0u8; 8];
        
        buf[0..2].copy_from_slice(&self.channel.to_le_bytes());
        buf[2] = self.bus;
        buf[3] = self.position;
        buf[4..8].copy_from_slice(&self.value.to_le_bytes());
        
        return buf;
    }
}

const MAX_MIXSERS: usize = 60;
pub struct Mprm {
    mixers: Vec<Mixer>,
}

impl Mprm {
    pub fn new(mixers: Vec<Mixer>) -> Self {
        assert!(mixers.len() <= MAX_MIXSERS);
        Self { mixers: mixers }
    }

    pub fn to_bytes(&self) -> [u8; 488] {
        let mut buf = [0u8; 488];
        let mut offset: usize = 4;
        for m in &self.mixers {
            buf[offset..offset+8].copy_from_slice(&m.to_bytes());
            offset += 8;
        }
        buf[484..488].copy_from_slice(&((self.mixers.len() as u32).to_le_bytes()));
        
        return buf;
    }
}

#[test]
fn test_mprm() {
    let mut mixers: Vec<Mixer> = vec![];
    mixers.push(Mixer::new(0, 0, 0, -145f32));
    let mprm = Mprm::new(mixers);
    mprm.to_bytes();
}