use std::io;

#[derive(Clone)]
pub struct Computer {
    noun: i32,
    verb: i32,
    current_pos: usize,
    params: Option<Vec<usize>>,
    pub start_input: Vec<i32>,
    pub computed_values: Vec<i32>,
    current_values: Values,
    pub outputs: Vec<i32>,
}

#[derive(Clone)]
pub struct Values {
    values: Option<Vec<i32>>,
    address: usize,
}

impl ComputerActions for Computer {
    fn new(noun: i32, verb: i32, start_input: Vec<i32>) -> Computer {
        Computer {
            noun,
            verb,
            params: None,
            current_pos: 0,
            computed_values: start_input.clone(),
            start_input,
            current_values: Values {
                values: None,
                address: 0,
            },
            outputs: Vec::new(),
        }
    }

    // To implement param mode, each value needs to be computed (grabbed the address or used the value)
    fn run(&mut self) {
        let opcode = self.format_instructions(self.computed_values[self.current_pos]);
        if opcode == 99 {
            return;
        } else {
            match opcode {
                1 => {
                    self.get_values(2);
                    let computed_values = self.current_values.values.as_ref().unwrap();
                    self.computed_values[self.current_values.address] =
                        computed_values[0] + computed_values[1];
                    self.current_pos += 4;
                }
                2 => {
                    self.get_values(2);
                    let computed_values = self.current_values.values.as_ref().unwrap();
                    self.computed_values[self.current_values.address] =
                        computed_values[0] * computed_values[1];
                    self.current_pos += 4;
                }
                3 => {
                    print!("Please enter your input\n");
                    self.get_values(0);
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .ok()
                        .expect("Couldn't read line");

                    self.computed_values[self.current_values.address] =
                        input.trim().parse::<i32>().unwrap();
                    self.current_pos += 2;
                }
                4 => {
                    if self.params.is_some() {
                        let output_value = self.computed_values[self.current_pos + 1];
                        println!("{}", output_value);
                        self.outputs.push(output_value)
                    } else {
                        self.get_values(0);
                        let output_value = self.computed_values[self.current_values.address];
                        println!("{}", output_value);
                        self.outputs.push(output_value);
                    }
                    self.current_pos += 2;
                }
                _ => return,
            }
            self.current_values.values = None;
            self.params = None;
            self.run();
        }
    }

    fn get_values(&mut self, num_of_values: usize) {
        let result_address = self.computed_values[self.current_pos + num_of_values + 1] as usize;
        self.current_values.address = result_address;

        if let Some(params_vec) = self.params.clone() {
            let mut current_values = Vec::new();
            let combined_map: Vec<(usize, usize)> = (1..=num_of_values)
                .collect::<Vec<usize>>()
                .into_iter()
                .zip(params_vec.into_iter())
                .collect();

            for (i, param) in combined_map.iter() {
                if *param == 1 {
                    current_values.push(self.computed_values[self.current_pos + i])
                } else {
                    current_values.push(
                        self.computed_values[self.computed_values[self.current_pos + i] as usize],
                    )
                };
            }
            self.current_values.values = Some(current_values);
        } else if num_of_values > 0 {
            let current_values = (1..=num_of_values)
                .into_iter()
                .map(|i| self.computed_values[self.computed_values[self.current_pos + i] as usize])
                .collect();
            self.current_values.values = Some(current_values);
        }
    }

    fn format_instructions(&mut self, instruction: i32) -> i32 {
        if instruction.to_string().len() == 1 {
            instruction
        } else {
            let mut instruction_vec = instruction
                .to_string()
                .chars()
                .map(|d| d.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>();

            let vec_len = instruction_vec.len();
            let opcode_vec: Vec<usize> = instruction_vec.drain(vec_len - 2..vec_len).collect();
            let opcode = opcode_vec[0].to_string() + &opcode_vec[1].to_string();
            instruction_vec.reverse();
            if instruction_vec.len() != 4 {
                let additional_zeros = 4 - instruction_vec.len();
                let mut zeros_vec: Vec<usize> = vec![0; additional_zeros];
                instruction_vec.append(&mut zeros_vec);
                self.params = Some(instruction_vec);
            } else {
                self.params = Some(instruction_vec);
            }

            opcode.parse::<i32>().unwrap()
        }
    }

    fn write(&mut self, value: i32, action: &str) {
        if action == "noun" {
            self.noun = value;
        } else if action == "verb" {
            self.verb = value;
        };
    }

    fn reset(&mut self, noun: i32, verb: i32) {
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
    fn new(noun: i32, verb: i32, start_input: Vec<i32>) -> Computer;
    fn write(&mut self, value: i32, action: &str);
    fn run(&mut self);
    fn reset(&mut self, noun: i32, verb: i32);
    fn restore_gravity_assist_program(&mut self);
    fn format_instructions(&mut self, instruction: i32) -> i32;
    fn get_values(&mut self, num_of_values: usize);
}
