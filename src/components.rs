use bevy::prelude::*;


#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);


#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component)]
pub struct CursorTool {
    pub first: usize,
    pub last: usize,
}

#[derive(Component)]
pub struct MainCamera;


#[derive(Default, Component, Reflect)]
pub struct Player {
    pub speed: f32,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub stick_pos: Vec2,
    pub deadzone: f32,
    
}

impl Player {
    pub fn left(&self) -> bool {
        return self.left || self.stick_pos.x < -self.deadzone;
    }
    pub fn right(&self) -> bool {
        return self.right || self.stick_pos.x > self.deadzone;
    }
    pub fn up(&self) -> bool {
        return self.up || self.stick_pos.y > self.deadzone;
    }
    pub fn down(&self) -> bool {
        return self.down || self.stick_pos.y < -self.deadzone;
    }
}


#[derive(Default, Component, Reflect)]
pub struct Car {
    pub speed: f32
}

#[derive(Default, Component, Reflect)]
pub struct Traffic {
    pub time_since_last_car: f32,
    pub next_car_at: f32
}

#[derive(Resource)]
pub struct SpriteSheetHandles {
    pub cars: Handle<TextureAtlas>
}