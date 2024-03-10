pub mod audio;
pub mod video;

// Export mp3 as mp2 directly in codec module
pub use audio::mp3 as mp2;
