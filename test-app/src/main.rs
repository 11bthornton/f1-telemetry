use f1_game_client::event_loop::DataHandler;
use f1_game_client::telemetry_data::PacketCarDamageData;

#[derive(Default)]
pub struct SimpleEventHandler {
    packets_dealt_with: usize
}

impl DataHandler for SimpleEventHandler {

    fn on_car_damage_data(&mut self, damage_data: PacketCarDamageData) {

        self.packets_dealt_with += 1;

        eprintln!("dealt with {} packets", self.packets_dealt_with);

    }
}

fn main() {
    println!("Hello, world!");

    let mut manager = SimpleEventHandler::default();

    manager.listen("127.0.0.1", "33333");
}
