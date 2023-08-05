use f1_game_client::event_loop::DataHandler;
use f1_game_client::telemetry_data::event_data::PacketEventFinal;
use f1_game_client::telemetry_data::PacketCarDamageData;
use std::collections::HashMap;

#[derive(Default)]
pub struct SimpleEventHandler {
    packets_dealt_with: usize,
    player_data: HashMap<usize, f1_game_client::telemetry_data::participant_data::ParticipantData>,
}

impl DataHandler for SimpleEventHandler {
    fn on_car_damage_data(& self, damage_data: PacketCarDamageData) {
        self.packets_dealt_with += 1;


        // println!("{:#?}", damage_data.car_damage_data[damage_data.header.player_car_index as usize])
        eprintln!("dealt with {} packets", self.packets_dealt_with);
    }

    fn on_event_data(& self, event_data: PacketEventFinal) {
        match event_data.r#type {
            f1_game_client::telemetry_data::event_data::EventType::Buttons(_) => {}

            _ => {
                // println!("{:#?}", event_data);
            }
        }
    }

    fn on_participant_data(
        & self,
        participant_data: f1_game_client::telemetry_data::PacketParticipantData,
    ) {
        for (idx, data) in participant_data.participants.into_iter().enumerate() {
            self.player_data.insert(idx, data);
        }

        for (_, driver) in &self.player_data {
            // println!("{} -> {:?} -> {:?}", driver.name(), driver.team_id, driver.nationality);
        }
    }

    fn on_session_data(& self, session_data: f1_game_client::telemetry_data::PacketSessionData) {
        // println!("Track! {:?}", session_data.track)
    }
}

fn main() {
    println!("Hello, world!");

    let  manager = SimpleEventHandler::default();

    manager.listen("192.168.1.180", "33333");
}
