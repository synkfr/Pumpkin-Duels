use pumpkin_plugin_api::common::BlockPos;
use pumpkin_plugin_api::server::Dimension;
use pumpkin_plugin_api::world::BlockFlags;
use pumpkin_plugin_api::worldgen::{ChunkBuffer, ChunkGenerator, GeneratorManager, PluginBiome};
use pumpkin_plugin_api::{Server, World};
use std::sync::OnceLock;

static VOID_GENERATOR_ID: OnceLock<u32> = OnceLock::new();

struct VoidGenerator;

impl ChunkGenerator for VoidGenerator {
    fn generate_biomes(&self, chunk: &mut ChunkBuffer) {
        chunk.fill_biome(PluginBiome::TheVoid);
    }

    fn generate_surface(&self, chunk: &mut ChunkBuffer) {
        if chunk.x().abs() <= 1 && chunk.z().abs() <= 1 {
            chunk.fill_cuboid(0, 64, 0, 15, 64, 15, 1);
        }
    }
}

pub fn init_void_generator() -> u32 {
    *VOID_GENERATOR_ID.get_or_init(|| GeneratorManager::register(VoidGenerator))
}

pub fn create_arena_void_world(
    server: &Server,
    arena_name: &str,
    dimension: Dimension,
) -> (String, World) {
    let gen_id = init_void_generator();
    let world_name = if arena_name.starts_with("arena_") {
        arena_name.to_string()
    } else {
        format!("arena_{arena_name}")
    };

    let world = server.create_world(&world_name, dimension);
    world.set_chunk_generator(gen_id);

    for x in -10..=10 {
        for z in -10..=10 {
            let _ = world.set_block_by_name(
                BlockPos { x, y: 64, z },
                "minecraft:stone",
                BlockFlags::empty(),
            );
        }
    }

    (world_name, world)
}
