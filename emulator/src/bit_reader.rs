pub enum BitOrder {
    LSB,
    MSB
}

pub struct BitReader {
    bits_read: u8,
    order: BitOrder, 
    amount: u8,
    value: u64
}

impl BitReader {
    pub fn new(amount: u8, order: BitOrder) -> Self {
        assert!(amount <= 64 && amount > 0, "amount must be between 1 and 64");
        BitReader {amount, bits_read: 0, value: 0, order}
    }

    pub fn lsb(amount: u8) -> Self {
        BitReader::new(amount, BitOrder::LSB)
    }

    pub fn msb(amount: u8) -> Self {
        BitReader::new(amount, BitOrder::MSB)
    }

    pub fn get_value(&self) -> Option<u64> {
        if self.bits_read > 0 {
            return Some(self.value);
        }
        None
    }

    // returns true if the reader is finished
    pub fn read_bit(&mut self, bit: bool) -> bool
    {
        if self.is_finished() {
            return true
        }

        let shift = match self.order {
            BitOrder::LSB => self.bits_read,
            BitOrder::MSB => self.amount - self.bits_read
        };

        self.value |= (bit as u64) << shift;
        self.bits_read += 1;

        self.is_finished()
    }
 
    pub fn is_finished(&self) -> bool {
        self.bits_read >= self.amount
    }
}