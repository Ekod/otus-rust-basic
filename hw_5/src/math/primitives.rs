#[derive(Default)]
pub struct SignedCounter {
    counter: usize,
}

impl SignedCounter {
    pub fn next_counter(&mut self) -> usize {
        self.counter += 1;

        self.counter
    }

    pub fn prev_counter(&mut self) -> usize {
        self.counter = self.counter.saturating_sub(1);

        self.counter
    }

    pub fn get_counter(&self) -> usize {
        self.counter
    }
}

#[derive(Default)]
pub struct UnsignedCounter {
    counter: usize,
}

impl UnsignedCounter {
    pub fn next_counter(&mut self) -> usize {
        self.counter += 1;

        self.counter
    }

    pub fn get_counter(&self) -> usize {
        self.counter
    }
}
