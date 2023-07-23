use crate::models::Todo;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use uuid::Uuid;

pub type Db = Arc<RwLock<HashMap<Uuid, Todo>>>;
