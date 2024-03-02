use crate::models::Todo;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use uuid::Uuid;

pub type Cache = Arc<RwLock<HashMap<Uuid, Todo>>>;
