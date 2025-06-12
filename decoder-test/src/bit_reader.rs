enum BitOrder {
    LSB,
    MSB
}

struct BitReader {
    i: u8,
    order: BitOrder, 
    amount: u8, 
    value: u64
}

impl BitReader {
    fn new(amount: u8, order: BitOrder) -> Self {
        assert!(i <= 64 && i > 0, "i must be between 1 and 64");
        BitReader {amount, i: 0, value: 0}
    }

    pub fn get_value(&self) -> Option<u64> {
        if i > 0 {
            return Some(self.value);
        }
        None
    }

    // returns true if the reader is finished
    pub fn read_bit(&self, bit: bool) -> bool
    {
        if self.i >= amount {
            return true
        }

        let shift = match self.order {
            BitOrder::LSB => self.i,
            BitOrder::MSB => self.amount - self.i
        };

        self.value |= (bit as u64) << i;
        false
    }
 
    pub fn is_finished(&self) {
        i == amount
    }
}