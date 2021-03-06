use crate::worldgen::util::shuffle_seed_for_chunk;
use crate::worldgen::{ChunkBiomes, FinishingGenerator, TopBlocks};
use feather_blocks::Block;
use feather_core::{Biome, Chunk};
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::{cmp, iter};

/// Clumped foliage generator.
#[derive(Default)]
pub struct ClumpedFoliageFinisher;

impl FinishingGenerator for ClumpedFoliageFinisher {
    fn generate_for_chunk(
        &self,
        chunk: &mut Chunk,
        biomes: &ChunkBiomes,
        top_blocks: &TopBlocks,
        seed: u64,
    ) {
        // Generate clumps of foliage for the biome.
        // Note that we currently use a hack
        // to ensure that clumps are within one
        // chunk.
        // The algorithm should be changed in the future
        // to allow for cross-chunk clumps.

        let mut rng = XorShiftRng::seed_from_u64(shuffle_seed_for_chunk(seed, chunk.position()));

        for x in 0..16 {
            for z in 0..16 {
                let biome = biomes.biome_at(x, z);

                if let Some(block) = biome_clump_block(biome) {
                    if rng.gen_range(0, 48) == 0 {
                        // Generate clump with center at this position.
                        iter::repeat(()).take(rng.gen_range(3, 6)).for_each(|_| {
                            let offset_x = rng.gen_range(-2, 3);
                            let offset_z = rng.gen_range(-2, 3);

                            // Clamp value within chunk border
                            let pos_x = cmp::max(0, cmp::min(x as i32 + offset_x, 15)) as usize;
                            let pos_z = cmp::max(0, cmp::min(z as i32 + offset_z, 15)) as usize;

                            if chunk.biome_at(pos_x, pos_z) != biome {
                                return; // Don't generate block outside this biome
                            }

                            let top = top_blocks.top_block_at(pos_x, pos_z);
                            chunk.set_block_at(pos_x, top + 1, pos_z, block);
                        });
                    }
                }
            }
        }
    }
}

fn biome_clump_block(biome: Biome) -> Option<Block> {
    match biome {
        Biome::Plains
        | Biome::SunflowerPlains
        | Biome::WoodedMountains
        | Biome::Mountains
        | Biome::Savanna
        | Biome::SavannaPlateau
        | Biome::Forest
        | Biome::DarkForest
        | Biome::DarkForestHills
        | Biome::BirchForest
        | Biome::TallBirchForest
        | Biome::BirchForestHills
        | Biome::Swamp => Some(Block::Grass),
        _ => None,
    }
}
