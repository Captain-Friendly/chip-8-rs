use std::sync::Mutex;

use wasm_bindgen::JsCast;
use web_sys::{HtmlTableElement, HtmlTableRowElement, Node};

use crate::{
    emulator::Program,
    ui::{add_class_name, document, get_element, remove_class_name, to_number},
};

pub static INTERVAL_HANDLE: Mutex<Option<i32>> = Mutex::new(None);
pub static RENDER_DEBUGGER: Mutex<bool> = Mutex::new(false);
pub static BREAKPOINTS: Mutex<Vec<usize>> = Mutex::new(Vec::new());

pub fn render_debugger(program: &Program) {
    // render_registers(
    //     &program.variable_regsiters,
    //     &get_element(&document(), "#registers-table"),
    // );

    // render_memory(
    //     &program.memory,
    //     program.program_counter as usize,
    //     &get_element(&document(), "#memory-table"),
    // );
}

fn render_registers(registers: &[u8], table: &HtmlTableElement) {
    let tbody = table
        .query_selector("tbody")
        .expect("bad query for tbody")
        .expect("There was no tbody");

    // setting data
    registers.iter().enumerate().for_each(|(i, &value)| {
        let old_node = tbody
            .child_nodes()
            .item((i * 2 + 1) as u32)
            .expect("Couldn't get register memory row {i}")
            .child_nodes()
            .item(3)
            .expect("Couldn't get old register memory");
        let old_value: u8 = to_number(&old_node);
        if old_value != value {
            old_node.set_text_content(Some(format!("{value}").as_str()));
        }
    });
}

fn render_memory(memory: &[u8], program_counter: usize, table: &HtmlTableElement) {
    let tbody = table
        .query_selector("tbody")
        .expect("bad query for tbody")
        .expect("There was no tbody");

    const INSTRUCTION_SIZE: usize = 2;
    const PAGE_SIZE: usize = 16;
    let row_number = program_counter % (PAGE_SIZE * INSTRUCTION_SIZE);
    let alignment = program_counter % INSTRUCTION_SIZE;
    let page_start = program_counter - row_number + alignment;

    for i in 0..PAGE_SIZE {
        let address = page_start + i * INSTRUCTION_SIZE;
        let value = ((memory[address] as u16) << 8) | memory[address + 1] as u16;
        let row: HtmlTableRowElement = tbody
            .child_nodes()
            .item((i * 2 + 1) as u32)
            .expect("Couldn't get register memory row {i}")
            .dyn_into()
            .expect("Could not dyn into a row");
        let address_node = row
            .child_nodes()
            .item(1)
            .expect("Could not get the child node for the address");
        let value_node = row
            .child_nodes()
            .item(5)
            .expect("Could not get the child node for the address");

        let old_address: usize = to_number(&address_node);

        let old_value: u16 = u16::from_str_radix(
            value_node.text_content().unwrap().trim_start_matches("0x"),
            16,
        )
        .unwrap();

        // update if there is a difference
        if address != old_address || value != old_value {
            let mnemonic_node = row
                .child_nodes()
                .item(3)
                .expect("Could not get the child node for the address");
            render_address(&address_node, &mnemonic_node, &value_node, address, value);
        }
        if address == program_counter {
            add_class_name(&row, "current-instruction");
        } else {
            remove_class_name(&row, "current-instruction");
        }
        let is_breakpoint = BREAKPOINTS
            .lock()
            .unwrap()
            .iter()
            .any(|&breakpoint| breakpoint == address);
        if is_breakpoint {
            add_class_name(&row, "breakpoint");
        } else {
            remove_class_name(&row, "breakpoint");
        }
    }
}

fn render_address(
    address_node: &Node,
    mnemonic_node: &Node,
    value_node: &Node,
    address: usize,
    value: u16,
) {
    address_node.set_text_content(Some(format!("{address}").as_str()));
    value_node.set_text_content(Some(format!("{:#04x}", value).as_str()));
    mnemonic_node.set_text_content(Some(interpret_instruction(value).as_str()));
}

fn interpret_instruction(instruction: u16) -> String {
    let first = (instruction & 0xF000) >> 12;
    let second = (instruction & 0x0F00) >> 8;
    let third = (instruction & 0x00F0) >> 4;
    let fourth = instruction & 0x000F;
    let nnn = instruction & 0x0FFF;
    let kk = instruction & 0x00FF;
    match (first, second, third, fourth) {
        (0x0, 0x0, 0xE, 0x0) => "CLS".to_string().to_string(),
        (0x0, 0x0, 0xE, 0xE) => "RET".to_string().to_string(),
        (0x0, _, _, _) => format!("SYS {nnn:x}").to_string(),
        (0x1, _, _, _) => format!("JP {nnn:x}").to_string(),
        (0x2, _, _, _) => format!("CALL {nnn:x}").to_string(),
        (0x3, _, _, _) => format!("SE V[{second:x}], {kk:x}").to_string(),
        (0x4, _, _, _) => format!("SNE V[{second:x}], {kk:x}").to_string(),
        (0x5, _, _, 0x0) => format!("SE V[{second:x}], V[{third:x}]").to_string(),
        (0x6, _, _, _) => format!("LD V[{second:x}], {kk:x}").to_string(),
        (0x7, _, _, _) => format!("ADD V[{second:x}], {kk:x}").to_string(),
        (0x8, _, _, 0x0) => format!("LD V[{second:x}], V[{third:x}]").to_string(),
        (0x8, _, _, 0x1) => format!("OR V[{second:x}], V[{third:x}]").to_string(),
        (0x8, _, _, 0x2) => format!("AND V[{second:x}], V[{third:x}]").to_string(),
        (0x8, _, _, 0x3) => format!("XOR V[{second:x}], V[{third:x}]").to_string(),
        (0x8, _, _, 0x4) => format!("ADD V[{second:x}], V[{third:x}]").to_string(),
        (0x8, _, _, 0x5) => format!("SUB V[{second:x}], V[{third:x}]").to_string(),
        (0x8, _, _, 0x6) => format!("SHR V[{second:x}]").to_string(),
        (0x8, _, _, 0x7) => format!("SUBR V[{second:x}], V[{third:x}]").to_string(),
        (0x8, _, _, 0xE) => format!("SHL V[{second:x}]").to_string(),
        (0x9, _, _, 0x0) => format!("SNE V[{second:x}], V[{third:x}]").to_string(),
        (0xA, _, _, _) => format!("LD I, {nnn:x}").to_string(),
        (0xB, _, _, _) => format!("JP V0, {nnn:x}").to_string(),
        (0xC, _, _, _) => format!("RND V[{second:x}], {kk:x}").to_string(),
        (0xD, _, _, _) => format!("DRW V[{second:x}], V[{third:x}], {fourth}").to_string(),
        (0xE, _, 0x9, 0xE) => format!("SKP V[{second:x}]").to_string(),
        (0xE, _, 0xA, 0x1) => format!("SKNP V[{second:x}]").to_string(),
        (0xF, _, 0x0, 0x7) => format!("LD V[{second:x}], DT").to_string(),
        (0xF, _, 0x0, 0xA) => format!("LD V[{second:x}], K").to_string(),
        (0xF, _, 0x1, 0x5) => format!("LD DT, V[{second:x}]").to_string(),
        (0xF, _, 0x1, 0x8) => format!("LD ST, V[{second:x}]").to_string(),
        (0xF, _, 0x1, 0xE) => format!("ADD I, V[{second:x}]").to_string(),
        (0xF, _, 0x2, 0x9) => format!("LD F, V[{second:x}]").to_string(),
        (0xF, _, 0x3, 0x3) => format!("LD B, V[{second:x}]").to_string(),
        (0xF, _, 0x5, 0x5) => format!("LD [I], V[{second:x}]").to_string(),
        (0xF, _, 0x6, 0x5) => format!("LD V[{second:x}], [I]").to_string(),
        _ => "".to_string(),
    }
}
