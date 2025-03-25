use std::sync::{Mutex, OnceLock};

use log::error;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::js_sys::Math::random;

use crate::ui;

const DISPLAY_WIDTH: u8 = 64;
const DISPLAY_HEIGHT: u8 = 32;
const ON_COLOR: [u8; 4] = Program::hex_to_rgba(0xff);
const OFF_COLOR: [u8; 4] = Program::hex_to_rgba(0xffffffff);
const RGBA: u8 = 4;

pub fn get_program() -> &'static Mutex<Program> {
    // this is some rust crazyness
    // This can only be written once so it's static
    static PROGRAM: OnceLock<Mutex<Program>> = OnceLock::new();
    PROGRAM.get_or_init(|| Mutex::new(Program::new()))
}

#[wasm_bindgen]
pub struct Program {
    memory: Vec<u8>,
    display: Vec<u8>,
    pub program_counter: u16,
    pub index_register: u16,
    call_stack: Vec<u16>,
    pub delay_timer: u8,
    pub sound_timer: u8,
    variable_regsiters: Vec<u8>,
    op_table: Vec<OpCodeFn>,
    f_op_table: Vec<OpCodeFn>,
    pub pressed_keys: u16, // each bit tells if the key is pressed
}

type OpCodeFn = fn(program: &mut Program, instruction: u16);

#[wasm_bindgen]
impl Program {

    pub fn get_font_address() -> usize {
    return 0x050;
    }

    pub fn get_start_address() -> u16 {
        return 0x200;
        const FONT_START_ADDR: usize = 0x050;
    }
    
    pub fn new() -> Self {
        const NULL_OP: OpCodeFn = |_, __| {};
        let mut p = Self {
            memory: vec![0; 4096],
            display: vec![0; 8192],
            program_counter: Self::get_start_address(),
            index_register: 0,
            call_stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            variable_regsiters: vec![0; 16],
            op_table: vec![NULL_OP; 0xF + 1],
            f_op_table: vec![NULL_OP; 0x65 + 1],
            pressed_keys: 0,
        };
        p.clear_display();
        p.set_font();
        p.set_instruction_table();
        p
    }

    pub fn reset(&mut self) {
        self.clear_display();
        self.program_counter = Self::get_start_address();
        self.index_register = 0;
        self.call_stack.clear();
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.variable_regsiters = vec![0; 16];
        self.pressed_keys = 0;
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.reset();
        for (i, value) in rom.iter().enumerate() {
            self.memory[i + Self::get_start_address() as usize] = *value;
        }
    }

    pub fn timer_tick(&mut self) {
        self.delay_timer = self.delay_timer.saturating_sub(1);
        self.sound_timer = self.sound_timer.saturating_sub(1);
        // if self.sound_timer != 0 {
        //     ui::beep();
        // } else {
        //     ui::stop_beep();
        // }
    }

    pub fn tick(&mut self) {
        let instr_first_byte = self.memory[self.program_counter as usize];
        let instr_second_byte = self.memory[(self.program_counter + 1) as usize];
        let entire_instruction = ((instr_first_byte as u16) << 8) | instr_second_byte as u16;
        let first_nible = instr_first_byte >> 4;
        self.program_counter += 2;
        self.op_table[first_nible as usize](self, entire_instruction);
    }

    pub fn set_key_down(&mut self, key: u8) {
        self.pressed_keys |= 0b1 << key;
    }

    pub fn set_key_up(&mut self, key: u8) {
        self.pressed_keys &= 0 << key;
    }

    fn key_is_pressed(&self, key: u8) -> bool {
        ((self.pressed_keys >> key) & 0b1) == 1
    }

    pub fn get_display(&self) -> *const u8 {
        self.display.as_ptr()
    }

    pub fn width() -> u8 {
        DISPLAY_WIDTH
    }

    #[inline]
    fn pixel_location(x: u8, y: u8) -> usize {
        ((y * RGBA) as usize * DISPLAY_WIDTH as usize) + (x * RGBA) as usize
    }

    fn set_font(&mut self) {
        // each character is 5 tall by 4 wide.
        // every bytes is a new character
        const CHARACTER_FONTS: [u8; 80] = [
            0xF, 0x9, 0x9, 0x9, 0xF, // 0
            0x2, 0x6, 0x2, 0x2, 0x7, // 1
            0xF, 0x1, 0xF, 0x8, 0xF, // 2
            0xF, 0x1, 0xF, 0x1, 0xF, // 3
            0x9, 0x9, 0xF, 0x1, 0x1, // 4
            0xF, 0x8, 0xF, 0x1, 0xF, // 5
            0xF, 0x8, 0xF, 0x9, 0xF, // 6
            0xF, 0x1, 0x2, 0x4, 0x4, // 7
            0xF, 0x9, 0xF, 0x9, 0xF, // 8
            0xF, 0x9, 0xF, 0x1, 0xF, // 9
            0xF, 0x9, 0xF, 0x9, 0x9, // A
            0xE, 0x9, 0xE, 0x9, 0xE, // B
            0xF, 0x8, 0x8, 0x8, 0xF, // C
            0xE, 0x9, 0x9, 0x9, 0xE, // D
            0xF, 0x8, 0xF, 0x8, 0xF, // E
            0xF, 0x8, 0xF, 0x8, 0x8, // F
        ];
        for (i, value) in CHARACTER_FONTS.iter().enumerate() {
            self.memory[i + Self::get_font_address()] = *value;
        }
    }

    fn clear_display(&mut self) {
        // there must be a better way of filling this
        for i in 0..self.display.len() {
            self.display[i] = OFF_COLOR[i % 4];
        }
    }

    const fn hex_to_rgba(hex: u32) -> [u8; 4] {
        [
            ((hex & 0xFF000000) >> 24) as u8,
            ((hex & 0x00FF0000) >> 16) as u8,
            ((hex & 0x0000FF00) >> 8) as u8,
            (hex & 0x000000FF) as u8,
        ]
    }

    fn pixel_is_on(&mut self, location: usize) -> bool {
        if ON_COLOR[0] == self.display[location] {
            return true;
        }
        if OFF_COLOR[0] == self.display[location] {
            return false;
        }
        panic!("Couldn't determine if pixel was on or off");
    }

    fn invert_pixel(&mut self, location: usize) {
        let color = if Self::pixel_is_on(self, location) {
            OFF_COLOR
        } else {
            ON_COLOR
        };
        self.display[location] = color[0];
        self.display[location + 1] = color[1];
        self.display[location + 2] = color[2];
        self.display[location + 3] = color[3];
    }

    pub fn set_instruction_table(&mut self) {
        self.op_table[0x0] = Program::op_0;
        self.op_table[0x1] = Program::op_1;
        self.op_table[0x2] = Program::op_2;
        self.op_table[0x3] = Program::op_3;
        self.op_table[0x4] = Program::op_4;
        self.op_table[0x5] = Program::op_5;
        self.op_table[0x6] = Program::op_6;
        self.op_table[0x7] = Program::op_7;
        self.op_table[0x8] = Program::op_8;
        self.op_table[0x9] = Program::op_9;
        self.op_table[0xA] = Program::op_A;
        self.op_table[0xB] = Program::op_B;
        self.op_table[0xC] = Program::op_C;
        self.op_table[0xD] = Program::op_D;
        self.op_table[0xE] = Program::op_E;
        self.op_table[0xF] = Program::op_F;

        self.f_op_table[0x7] = Program::op_FX07;
        self.f_op_table[0xA] = Program::op_FX0A;
        self.f_op_table[0x15] = Program::op_FX15;
        self.f_op_table[0x18] = Program::op_FX18;
        self.f_op_table[0x1E] = Program::op_FX1E;
        self.f_op_table[0x29] = Program::op_FX29;
        self.f_op_table[0x33] = Program::op_FX33;
        self.f_op_table[0x55] = Program::op_FX55;
        self.f_op_table[0x65] = Program::op_FX65;
    }

    // Instructions bellow
    fn op_0(program: &mut Program, instruction: u16) {
        match instruction {
            // clear screen
            0x00E0 => {
                program.clear_display();
            },
            // return from function
            0x00EE => {
                let pointer = program.call_stack.pop().expect("returned from a function without a return address");
                program.program_counter = pointer;
            },
            _ => error!("Encountered an execute machine language routine instruction. This isn't implemented")
        }
    }
    fn op_1(program: &mut Program, instruction: u16) {
        let jump_location = instruction & 0x0FFF;
        program.program_counter = jump_location;
    }
    fn op_2(program: &mut Program, instruction: u16) {
        // call the function
        let pointer = instruction & 0x0FFF;
        program.call_stack.push(program.program_counter);
        program.program_counter = pointer;
    }
    fn op_3(program: &mut Program, instruction: u16) {
        let register_name = ((instruction & 0x0F00) >> 8) as usize;
        let value = (instruction & 0x00FF) as u8;
        if program.variable_regsiters[register_name] == value {
            program.program_counter += 2;
        }
    }
    fn op_4(program: &mut Program, instruction: u16) {
        let register_name = ((instruction & 0x0F00) >> 8) as usize;
        let value = (instruction & 0x00FF) as u8;
        if program.variable_regsiters[register_name] != value {
            program.program_counter += 2;
        }
    }
    fn op_5(program: &mut Program, instruction: u16) {
        let x_register_name = ((instruction & 0x0F00) >> 8) as usize;
        let y_register_name = ((instruction & 0x00F0) >> 4) as usize;
        if program.variable_regsiters[x_register_name]
            == program.variable_regsiters[y_register_name]
        {
            program.program_counter += 2;
        }
    }
    fn op_6(program: &mut Program, instruction: u16) {
        let register_name = ((instruction & 0x0F00) >> 8) as usize;
        let value = (instruction & 0x00FF) as u8;
        program.variable_regsiters[register_name] = value;
    }
    fn op_7(program: &mut Program, instruction: u16) {
        let register_name = ((instruction & 0x0F00) >> 8) as usize;
        let value = (instruction & 0x00FF) as u8;
        let register = &mut program.variable_regsiters[register_name];
        *register = register.wrapping_add(value);
    }
    fn op_8(program: &mut Program, instruction: u16) {
        // TODO: make these configurable for more modern interpreter
        // https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#8xy6-and-8xye-shift
        // https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#fx55-and-fx65-store-and-load-memory
        let x_register_name = ((instruction & 0x0F00) >> 8) as usize;
        let y_register_name = ((instruction & 0x00F0) >> 4) as usize;
        let y_register = program.variable_regsiters[y_register_name];

        // this depends on the target platform: we default to 0 instead of VF
        // since we choose to suport Chip 8 only, it defaults the flag to 0.
        // UPDATE: this doesn't work with the Octo compiler since it expects no reset of the flag
        let x_register = &mut program.variable_regsiters[x_register_name];

        let op_type = instruction & 0x000F;
        match op_type {
            0x0 => {
                *x_register = y_register;
            }
            0x1 => {
                *x_register |= y_register;
            }
            0x2 => {
                *x_register &= y_register;
            }
            0x3 => {
                *x_register ^= y_register;
            }
            0x4 => {
                let (new_value, overflow) = x_register.overflowing_add(y_register);
                *x_register = new_value;
                let overflow_value = if overflow { 1 } else { 0 };
                program.variable_regsiters[0xF] = overflow_value;
            }
            0x5 => {
                let f_flag_value = if *x_register >= y_register { 1 } else { 0 };
                *x_register = x_register.wrapping_sub(y_register);
                program.variable_regsiters[0xF] = f_flag_value;
            }
            0x6 => {
                let shifted_out = y_register & 0b1;
                let new_value = y_register >> 1;
                *x_register = new_value;
                program.variable_regsiters[0xF] = shifted_out;
            }
            0x7 => {
                let f_flag_value = if y_register >= *x_register { 1 } else { 0 };
                *x_register = y_register.wrapping_sub(*x_register);
                program.variable_regsiters[0xF] = f_flag_value;
            }
            0xE => {
                let shifted_out = (y_register & 0b10000000) >> 7;
                let new_value = y_register << 1;
                *x_register = new_value;
                program.variable_regsiters[0xF] = shifted_out;
            }
            _ => error!("This arithmetic operation is not supported"),
        }

        // program.variable_regsiters[0xF] = f_flag_value;
    }
    fn op_9(program: &mut Program, instruction: u16) {
        let x_register_name = ((instruction & 0x0F00) >> 8) as usize;
        let y_register_name = ((instruction & 0x00F0) >> 4) as usize;
        if program.variable_regsiters[x_register_name]
            != program.variable_regsiters[y_register_name]
        {
            program.program_counter += 2;
        }
    }
    #[allow(non_snake_case)]
    fn op_A(program: &mut Program, instruction: u16) {
        let location = instruction & 0x0FFF;
        program.index_register = location;
    }
    #[allow(non_snake_case)]
    fn op_B(program: &mut Program, instruction: u16) {
        // TODO: make this configurable with SUPER-CHIP
        let jump_pointer = instruction & 0x0FFF;
        let offset = program.variable_regsiters[0] as u16;
        program.program_counter = jump_pointer + offset;
    }
    #[allow(non_snake_case)]
    fn op_C(program: &mut Program, instruction: u16) {
        let r = (random() * (u8::MAX as f64)) as u8;
        let register_name = ((instruction & 0x0F00) >> 8) as usize;
        let value = (instruction & 0x00FF) as u8;
        program.variable_regsiters[register_name] = value & r;
    }
    #[allow(non_snake_case)]
    fn op_D(program: &mut Program, instruction: u16) {
        let x_register = (instruction & 0x0F00) >> 8;
        let y_register = (instruction & 0x00F0) >> 4;

        // we use modulo in case the variable goes off screen
        let x_start = program.variable_regsiters[x_register as usize] % DISPLAY_WIDTH;
        let y_start = program.variable_regsiters[y_register as usize] % DISPLAY_HEIGHT;
        let rows = (instruction & 0x000F) as u8;

        program.variable_regsiters[0xF_usize] = 0;
        for y in 0..rows {
            let y_location = y_start + y;
            if y_location >= DISPLAY_HEIGHT {
                break;
            }

            let sprite_row = program.memory[(program.index_register + y as u16) as usize];

            for x in 0_u8..8 {
                let x_location = x_start + x;
                if x_location >= DISPLAY_WIDTH {
                    break;
                }
                if ((sprite_row >> (7 - x)) & 0b1) == 1 {
                    let pixel_location = Program::pixel_location(x_location, y_location);
                    if program.pixel_is_on(pixel_location) {
                        program.variable_regsiters[0xF_usize] = 1;
                    }
                    program.invert_pixel(pixel_location);
                }
            }
        }
    }
    #[allow(non_snake_case)]
    fn op_E(program: &mut Program, instruction: u16) {
        let register_name = ((instruction & 0x0F00) >> 8) as usize;
        let key = program.variable_regsiters[register_name];
        let is_pressed = program.key_is_pressed(key);
        let instr_type = instruction & 0x00FF;
        match (instr_type, is_pressed) {
            (0x9E, true) => {
                program.program_counter += 2;
            }
            (0x9E, false) => {}
            (0xA1, false) => {
                program.program_counter += 2;
            }
            (0xA1, true) => {}
            _ => error!("This isntruction type shouldn't exist"),
        }
    }
    #[allow(non_snake_case)]
    fn op_F(program: &mut Program, instruction: u16) {
        let instr_type = (instruction & 0x00FF) as usize;
        let register_name = (instruction & 0x0F00) >> 8;
        program.f_op_table[instr_type](program, register_name);
    }
    #[allow(non_snake_case)]
    fn op_FX07(program: &mut Program, register_name: u16) {
        program.variable_regsiters[register_name as usize] = program.delay_timer;
    }
    #[allow(non_snake_case)]
    fn op_FX0A(program: &mut Program, register_name: u16) {
        for key in 0..0xF_u8 {
            if program.key_is_pressed(key) {
                program.variable_regsiters[register_name as usize] = key;
                return;
            }
        }
        program.program_counter -= 2;
    }
    #[allow(non_snake_case)]
    fn op_FX15(program: &mut Program, register_name: u16) {
        program.delay_timer = program.variable_regsiters[register_name as usize];
    }
    #[allow(non_snake_case)]
    fn op_FX18(program: &mut Program, register_name: u16) {
        program.sound_timer = program.variable_regsiters[register_name as usize];
    }
    #[allow(non_snake_case)]
    fn op_FX1E(program: &mut Program, register_name: u16) {
        program.index_register += program.variable_regsiters[register_name as usize] as u16;
        // this counts as an "overflow" on some interpreters
        if program.index_register > 0xFFF {
            program.variable_regsiters[0xF_usize] = 1;
        }
    }
    #[allow(non_snake_case)]
    fn op_FX29(program: &mut Program, register_name: u16) {
        let hex = program.variable_regsiters[register_name as usize] & 0x0F;
        program.index_register =
            program.memory[Self::get_font_address() + ((hex * 5) as usize)] as u16;
    }
    #[allow(non_snake_case)]
    fn op_FX33(program: &mut Program, register_name: u16) {
        let register_value = program.variable_regsiters[register_name as usize];
        let d1 = register_value / 100;
        let d2 = (register_value / 10) % 10;
        let d3 = register_value % 10;
        program.memory[program.index_register as usize] = d1;
        program.memory[(program.index_register + 1) as usize] = d2;
        program.memory[(program.index_register + 2) as usize] = d3;
    }
    #[allow(non_snake_case)]
    fn op_FX55(program: &mut Program, register_name: u16) {
        // Since we are targeting the original chip 8,
        // we also increment the i register

        for i in 0..(register_name + 1) {
            program.memory[program.index_register as usize] =
                program.variable_regsiters[i as usize];
            program.index_register += 1;
        }
    }
    #[allow(non_snake_case)]
    fn op_FX65(program: &mut Program, register_name: u16) {
        for i in 0..(register_name + 1) {
            program.variable_regsiters[i as usize] =
                program.memory[program.index_register as usize];
            program.index_register += 1;
        }
    }
}
