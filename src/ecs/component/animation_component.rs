pub struct AnimationComponent {
    pub initial: f64,
    pub speed: f64
}

impl AnimationComponent {
    pub fn new() -> AnimationComponent {
        AnimationComponent { initial: 0.0, speed: 0.0 }
    }

    pub fn set_speed(&self, speed: f64) -> AnimationComponent {
        AnimationComponent { initial: 0.0, speed }
    }

    pub fn play(&mut self) {
        self.initial += self.speed;
        if self.initial == 360.0 {
            self.initial = 0.0;
        }
    }
}