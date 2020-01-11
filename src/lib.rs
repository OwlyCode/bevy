mod app;
pub mod asset;
mod core;
pub mod render;
mod vertex;

pub use crate::core::*;
pub use app::{App, AppBuilder, AppStage};

pub use glam as math;
pub use legion;
pub use legion::prelude::*;
pub use legion::schedule::Schedulable;
pub use legion_transform;
pub use legion_transform::prelude::*;
pub use legion_transform::transform_system_bundle;
pub use wgpu;
