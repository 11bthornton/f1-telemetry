pub mod event_loop;

pub use event_loop::event_loop_generator;
pub use event_loop::GeneratorIteratorAdapter;

use crate::telemetry_data::F1Data;
use crate::telemetry_data::car_damage_data::PacketCarDamageData;

pub trait DataHandler {
    #![allow(unused_variables)]

    fn on_car_damage_data(&mut self, damage_data: PacketCarDamageData) {}
    fn on_car_setup_data() {}
    fn on_car_telemetry_data() {}
    fn on_event_data() {}

    fn listen(&mut self, ip: &str, port: &str) -> () {

        let generator = futures::executor::block_on(
            event_loop_generator(ip, port)
        );

        let mut iter = GeneratorIteratorAdapter::new(generator);

        while let Some(packet) = iter.next() {
            match packet {
                F1Data::Damage(pack) => self.on_car_damage_data(pack),
                _ => {}
            }
        }

    }
}
