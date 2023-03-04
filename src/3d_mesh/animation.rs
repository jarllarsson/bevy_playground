use bevy::{prelude::*};

// Various animation help

pub struct AnimationPlugin;

// Component containing a link to an animation-player entity. Used for owning
// top level entity to be able to reference a child animation player entity to control
// the animations on a per-entity basis.
#[derive(Component)]
pub struct AnimationLink(pub Entity);

impl Plugin for AnimationPlugin{
    fn build(&self, app: &mut App) {
        app.add_system(animation_link_setup);
    }
}

fn find_top_parent (mut curr_entity: Entity, parent_query: &Query<&Parent>) -> Entity { 
    while let Ok(parent) = parent_query.get(curr_entity) { // fetch parent of current entity
        curr_entity = parent.get();
    }
    curr_entity
}

// When a new animation player is added, link to it in its topmost parent entity
fn animation_link_setup(
    added_anim_query: Query<Entity, Added<AnimationPlayer>>, // Newly added anim players
    parent_query: Query<&Parent>,                            // All parent entities
    existing_anim_links_query: Query<&AnimationLink>,  // All existing anim links
    mut commands: Commands,
){  
    for anim_entity in added_anim_query.iter() {
        let top_parent = find_top_parent(anim_entity, &parent_query);
        if existing_anim_links_query.get(top_parent).is_ok() {
            warn!("Linking multiple animation players to one top entity currently not supported!");
        } else {
            commands.entity(top_parent).insert(AnimationLink(anim_entity));
        }
    }
}
