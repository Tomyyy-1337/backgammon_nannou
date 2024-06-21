struct dice {
    value1: u8,
    value2: u8,
}

impl dice {
    fn roll(&mut self) {
        self.value1 = rand::random::<u8>() % 6 + 1;
        self.value2 = rand::random::<u8>() % 6 + 1;
    }

    fn get_distinct_values(&self) -> Vec<u8> {
        if self.value1 == self.value2 {
            vec![self.value1]
        } else {
            vec![self.value1, self.value2]
        }
    }

    fn remaining_dice(&self, value: u8) -> Vec<u8> {
        if self.value1 == self.value2 {
            return vec![value; 3];
        } 
        if self.value1 == value {
            vec![self.value2]
        } else {
            vec![self.value1]
        }
    }

    fn is_double(&self) -> bool {
        self.value1 == self.value2
    }

}