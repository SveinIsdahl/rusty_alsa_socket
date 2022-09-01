use alsa::mixer;

pub fn init_mixer() {
    let mixer: mixer::Mixer = mixer::Mixer::new("default", true).unwrap();

    let mixer_select = mixer::SelemId::new("Master", 0);
    let mixer_channel = match mixer.find_selem(&mixer_select) {
        Some(value) => {value}
        None => {
            return ();
        }

    };
    
    mixer_channel.set_playback_volume_all(1000).unwrap();
    //let vol = mixer_channel.get_id();//mixer_channel.get_playback_volume(mixer::SelemChannelId::Unknown).unwrap();
    let test = mixer_channel.get_playback_volume(mixer::SelemChannelId::FrontLeft).unwrap();
    //Loop though enumerator, to handle all channels
    println!("{test}");
}