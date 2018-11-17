use amethyst::{
    ecs::{Component, DenseVecStorage},
    renderer::SpriteRender,
};

pub struct Animation {
    pub total_frames: usize,
    pub max_count_till_next_frame: u32,
    pub frame_life_time_count: u32,
    pub current_frame: usize,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            total_frames: 0,
            max_count_till_next_frame: 0,
            frame_life_time_count: 0,
            current_frame: 0,
        }
    }
}

impl Component for Animation {
    type Storage = DenseVecStorage<Self>;
}

impl Animation {

    pub fn frame_update(&mut self, sprite_render: &mut SpriteRender) {
        if self.frame_life_time_count > 0 {
            self.frame_life_time_count = self.frame_life_time_count - 1;
        } else {
            self.frame_life_time_count = self.max_count_till_next_frame;
            self.current_frame = (self.current_frame + 1) % self.total_frames;
        }

        sprite_render.sprite_number = self.current_frame;
    }
}
