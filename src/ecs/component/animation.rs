pub struct Animation {
    pub initial: f64,
    pub speed: f64
}

impl Animation {
    pub fn new() -> Animation {
        Animation { initial: 0.0, speed: 0.0 }
    }

    pub fn set_speed(&self, speed: f64) -> Animation {
        Animation { initial: 0.0, speed }
    }

    pub fn play(&mut self) {
        self.initial += self.speed;
        if self.initial == 360.0 {
            self.initial = 0.0;
        }
    }
}