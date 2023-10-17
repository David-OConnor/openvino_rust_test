use openvino;
// use openvino_sys;

use env_logger; // todo temp 

fn main() {
    env_logger::init(); // todo temp
    println!("OV version: {}", openvino::version());

    let core = openvino::Core::new(None).expect("to instantiate the OpenVINO library");

    // todo: Not working; guessing from the Python API, but not able to find the `Core` methods
    // on Rust docs.
    // println!("Available devices: {}", core.available_devices());
}
