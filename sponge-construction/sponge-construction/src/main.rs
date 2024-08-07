struct SpongeConstruction {
    state: Vec<u8>,
    rate: usize,
    capacity: usize,
}

impl SpongeConstruction {
    pub fn new(rate: usize, capacity: usize) -> Self {
        let state_size = rate + capacity;

        SpongeConstruction {
            rate: rate,
            capacity: capacity,
            state: vec![0; state_size],
        }
    }

    pub fn absorb(&mut self, message: &[u8]) {
        let mut padded_message = message.to_vec();
        let padding_length = (self.rate - (message.len() % self.rate)) % self.rate;

        // Pad divided message before dividing into r bit blocks
        if padding_length > 0 {
            padded_message.extend(vec![0u8; padding_length]);
        }

        // Divide into r bit blocks
        let m_bit_blocks = Vec::new();
        for chunk in padded_message.chunks(self.rate) {
            for (i, &byte) in chunk.iter().enumerate() {
                self.state[i] ^= byte;
            }
            self.permutation();
        }
    }

    pub fn squeeze(&mut self, output_length: usize) -> Vec<u8> {
        let mut output = Vec::with_capacity(output_length);

        while output.len() < output_length {
            let remaining = output_length - output.len();
            let to_copy = std::cmp::min(remaining, self.rate);

            output.extend_from_slice(&self.state[..to_copy]);

            if output.len() < output_length {
                self.permutation();
            }
        }

        output
    }

    // Permutation through bitwise rotation
    fn permutation(&mut self) {
        for byte in self.state.iter_mut() {
            *byte = byte.rotate_left(3) ^ 0x1B;
        }
    }

    // Permutation through byte substitution
    // fn permutation(&mut self) {
    //     for byte in self.state.iter_mut() {
    //         *byte = (*byte).wrapping_mul(167).wrapping_add(13);
    //     }
    // }
}

fn main() {
    // Example usage
    let mut sponge = SpongeConstruction::new(16, 16);
    sponge.absorb(b"Hello, world!");
    let output = sponge.squeeze(32);
    println!("Output: {:?}", output);
}
