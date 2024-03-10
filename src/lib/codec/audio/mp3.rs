pub fn convert_to_mp3() {
    println!("Converting to mp3");

    // Goes to the base module, "lib"
    crate::lib();

    // Relative
    // Don't use this since it's confusing
    super::super::super::transcode::transcode();
}
