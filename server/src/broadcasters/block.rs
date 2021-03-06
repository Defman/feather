//! Broadcasting of block updates, i.e. when a block is changed to another.

use crate::game::Game;
use feather_core::network::packet::implementation::BlockChange;
use feather_core::{Block, BlockExt, BlockPosition};
use fecs::World;

/// System for broadcasting block update
/// events to all clients.
pub fn on_block_update_broadcast(
    game: &mut Game,
    world: &mut World,
    pos: BlockPosition,
    new_block: Block,
) {
    // Broadcast Block Change packet.
    let packet = BlockChange {
        location: pos,
        block_id: new_block.native_state_id() as i32,
    };
    game.broadcast_chunk_update(world, packet, pos.into(), None);
}
