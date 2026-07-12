pub type Meter = f32;

const DATA_SIZE: usize = 162;

pub struct QUSt {
    input: Vec<Meter>,    
    daw: Vec<Meter>,
    output: Vec<Meter>,
    opt1: u8,
    opt2: u8,
    version: String,
}

impl QUSt {
    pub fn new(input: Vec<Meter>, daw: Vec<Meter>, output: Vec<Meter>, opt1: u8, opt2: u8, version: String) -> Self {
        assert!(input.len() == 16);
        assert!(daw.len() == 10);
        assert!(output.len() == 10);
        Self {
            input,
            daw,
            output,
            opt1: opt1,
            opt2: opt2,
            version: version,
        }
    }
    
    pub fn placeholder() -> Vec<u8> {
        vec![0u8; DATA_SIZE]
    }
    
    pub fn to_bytes(self) -> [u8; DATA_SIZE] {
        let mut buf = [0u8; DATA_SIZE];
        
        let mut offset: usize = 0;
        for ch in self.input {
            buf[offset .. offset + 4].copy_from_slice(&ch.to_le_bytes());
            offset += 4;
        }
        for ch in self.daw {
            buf[offset .. offset + 4].copy_from_slice(&ch.to_le_bytes());
            offset += 4;
        }
        for ch in self.output {
            buf[offset .. offset + 4].copy_from_slice(&ch.to_le_bytes());
            offset += 4;
        }
        offset = buf.len() - 18;
        buf[offset] = self.opt1;
        buf[offset + 1] = self.opt2;
        buf[offset + 2 .. offset + 14].copy_from_slice(self.version.to_string().as_bytes());
        
        buf
    }
    
    pub fn parse(data: &[u8], size: u32) -> Option<Self> {
        if size < DATA_SIZE as u32 {
            return None;
        }
        let mut input : Vec<Meter> = Vec::with_capacity(16);
        let mut daw: Vec<Meter> = Vec::with_capacity(10);
        let mut output: Vec<Meter> = Vec::with_capacity(10);
        for ch in 0 .. 16 {
            let offset = ch * 4;
            input.push(f32::from_le_bytes(data[offset .. offset + 4].try_into().unwrap()));
        }
        for ch in 16 .. 26 {
            let offset = ch * 4;
            daw.push(f32::from_le_bytes(data[offset .. offset + 4].try_into().unwrap()));
        }
        for ch in 26 .. 36 {
            let offset = ch * 4;
            output.push(f32::from_le_bytes(data[offset .. offset + 4].try_into().unwrap()));
        }
        let opt1 = data[DATA_SIZE - 18];
        let opt2 = data[DATA_SIZE - 17];
        let version = String::from_utf8(data[DATA_SIZE - 16 .. DATA_SIZE - 4].to_vec()).unwrap();
        
        Some(Self::new(
            input,
            daw,
            output,
            opt1,
            opt2,
            version,
        ))
    }
}

#[test]
fn test_qust() {
    let bytes = QUSt::new(
        [0f32; 16].to_vec(),
        [0f32; 10].to_vec(),
        [0f32; 10].to_vec(),
        1,
        4,
        "this.is.test".to_string(),
    ).to_bytes();
    let q = QUSt::parse(&bytes, bytes.len() as u32).unwrap();
    assert_eq!(q.opt1, 1);
    assert_eq!(q.opt2, 4);
    assert_eq!(q.version, "this.is.test")
}