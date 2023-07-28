pub struct BitCounter;
impl BitCounter {
    const DeBruijnLow32 : [u8; 32] = [
        0, 1, 28, 2, 29, 14, 24, 3, 30, 22, 20, 15, 25, 17, 4, 8,
        31, 27, 13, 23, 21, 19, 16, 7, 26, 12, 18, 6, 11, 5, 10, 9
    ];

    const DeBruijnHigh32 : [u8; 32] = [
        0, 9, 1, 10, 13, 21, 2, 29, 11, 14, 16, 18, 22, 25, 3, 30,
        8, 12, 20, 28, 15, 17, 24, 7, 19, 27, 23, 6, 26, 5, 4, 31
    ];

    const DeBruijnLow64 : [u8; 64] = [
        0,   1,  2, 53,  3,  7, 54, 27, 4,  38, 41,  8, 34, 55, 48, 28,
        62,  5, 39, 46, 44, 42, 22,  9, 24, 35, 59, 56, 49, 18, 29, 11,
        63, 52,  6, 26, 37, 40, 33, 47, 61, 45, 43, 21, 23, 58, 17, 10,
        51, 25, 36, 32, 60, 20, 57, 16, 50, 31, 19, 15, 30, 14, 13, 12,
    ];

    /*
    fn countOn<u32>(v : u32) -> u8 {
        v = v - ((v >> 1) & 0x55555555u32);
        v = (v & 0x33333333u32) + ((v >> 2) & 0x33333333u32);
        return (((v + (v >> 4)) & 0xF0F0F0Fu32) * 0x1010101u32) >> 24 as u8;
    }*/

    pub fn countOn(u : u64) -> u8 {
        let mut v = u;
        v = v - ((v >> 1) & 0x5555555555555555u64);
        v = (v & 0x3333333333333333u64) + ((v >> 2) & 0x3333333333333333u64);
        v = (((v + (v >> 4)) & 0xF0F0F0F0F0F0F0Fu64) * 0x101010101010101u64) >> 56;
        v as u8
    }

    /*
    fn countOff<u32>(v : u32) -> u8 {
        BitCounter.contOn<u32>(!v)
    }*/

    pub fn countOff(v : u64) -> u8 {
        BitCounter::countOn(!v)
    }
    /*
    fn lowestOn(v : u32) -> u8 {
        let v = self.0 as u32;
        DeBruijnLow32[(v & (!v + 1) * 0x077CB531u32) >> 27]
    }*/

    pub fn lowestOn(v : u64) -> u8 {
        let index = ((v & (!v + 1)) * 0x022FDD63CC95386d as u64) >> 58;
        BitCounter::DeBruijnLow64[index as usize]
    }

    /*
    fn lowestOff(v : u32) -> u8 {
        BitCounter.lowestOn(!v)
    }*/

    pub fn lowestOff(v : u64) -> u8 {
        BitCounter::lowestOn(!v)
    }

    /*
    fn highestOn(v : u32) -> u8 {
        v |= v >> 1; // first round down to one less than a power of 2
        v |= v >> 2;
        v |= v >> 4;
        v |= v >> 8;
        v |= v >> 16;
        DeBruijnHighu32[(v * 0x07C4ACDDu32) >> 27]
    }*/

    

    
}