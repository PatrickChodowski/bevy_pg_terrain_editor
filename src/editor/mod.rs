use bevy::prelude::*;

pub mod terrain_brushes;
pub mod planes;
pub mod vertex;

use crate::editor::vertex::TerrainEditorVertexPlugin;

pub struct PGTerrainEditorToolsPlugin;

impl Plugin for PGTerrainEditorToolsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(TerrainEditorVertexPlugin)
        ;
    }
}

