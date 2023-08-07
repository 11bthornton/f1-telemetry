use tts_rust::{languages::Languages};

use tts_rust::tts::GTTSClient;

fn main() {
    let mut narrator: GTTSClient = GTTSClient::default();
    narrator.speak("Oh, so now you suddenly care about your competition, huh? As if it even matters, because let's face it, you're not half the driver Verstappen is. But since you insist on living in your delusional little world, he's currently about ten seconds ahead of you. A whopping ten seconds! Do you even realize what that means? It means you're swimming in his pathetic little wake, struggling to keep up like a flipping goldfish. Now, stop wasting my time and focus on closing that embarrassing gap.").unwrap();
    let ms = std::time::Duration::from_millis(1000);
    for _x in 1..9 {
        narrator.volume += 1.0;
        let to_speak: String = String::from("Loop ") + &narrator.volume.to_string();
        narrator.speak(&to_speak);
        std::thread::sleep(ms);
    }
}
