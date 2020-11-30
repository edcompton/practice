#[derive(Clone)]
pub struct Computer {
    noun: usize,
    verb: usize,
    current_pos: usize,
    pub start_input: Vec<usize>,
    pub computed_values: Vec<usize>,
}

impl ComputerActions for Computer {
    fn new(noun: usize, verb: usize, start_input: Vec<usize>) -> Computer {
        let mut computer = Computer {
            noun,
            verb,
            current_pos: 0,
            computed_values: start_input.clone(),
            start_input,
        };
        computer.restore_gravity_assist_program();
        computer
    }

    fn run(&mut self) {
        let opcode = self.computed_values[self.current_pos];
        if opcode == 99 {
            return;
        } else {
            let first_position_address = self.computed_values[self.current_pos + 1];
            let second_position_address = self.computed_values[self.current_pos + 2];
            let result_address = self.computed_values[self.current_pos + 3];
            match opcode {
                1 => {
                    self.computed_values[result_address] = self.computed_values
                        [first_position_address]
                        + self.computed_values[second_position_address]
                }
                2 => {
                    self.computed_values[result_address] = self.computed_values
                        [first_position_address]
                        * self.computed_values[second_position_address]
                }
                _ => return,
            }
            self.current_pos = self.current_pos + 4;
            self.run();
        }
    }

    fn write(&mut self, value: usize, action: &str) {
        if action == "noun" {
            self.noun = value;
        } else if action == "verb" {
            self.verb = value;
        };
    }

    fn reset(&mut self, noun: usize, verb: usize) {
        self.computed_values = self.start_input.clone();
        self.current_pos = 0;
        self.noun = noun;
        self.verb = verb;
        self.restore_gravity_assist_program();
    }

    fn restore_gravity_assist_program(&mut self) {
        self.computed_values[1] = self.noun;
        self.computed_values[2] = self.verb;
    }
}

pub trait ComputerActions {
    fn new(noun: usize, verb: usize, start_input: Vec<usize>) -> Computer;
    fn write(&mut self, value: usize, action: &str);
    fn run(&mut self);
    fn reset(&mut self, noun: usize, verb: usize);
    fn restore_gravity_assist_program(&mut self);
}
