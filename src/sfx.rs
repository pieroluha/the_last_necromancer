use crate::prelude::*;
pub use bevy_kira_audio::Audio;
use bevy_kira_audio::{AudioChannel, AudioPlugin, AudioSource, InstanceHandle, PlaybackState};

pub struct AudioChannels {
    pub audio_channel_jank: AudioChannel,
    pub audio_channel_420: AudioChannel,
    pub audio_channel_69: AudioChannel,
}
impl Default for AudioChannels {
    fn default() -> Self {
        Self {
            audio_channel_jank: AudioChannel::new("I Scream".to_string()),
            audio_channel_420: AudioChannel::new("Vlaze it".to_string()),
            audio_channel_69: AudioChannel::new("Lewds".to_string()),
        }
    }
}

#[derive(Default)]
pub struct PlaySfx {
    pub boom: bool,
    pub oof: bool,
    pub order: bool,
    pub death: bool,
    pub bullet_barrage: bool,
    pub skull_buster: bool,
    pub deezer: bool,
}

pub struct OofInstance {
    handle: InstanceHandle,
}

pub struct OrderInstance {
    handle: InstanceHandle,
}

pub struct BulletBarrageInstance {
    handle: InstanceHandle,
}

pub struct SkullBusterInstance {
    handle: InstanceHandle,
}

pub struct DeezerInstance {
    handle: InstanceHandle,
}

fn start_loop(audio: Res<Audio>, sfx_handles: Res<SfxHandles>, audio_channels: Res<AudioChannels>) {
    audio.play_looped_in_channel(sfx_handles.bgm.clone(), &audio_channels.audio_channel_jank);
}

fn edit_volume(audio: Res<Audio>, audio_channels: Res<AudioChannels>) {
    audio.set_volume_in_channel(0.8, &audio_channels.audio_channel_jank);
    audio.set_volume_in_channel(0.5, &audio_channels.audio_channel_420);
    audio.set_volume_in_channel(2.0, &audio_channels.audio_channel_69);
}

fn setup_instance_handles(
    mut commands: Commands,
    sfx_handles: Res<SfxHandles>,
    audio_channels: Res<AudioChannels>,
    audio: Res<Audio>,
) {
    let handle = audio.play_in_channel(sfx_handles.blank.clone(), &audio_channels.audio_channel_69);

    commands.insert_resource(OofInstance {
        handle: handle.clone(),
    });

    commands.insert_resource(OrderInstance {
        handle: handle.clone(),
    });

    commands.insert_resource(BulletBarrageInstance {
        handle: handle.clone(),
    });

    commands.insert_resource(SkullBusterInstance {
        handle: handle.clone(),
    });

    commands.insert_resource(OrderInstance {
        handle: handle.clone(),
    });

    commands.insert_resource(DeezerInstance {
        handle: handle.clone(),
    });
}

fn play_sfx(
    audio: Res<Audio>,
    sfx_handles: Res<SfxHandles>,
    audio_channels: Res<AudioChannels>,
    oof_instance: Res<OofInstance>,
    order_instance: Res<OrderInstance>,
    bullet_barrage_instance: Res<BulletBarrageInstance>,
    skull_buster_instance: Res<SkullBusterInstance>,
    deezer_instance: Res<DeezerInstance>,
    mut play_sfx: ResMut<PlaySfx>,
    mut commands: Commands,
) {
    if play_sfx.boom {
        play_sfx.boom = false;
        audio.play_in_channel(sfx_handles.boom.clone(), &audio_channels.audio_channel_420);
    }

    if play_sfx.oof {
        match audio.state(oof_instance.handle.clone()) {
            PlaybackState::Stopped => {
                commands.insert_resource(OofInstance {
                    handle: audio
                        .play_in_channel(sfx_handles.oof.clone(), &audio_channels.audio_channel_69),
                });
            }
            _ => (),
        }
        play_sfx.oof = false;
    }

    if play_sfx.order {
        match audio.state(order_instance.handle.clone()) {
            PlaybackState::Stopped => {
                commands.insert_resource(OrderInstance {
                    handle: audio.play_in_channel(
                        sfx_handles.jubba.clone(),
                        &audio_channels.audio_channel_69,
                    ),
                });
            }
            _ => (),
        }
        play_sfx.order = false;
    }

    if play_sfx.death {
        audio.play_in_channel(
            sfx_handles.minion_roar.clone(),
            &audio_channels.audio_channel_69,
        );
        play_sfx.death = false;
    }

    if play_sfx.bullet_barrage {
        match audio.state(bullet_barrage_instance.handle.clone()) {
            PlaybackState::Stopped => {
                commands.insert_resource(BulletBarrageInstance {
                    handle: audio.play_in_channel(
                        sfx_handles.bullet_barrage.clone(),
                        &audio_channels.audio_channel_69,
                    ),
                });
            }
            _ => (),
        }
        play_sfx.bullet_barrage = false;
    }

    if play_sfx.skull_buster {
        match audio.state(skull_buster_instance.handle.clone()) {
            PlaybackState::Stopped => {
                commands.insert_resource(SkullBusterInstance {
                    handle: audio.play_in_channel(
                        sfx_handles.skull_buster.clone(),
                        &audio_channels.audio_channel_69,
                    ),
                });
            }
            _ => (),
        }
        play_sfx.skull_buster = false;
    }

    if play_sfx.deezer {
        match audio.state(deezer_instance.handle.clone()) {
            PlaybackState::Stopped => {
                commands.insert_resource(DeezerInstance {
                    handle: audio.play_in_channel(
                        sfx_handles.deezer.clone(),
                        &audio_channels.audio_channel_69,
                    ),
                });
            }
            _ => (),
        }
        play_sfx.deezer = false;
    }
}

pub struct SfxPlugin;
impl Plugin for SfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .init_resource::<PlaySfx>()
            .init_resource::<AudioChannels>()
            .add_system_set(SystemSet::on_enter(Playing).with_system(start_loop))
            .add_system_set(
                SystemSet::on_enter(Playing)
                    .with_system(setup_instance_handles.label("setup_instance")),
            )
            .add_system_set(
                SystemSet::on_enter(Playing)
                    .with_system(edit_volume)
                    .after("setup_instance"),
            )
            .add_system_set(SystemSet::on_update(Playing).with_system(play_sfx));
    }
}
