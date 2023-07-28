use super::utils::BitCounter;

pub struct Mask<const WORD_COUNT : usize>{
    pub words : [u64; WORD_COUNT],
    pub size : u32,
    pub word_count : u32,
}

impl<const WORD_COUNT : usize> Mask<WORD_COUNT> {

    pub fn new() -> Self {
        Self {
            word_count : WORD_COUNT as u32,
            size: (WORD_COUNT as u32) << 6,
            words : [0u64; WORD_COUNT]
        }
    }

    pub fn memUsage(&self) -> u32 {
        return self.word_count * std::mem::size_of::<u64> as u32;
    }

    pub fn countOn(&self) -> u32{
        let mut sum : u32 = 0;
        for i in 0..WORD_COUNT {
            sum += BitCounter::countOn(self.words[i]) as u32;
        }
        return sum;
    }

    pub fn countOff(&self) -> u32 {
        return self.size - self.countOn();
    }

    pub fn setOn(&mut self, n : usize) {
        self.words[n >> 6] |= 1 << (n & 63);
    }

    pub fn setOff(&mut self, n : usize) {
        self.words[ n >> 6] &= !(1 << (n & 63));
    }

    pub fn set(&mut self, n : usize, on : bool) {
        if on {self.setOn(n) } else { self.setOff(n) };
    }

    pub fn setAll(&mut self, on : bool) {
        let state : u64 = if on {!0} else {0};
        for i in 0..WORD_COUNT {
            self.words[i] = state;
        }
    }

    pub fn setAllOn(&mut self) {
        self.setAll(true);
    }

    pub fn setAllOff(&mut self) {
        self.setAll(false);
    }

    pub fn toggle(&mut self, n : usize) {
        self.words[n >> 6] ^= 1 << (n & 63);
    }

    pub fn toggleAll(&mut self) {
        for i in 0..WORD_COUNT {
            self.words[i] = !self.words[i];
        }
    }

    pub fn isOn(&self, n : usize) -> bool {
        return 0 != (self.words[n >> 6] & (1 << (n & 63)));
    }

    pub fn isOff(&self, n : usize) -> bool {
        return !self.isOn(n);
    }

    pub fn lowestOn(&self) -> u32 {
        let mut i = 0;
        let mut word : u64 = self.words[i];
        while i < WORD_COUNT && word == 0 { i += 1; word = self.words[i]; }
        return if i == WORD_COUNT { self.size } else { (i << 6) as u32 + BitCounter::lowestOn(word) as u32 }
    }

    pub fn lowestOff(&self) -> u32 {
        let mut i = 0;
        let mut word : u64 = self.words[i];
        while i < WORD_COUNT && !word == 0 { i += 1; word = self.words[i]; }
        if i == WORD_COUNT { self.size } else { (i << 6) as u32 + BitCounter::lowestOff(word) as u32 }
    }

    pub fn findNextOn(&self, start: usize) -> u32 {
        let mut n : usize = start >> 6;
        if n >= WORD_COUNT { return self.size; }
        let mut m : usize = n & 63;
        let mut word : u64 = self.words[n];
        if word & (1 << m) > 0 { return start as u32; }
        word &= !0 << m;
        n += 1;
        while word == 0 && n < WORD_COUNT {
            word = self.words[n];
            n += 1;
        };
        if word == 0 { self.size } else {(n << 6) as u32 + BitCounter::lowestOn(word) as u32 }
    }

}
