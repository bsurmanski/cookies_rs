use rand::random;
use sdl2::mixer::*;

macro_rules! include_wav {
    ($x:literal) => {
        sdl2::rwops::RWops::from_bytes(include_bytes!($x).as_ref()).unwrap().load_wav().unwrap()
    };
}

pub struct MusicContext {
    boop1: Chunk,
    boop2: Chunk,
    boop3: Chunk,
    boop4: Chunk,
    boop5: Chunk,
    bonk1: Chunk,
    timer: f32,
    hit_count: i8,
}

impl MusicContext {
    pub fn new() -> MusicContext {
        MusicContext {
            boop1: include_wav!("../res/music/boop1.wav"),
            boop2: include_wav!("../res/music/boop2.wav"),
            boop3: include_wav!("../res/music/boop3.wav"),
            boop4: include_wav!("../res/music/boop4.wav"),
            boop5: include_wav!("../res/music/boop5.wav"),
            bonk1: include_wav!("../res/music/bonk1.wav"),
            timer: 0.0,
            hit_count: 1,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.timer -= dt;
        if self.timer < 0.0 {
            self.hit_count += 1;
            self.timer = 0.5;
            if self.hit_count % 2 == 0 {
                self.hit_count -= 2;
                sdl2::mixer::Channel(-1).play_timed(&self.bonk1, 0, -1).unwrap();
            } else {
                let r = random::<f32>() * 5.0;
                if r < 1.0 {
                    sdl2::mixer::Channel(-1).play_timed(&self.boop1, 0, -1).unwrap();
                } else if r < 2.0 {
                    sdl2::mixer::Channel(-1).play_timed(&self.boop2, 0, -1).unwrap();
                } else if r < 3.0 {
                    sdl2::mixer::Channel(-1).play_timed(&self.boop3, 0, -1).unwrap();
                } else if r < 4.0 {
                    sdl2::mixer::Channel(-1).play_timed(&self.boop4, 0, -1).unwrap();
                } else {
                    sdl2::mixer::Channel(-1).play_timed(&self.boop5, 0, -1).unwrap();
                }
            }
        }
    }
}
