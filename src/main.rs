use std::time;
use std::time::Duration;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{Source};
use std::f32::consts::PI;
use std::i16;
use hound;


struct WaveTableOscilator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
}

impl WaveTableOscilator {
    fn new(sample_rate: u32, wave_table: Vec<f32>) -> WaveTableOscilator {
        return WaveTableOscilator {
            sample_rate: sample_rate,
            wave_table: wave_table,
            index: 0.0,
            index_increment: 0.0,
        };
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }

    fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        return sample;
    }

    fn lerp(&self) -> f32 {
        let truncated_index: usize = self.index as usize;
        let next_index: usize = (truncated_index + 1) % self.wave_table.len();

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight: f32 = 1.0 - next_index_weight;

        return truncated_index_weight * self.wave_table[truncated_index] + next_index_weight * self.wave_table[next_index];
    }
}

impl Iterator for WaveTableOscilator {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        return Some(self.get_sample());
    }
}

impl Source for WaveTableOscilator {
    fn channels(&self) -> u16 {
        return 1;
    }

    fn sample_rate(&self) -> u32 {
        return self.sample_rate;
    }

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }
    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}

fn main() {
    println!("Hello, world!");


    let wave_table_size: usize = 64;

    let notes: Vec<f32> = vec![261.63,
                               277.18,
                               293.66,
                               311.13,
                               329.63,
                               349.23,
                               369.99,
                               392.00,
                               415.30,
                               440.00,
                               466.16,
                               261.63,
                               277.18,
                               293.66,
                               440.00,
                               493.88,
                               329.63,
                               311.13,
                               349.23,
                               369.99,
                               392.00,
                               415.30,
                               493.88,
                               466.16,
                               349.23,
                               261.63,
                               293.66,
                               311.13,
                               329.63,
                               277.18,
                               329.63,
                               369.99,
                               415.30,
                               440.00,
                               466.16,
                               277.18,
                               493.88,
                               392.00,
                               261.63,
                               311.13,
                               349.23,
                               293.66,
                               369.99,
                               392.00,
                               415.30,
                               440.00,
                               466.16,
                               493.88];


    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();

    for i in notes {
        let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
        for n in 0..wave_table_size {
            wave_table.push((2.0 * std::f32::consts::PI * n as f32 / wave_table_size as f32).sin());
        }
        let mut oscilator = WaveTableOscilator::new(44100, wave_table);
        oscilator.set_frequency(i);
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let _result = stream_handle.play_raw(oscilator.convert_samples());
        std::thread::sleep(Duration::from_millis(80));


        let mut second_wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);

        for n in ( 0..4000).map(|x| x as f32 / 44100.0) {
            let sample = (2.0 * std::f32::consts::PI * n * i).sin();
            let amplitude = i16::MAX as f32;
            writer.write_sample((sample * amplitude) as i16).unwrap();
        }

        // for t in (0..44100).map(|x| x as f32 / 44100.0) {
        //     let sample = (i * 440.0 * 2.0 * PI).sin();
        // }
    }
}
