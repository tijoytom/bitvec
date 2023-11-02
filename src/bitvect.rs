pub trait BitVec {
    fn is_set(&mut self, index: usize) -> bool;
    fn set(&mut self, index: usize); 
    fn unset(&mut self, index: usize);
}

impl BitVec for Vec<u8> {
    fn is_set(&mut self, index: usize) -> bool {
        
        let slot = index >> 3; // shift out last 3 bits. same as int(index/8)
        return slot < self.len() &&  self[slot] & (1  << (index  as u8 & 7)) > 0;
    }

    fn set(&mut self, index:usize) {
        let slot = index >> 3;
        while slot >= self.len(){
            self.push(0);
        }
        self[slot] = self[slot] | (1 << (index as u8 & 7));
    }

    fn unset(&mut self, index: usize) {
        let slot = index >> 3;
        self[slot] = self[slot] & !(1 << (index as u8 & 7));
    }
}