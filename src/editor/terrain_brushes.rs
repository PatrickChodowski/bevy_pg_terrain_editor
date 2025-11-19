use bevy::prelude::*;
use bevy_pg_editor_tools::prelude::BrushType;
use bevy::ecs::system::SystemState;

use crate::prelude::{PlaneVertex, SelectedVertex};

#[derive(Clone)]
pub struct TerrainHeightBrush {
    pub y_change: f32 
}

impl BrushType for TerrainHeightBrush {
    fn apply(&mut self, world: &mut World, loc: Vec3, radius: f32) {
        let mut system_state: SystemState<(
            Commands,
            Query<(Entity, &PlaneVertex, &mut Transform), Without<SelectedVertex>>
        )> = SystemState::new(world);
        let (mut commands, mut plane_vertices) = system_state.get_mut(world);
        for (vertex_entity, plane_vertex, mut vertex_transform) in plane_vertices.iter_mut(){
            if vertex_transform.translation.xz().distance(loc.xz()) <= radius + plane_vertex.radius {
                commands.entity(vertex_entity).insert(SelectedVertex);
                vertex_transform.translation.y += self.y_change;
            }
        }
        system_state.apply(world);
    }
    fn done(&mut self, world: &mut World) {}
    fn started(&mut self, world: &mut World) {}
}
