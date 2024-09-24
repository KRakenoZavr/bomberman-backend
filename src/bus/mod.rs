use std::{collections::HashMap, sync::Arc};

use bus::Bus;
use tokio::sync::Mutex;

pub type BusMap<T> = Arc<Mutex<HashMap<u8, Bus<T>>>>;
