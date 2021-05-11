use std::collections::HashMap;
use std::io::{BufWriter, Write};
use std::error::Error;
use std::hash::{Hasher, Hash};
use metrohash::MetroHash;

include!("elements.rs");
include!("region.rs");
