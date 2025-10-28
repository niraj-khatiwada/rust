use lib::codecs::{audio, video};

fn modules() {
    audio::mp3::convert();
    video::mp4::convert();
}
