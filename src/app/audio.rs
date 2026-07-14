use super::DiffPlayerApp;

impl DiffPlayerApp {
    pub(super) fn drain_audio_and_update_levels(&mut self) {
        let is_playing = self.playback.is_playing;

        // ── Channel A ────────────────────────────────────────────────────────
        let mut processed_a = false;
        if let Some(dec) = &mut self.decoder_a {
            if let Some(sink) = &self.sink_a {
                while let Ok(audio) = dec.audio_rx.try_recv() {
                    if is_playing {
                        processed_a = true;
                        sink.append(rodio::buffer::SamplesBuffer::new(
                            audio.channels,
                            audio.sample_rate,
                            audio.samples,
                        ));
                    }
                }
            } else {
                while let Ok(_) = dec.audio_rx.try_recv() {}
            }
            if is_playing {
                let l = *dec.loudness_arc.lock();
                self.view.loudness_a = l;
            }
        }
        if !processed_a || !is_playing {
            self.view.loudness_a.true_peak = [0.0, 0.0];
        }

        // ── Channel B ────────────────────────────────────────────────────────
        let mut processed_b = false;
        if let Some(dec) = &mut self.decoder_b {
            if let Some(sink) = &self.sink_b {
                while let Ok(audio) = dec.audio_rx.try_recv() {
                    if is_playing {
                        processed_b = true;
                        sink.append(rodio::buffer::SamplesBuffer::new(
                            audio.channels,
                            audio.sample_rate,
                            audio.samples,
                        ));
                    }
                }
            } else {
                while let Ok(_) = dec.audio_rx.try_recv() {}
            }
            if is_playing {
                let l = *dec.loudness_arc.lock();
                self.view.loudness_b = l;
            }
        }
        if !processed_b || !is_playing {
            self.view.loudness_b.true_peak = [0.0, 0.0];
        }
    }

    pub(super) fn apply_sink_volumes(&mut self) {
        if let Some(sink) = &self.sink_a {
            if self.view.mute_a {
                sink.set_volume(0.0);
            } else {
                sink.set_volume(1.0); // Hardcoded fixed volume
            }
        }
        if let Some(sink) = &self.sink_b {
            if self.view.mute_b {
                sink.set_volume(0.0);
            } else {
                sink.set_volume(1.0); // Hardcoded fixed volume
            }
        }
    }
}
