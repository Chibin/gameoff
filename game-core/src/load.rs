use amethyst::{
    animation::{Animation, InterpolationFunction, MaterialChannel, MaterialPrimitive, Sampler},
    assets::{AssetStorage, Handle, Loader},
    prelude::*,
    renderer::{
        Material, MaterialTextureSet, PngFormat, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture,
        TextureMetadata,
    },
};
use std::collections::HashMap;

#[derive(Default)]
pub struct LoadedTextures {
    pub textures: HashMap<String, SpriteSheetHandle>,
}

pub fn sprite_sheet(world: &mut World, png_path: &str, ron_path: &str) -> SpriteSheetHandle {
    let texture_id = super::load::texture(world, png_path);

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    let handle = loader.load(
        ron_path,
        SpriteSheetFormat,
        texture_id,
        (),
        &sprite_sheet_store,
    );

    let mut my = world.write_resource::<LoadedTextures>();
    let old_val = my.textures.insert(png_path.into(), handle.clone());
    assert!(old_val.is_none());

    handle
}

/// Loads texture into world and returns texture id.
pub fn texture(world: &mut World, png_path: &str) -> u64 {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            png_path,
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let mut material_texture_set = world.write_resource::<MaterialTextureSet>();
    let texture_id = material_texture_set.len() as u64;
    material_texture_set.insert(texture_id, texture_handle);

    texture_id
}

/// Sprite animations
pub fn penguin_animation(
    world: &mut World, png_path: &str, ron_path: &str) -> Handle<Animation<Material>> {

    let sprite_sheet_handle = sprite_sheet(world, png_path, ron_path);

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
