use std::io;

#[derive(Clone)]
pub struct Computer {
    noun: i32,
    verb: i32,
    instruction_pointer: usize,
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
            instruction_pointer: 0,
            computed_values: start_input.clone(),
            start_input,
            current_values: Values {
                values: None,
                address: 0,
            },
            outputs: Vec::new(),
        }
    }

    fn write(&mut self, value: i32, action: &str) {
        if action == "noun" {
            self.noun = value;
        } else if action == "verb" {
            self.verb = value;
        };
    }

    // To implement param mode, each value needs to be computed (grabbed the address or used the value)
    fn run(&mut self) {
        let opcode = self.format_instructions(self.computed_values[self.instruction_pointer]);
        if opcode == 99 {
            return;
        } else {
            match opcode {
                1 => {
                    self.compute_values(2);
                    let computed_values = self.current_values.values.as_ref().unwrap();
                    self.computed_values[self.current_values.address] =
                        computed_values[0] + computed_values[1];
                    self.instruction_pointer += 4;
                }
                2 => {
                    self.compute_values(2);
                    let computed_values = self.current_values.values.as_ref().unwrap();
                    self.computed_values[self.current_values.address] =
                        computed_values[0] * computed_values[1];
                    self.instruction_pointer += 4;
                }
                // 3 - Take an input and store at address
                3 => {
                    print!("Please enter your input\n");
                    self.compute_values(0);
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .ok()
                        .expect("Couldn't read line");

                    self.computed_values[self.current_values.address] =
                        input.trim().parse::<i32>().unwrap();
                    self.instruction_pointer += 2;
                }
                // 4 - Print the value at address
                4 => {
                    if self.params.is_some() {
                        let output_value = self.computed_values[self.instruction_pointer + 1];
                        println!("{}", output_value);
                        self.outputs.push(output_value)
                    } else {
                        self.compute_values(0);
                        let output_value = self.computed_values[self.current_values.address];
                        println!("{}", output_value);
                        self.outputs.push(output_value);
                    }
                    self.instruction_pointer += 2;
                }
                // 5 - if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter.
                // Otherwise, it does nothing.
                5 => {
                    self.compute_values(2);
                    let computed_values = self.current_values.values.as_ref().unwrap();
                    if computed_values[0] != 0 {
                        self.instruction_pointer = computed_values[1] as usize;
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                // 6 - if the first parameter is zero, it sets the instruction pointer to the value from the second parameter.
                // Otherwise, it does nothing.
                6 => {
                    self.compute_values(2);
                    let computed_values = self.current_values.values.as_ref().unwrap();
                    if computed_values[0] == 0 {
                        self.instruction_pointer = computed_values[1] as usize;
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                // 7 - if the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter.
                // Otherwise, it stores 0.
                7 => {
                    self.compute_values(2);
                    let computed_values = self.current_values.values.as_ref().unwrap();
                    if computed_values[0] < computed_values[1] {
                        self.computed_values[self.current_values.address] = 1;
                    } else {
                        self.computed_values[self.current_values.address] = 0;
                    }
                    self.instruction_pointer += 4;
                }
                // 8 - if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter.
                // Otherwise, it stores 0.
                8 => {
                    self.compute_values(2);
                    let computed_values = self.current_values.values.as_ref().unwrap();
                    if computed_values[0] == computed_values[1] {
                        self.computed_values[self.current_values.address] = 1;
                    } else {
                        self.computed_values[self.current_values.address] = 0;
                    }
                    self.instruction_pointer += 4;
                }
                _ => return,
            }
            self.current_values.values = None;
            self.params = None;
            self.run();
        }
    }

    fn reset(&mut self, noun: i32, verb: i32) {
        self.computed_values = self.start_input.clone();
        self.instruction_pointer = 0;
        self.noun = noun;
        self.verb = verb;
        self.restore_gravity_assist_program();
    }

    fn restore_gravity_assist_program(&mut self) {
        self.computed_values[1] = self.noun;
        self.computed_values[2] = self.verb;
    }

    fn format_instructions(&mut self, instruction: i32) -> i32 {
        if instruction.to_string().len() == 1 {
            instruction
        } else {
            let mut instruction_vec = create_instruction_vec(&instruction);
            let opcode = get_opcode_from_instruction_vec(&mut instruction_vec);
            let formatted_instruction_vec = format_remaining_params(instruction_vec);

            self.params = Some(formatted_instruction_vec);
            opcode
        }
    }

    fn compute_values(&mut self, num_of_values: usize) {
        let result_address =
            self.computed_values[self.instruction_pointer + num_of_values + 1] as usize;
        self.current_values.address = result_address;
        let mut current_values = Vec::new();

        if let Some(params_vec) = self.params.clone() {
            self.compute_values_from_param_modes(params_vec, &num_of_values, &mut current_values);

        //     No params, treat all instructions as indices
        } else if num_of_values > 0 {
            self.compute_values_from_positions(&num_of_values, &mut current_values);
        }

        self.current_values.values = Some(current_values);
    }

    fn compute_values_from_param_modes(
        &mut self,
        params_vec: Vec<usize>,
        num_of_values: &usize,
        current_values: &mut Vec<i32>,
    ) {
        let combined_map = combine_params_and_index(params_vec, &num_of_values);

        for (i, param) in combined_map.iter() {
            if *param == 1 {
                current_values.push(self.computed_values[self.instruction_pointer + i])
            } else {
                current_values.push(
                    self.computed_values
                        [self.computed_values[self.instruction_pointer + i] as usize],
                )
            };
        }
    }

    fn compute_values_from_positions(
        &mut self,
        num_of_values: &usize,
        current_values: &mut Vec<i32>,
    ) {
        for i in (1..=*num_of_values).into_iter() {
            current_values.push(
                self.computed_values[self.computed_values[self.instruction_pointer + i] as usize],
            );
        }
    }
}

fn create_instruction_vec(instruction: &i32) -> Vec<usize> {
    instruction
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>()
}

fn get_opcode_from_instruction_vec(instruction_vec: &mut Vec<usize>) -> i32 {
    let vec_len = instruction_vec.len();
    let opcode_vec: Vec<usize> = instruction_vec.drain(vec_len - 2..vec_len).collect();
    let opcode = opcode_vec[0].to_string() + &opcode_vec[1].to_string();
    opcode.parse::<i32>().unwrap()
}

fn format_remaining_params(instruction_vec: Vec<usize>) -> Vec<usize> {
    let mut mutable_vec = instruction_vec;
    mutable_vec.reverse();
    if mutable_vec.len() != 4 {
        let additional_zeros = 4 - mutable_vec.len();
        let mut zeros_vec: Vec<usize> = vec![0; additional_zeros];
        mutable_vec.append(&mut zeros_vec);
    }
    mutable_vec
}

fn combine_params_and_index(params_vec: Vec<usize>, num_of_values: &usize) -> Vec<(usize, usize)> {
    (1..=*num_of_values)
        .collect::<Vec<usize>>()
        .into_iter()
        .zip(params_vec.into_iter())
        .collect()
}

pub trait ComputerActions {
    fn new(noun: i32, verb: i32, start_input: Vec<i32>) -> Computer;
    fn write(&mut self, value: i32, action: &str);
    fn run(&mut self);
    fn reset(&mut self, noun: i32, verb: i32);
    fn restore_gravity_assist_program(&mut self);
    fn format_instructions(&mut self, instruction: i32) -> i32;
    fn compute_values(&mut self, num_of_values: usize);
    fn compute_values_from_param_modes(
        &mut self,
        params_vec: Vec<usize>,
        num_of_values: &usize,
        current_values: &mut Vec<i32>,
    );
    fn compute_values_from_positions(
        &mut self,
        num_of_values: &usize,
        current_values: &mut Vec<i32>,
    );
}
