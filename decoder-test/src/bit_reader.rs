pub enum BitOrder {
    LSB,
    MSB
}

pub struct BitReader {
    i: u8,
    order: BitOrder, 
    amount: u8,
    value: u64
}

impl BitReader {
    pub fn new(amount: u8, order: BitOrder) -> Self {
        assert!(amount <= 64 && amount > 0, "i must be between 1 and 64");
        BitReader {amount, i: 0, value: 0, order}
    }

    pub fn get_value(&self) -> Option<u64> {
        if self.i > 0 {
            return Some(self.value);
        }
        None
    }

    // returns true if the reader is finished
    pub fn read_bit(&self, bit: bool) -> bool
    {
        if self.i >= self.amount {
            return true
        }

        let shift = match self.order {
            BitOrder::LSB => self.i,
            BitOrder::MSB => self.amount - self.i
        };

        self.value |= (bit as u64) << shift;
        false
    }
 
    pub fn is_finished(&self) {
        self.i == self.amount
    }
}