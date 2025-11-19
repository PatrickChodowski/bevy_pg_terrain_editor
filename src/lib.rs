pub mod planes;
pub mod vertex;
pub mod terrain_brushes;

pub mod prelude {
    pub use crate::planes::{PlaneToEdit, plane_mesh};
    pub use crate::vertex::{SpawnVertices, SelectedVertex, PlaneVertex, TerrainEditorVertexPlugin, TerrainVertexController, terrain_vertex_controller};
    pub use crate::terrain_brushes::{TerrainHeightBrush, TerrainColorBrush, HeightBrushType};
}