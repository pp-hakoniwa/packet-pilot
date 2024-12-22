use std::sync::Arc;
use crate::PhysicalLayerFrame;

// Callback function type -------------------------------------
pub type PhysicalLayerCallback    = Arc<dyn Fn(PhysicalLayerFrame) + Send + Sync>;