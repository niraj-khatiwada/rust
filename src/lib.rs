use lib::codecs::get_audio_codecs_directly;
use lib::codecs::{audio, video};

fn main() {
    println!("{:?}", audio::get_audio_codecs());
    println!("{:?}", video::get_video_codecs());
    println!("{:?}", get_audio_codecs_directly());
}
