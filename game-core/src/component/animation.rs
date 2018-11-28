use amethyst::{
    animation::{Animation, InterpolationFunction, MaterialChannel, MaterialPrimitive, Sampler},
    assets::{AssetStorage, Handle, Loader},
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Material, SpriteRender, SpriteSheetHandle, SpriteSheet},
};

pub struct AAnimation {
    pub total_frames: usize,
    pub max_count_till_next_frame: f32, // These are in seconds
    pub frame_life_time_count: f32,     // These are in seconds
    pub current_frame: usize,
}

impl Default for AAnimation {
    fn default() -> Self {
        Self {
            total_frames: 0,
            max_count_till_next_frame: 0.0,
            frame_life_time_count: 0.0,
            current_frame: 0,
        }
    }
}

impl Component for AAnimation {
    type Storage = DenseVecStorage<Self>;
}

impl AAnimation {
    pub fn frame_update(&mut self, sprite_render: &mut SpriteRender, seconds: f32) {
        if self.frame_life_time_count > 0.0 {
            self.frame_life_time_count -= seconds;
        } else {
            self.frame_life_time_count = self.max_count_till_next_frame;
            self.current_frame = (self.current_frame + 1) % self.total_frames;
        }

        sprite_render.sprite_number = self.current_frame;
    }
}

pub fn penguin_animation(
    world: &mut World,
    sprite_sheet_handle: &SpriteSheetHandle,
) -> Handle<Animation<Material>> {

    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    let something = sprite_sheet_store.get(&sprite_sheet_handle);
    let sprite_sheet = something.unwrap();

    let sprite_offsets = sprite_sheet.sprites[0..1]
        .iter()
        .map(|sprite| sprite.into())
        .collect::<Vec<MaterialPrimitive>>();

    let sprite_offset_sampler = {
        Sampler {
            input: vec![0.],
            function: InterpolationFunction::Step,
            output: sprite_offsets,
        }
    };

    let texture_sampler = Sampler {
        input: vec![0.],
        function: InterpolationFunction::Step,
        output: vec![MaterialPrimitive::Texture(sprite_sheet.texture_id)],
    };

    let loader = world.write_resource::<Loader>();
    let sampler_animation_handle =
        loader.load_from_data(sprite_offset_sampler, (), &world.read_resource());
    let texture_animation_handle =
        loader.load_from_data(texture_sampler, (), &world.read_resource());

    let animation = Animation {
        nodes: vec![
            (0, MaterialChannel::AlbedoTexture, texture_animation_handle),
            (0, MaterialChannel::AlbedoOffset, sampler_animation_handle),
        ],
    };

    loader.load_from_data(animation, (), &world.read_resource())
}
