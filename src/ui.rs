use std::str::FromStr;

use crate::{
    debugger::{render_debugger, RENDER_DEBUGGER},
    emulator::Program,
};
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{
    CanvasRenderingContext2d, Document, Element, HtmlAudioElement, HtmlCanvasElement, ImageData,
    Node,
};

pub fn render_emulator(program: &Program, ctx: &CanvasRenderingContext2d) {
    // let width = Program::width() as u32;

    // let data = ImageData::new_with_u8_clamped_array(Clamped(&program.get_display()), width)
    //     .expect("Could not create the image data");

    // ctx.put_image_data(&data, 0.0, 0.0)
    //     .expect("Could not put image data");
    // if *RENDER_DEBUGGER.lock().unwrap() {
    //     render_debugger(program);
    // }
}

pub fn get_element<T: JsCast>(document: &Document, id: &str) -> T {
    let type_name = std::any::type_name::<T>();
    document
        .query_selector(id)
        .expect("The query was wrong")
        .unwrap_or_else(|| panic!("The element {type_name} could not be found"))
        .dyn_into()
        .unwrap_or_else(|_| panic!("Could not dyn into the element {type_name}"))
}

pub fn to_number<T>(node: &Node) -> T
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    node.text_content()
        .expect("Could not get text content of node")
        .parse()
        .expect("Could not parse node to number")
}

pub fn add_class_name(element: &Element, name: &str) {
    let mut classes = element.class_name();
    classes.push_str(&format!(" {name}"));
    element.set_class_name(&classes);
}

pub fn remove_class_name(element: &Element, name: &str) {
    let new_classes = element.class_name().replace(name, "").replace("  ", "");
    element.set_class_name(new_classes.trim());
}

fn audio() -> HtmlAudioElement {
    document()
        .query_selector("#beep")
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlAudioElement>()
        .unwrap()
}

// the audio is continuously playing, we only unmute it to have a sound.
// this is faster and easier than having play/pause
pub fn beep() {
    audio().set_muted(false);
}

pub fn stop_beep() {
    audio().set_muted(true);
}

pub fn canvas() -> HtmlCanvasElement {
    document()
        .query_selector("canvas")
        .expect("the selector is not valid")
        .expect("There was no canvas in the html document")
        .dyn_into()
        .expect("Could not dyn into canvas")
}

pub fn get_canvas_context() -> CanvasRenderingContext2d {
    canvas()
        .get_context("2d")
        .expect("Could not get the canvas context")
        .expect("There was no context")
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("Couldn't transform the js object into canvas context")
}

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global 'window' found")
}

pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("there was no document for this window")
}
