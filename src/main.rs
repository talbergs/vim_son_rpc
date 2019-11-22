mod core;

fn main() {
    let mut eh = core::eventhandler::EventHandler::new();

    eh.recv();
}
