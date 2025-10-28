use lib::codecs::{audio, video};

fn main() {
    audio::mp3::convert();
    video::mp4::convert();
}
