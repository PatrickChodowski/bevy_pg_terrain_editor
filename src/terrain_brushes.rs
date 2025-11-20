use bevy::prelude::*;
use bevy_pg_editor_tools::prelude::BrushType;
use bevy::ecs::system::SystemState;

use crate::prelude::{PlaneVertex, SelectedVertex, Noise};

#[derive(Clone)]
pub struct Terrace {
    pub min: f32,
    pub max: f32,
    pub value: f32
}
impl Terrace {
    fn new() -> Terrace {
        Terrace { min: 0.0, max: 1.0, value: 0.5}
    }
}

#[derive(Clone)]
pub enum HeightBrushType {
    Value(f32),
    Terraces(Vec<Terrace>),
    Noise((Vec<Noise>, f32))
}

#[derive(Clone)]
pub struct TerrainHeightBrush {
    pub typ: HeightBrushType 
}

impl BrushType for TerrainHeightBrush {
    fn apply(&mut self, world: &mut World, loc: Vec3, radius: f32) {

        let mut system_state: SystemState<(
            Commands,
            Query<(Entity, &mut PlaneVertex, &mut Transform, &GlobalTransform, Option<&SelectedVertex>)>
        )> = SystemState::new(world);
        let (mut commands, mut plane_vertices) = system_state.get_mut(world);

        for (vertex_entity, mut plane_vertex, mut vertex_transform, global_transform, maybe_selected) in plane_vertices.iter_mut(){

            let global_loc = global_transform.translation();
            let near: bool = global_loc.xz().distance(loc.xz()) <= (radius + plane_vertex.radius);

            if near & maybe_selected.is_none() {
                commands.entity(vertex_entity).insert(SelectedVertex);

                match &self.typ {
                    HeightBrushType::Value(y) => {
                        vertex_transform.translation.y += y;
                    }
                    HeightBrushType::Terraces(terraces) => {
                        for terrace in terraces {
                            if vertex_transform.translation.y >= terrace.min && vertex_transform.translation.y <= terrace.max {
                                vertex_transform.translation.y = terrace.value;
                                continue;
                            }
                        }
                    }
                    HeightBrushType::Noise(noises) => {
                        let mut combined_noise: f32 = 0.0;
                        for noise in noises.0.iter(){
                            let noise_value = noise.apply(global_loc);
                            combined_noise += noise_value;
                        }
                        let new_y: f32 = combined_noise*noises.1;
                        vertex_transform.translation.y = new_y;
                    }
                }

                plane_vertex.loc = vertex_transform.translation.into();
            } else if  near & maybe_selected.is_some() {

            } else {
                // commands.entity(vertex_entity).remove::<SelectedVertex>();
            }
        }
        system_state.apply(world);
    }
    fn done(&mut self, world: &mut World) {
        let mut system_state: SystemState<(
            Commands,
            Query<Entity, With<SelectedVertex>>
        )> = SystemState::new(world);
        let (mut commands, plane_vertices) = system_state.get_mut(world);
        for entity in plane_vertices.iter(){
            commands.entity(entity).remove::<SelectedVertex>();
        }
        system_state.apply(world);
    }
    fn started(&mut self, world: &mut World) {}
}

#[derive(Clone)]
pub struct TerrainColorBrush {
    pub color: [f32;4]
}

impl BrushType for TerrainColorBrush {
    fn apply(&mut self, world: &mut World, loc: Vec3, radius: f32) {
        let mut system_state: SystemState<(
            Commands,
            Query<(Entity, &mut PlaneVertex, &GlobalTransform, Option<&SelectedVertex>)>
        )> = SystemState::new(world);
        let (mut commands, mut plane_vertices) = system_state.get_mut(world);

        for (vertex_entity, mut plane_vertex, global_transform, maybe_selected) in plane_vertices.iter_mut(){

            let global_loc = global_transform.translation();
            let near: bool = global_loc.xz().distance(loc.xz()) <= (radius + plane_vertex.radius);

            if (global_loc.xz().distance(loc.xz()) <= radius + plane_vertex.radius) & maybe_selected.is_none() {
                commands.entity(vertex_entity).insert(SelectedVertex);
                plane_vertex.clr = self.color;
            } else if  near & maybe_selected.is_some() {

            } else {
                commands.entity(vertex_entity).remove::<SelectedVertex>();
            }
        }
        system_state.apply(world);
    }
    fn done(&mut self, world: &mut World) {

        let mut system_state: SystemState<(
            Commands,
            Query<Entity, With<SelectedVertex>>
        )> = SystemState::new(world);
        let (mut commands, plane_vertices) = system_state.get_mut(world);
        for entity in plane_vertices.iter(){
            commands.entity(entity).remove::<SelectedVertex>();
        }
        system_state.apply(world);

    }
    fn started(&mut self, world: &mut World) {}
}
