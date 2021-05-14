use std::hash::{Hash, Hasher};

use metrohash::MetroHash;
use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex, MutexGuard};
use std::ops::{Deref, DerefMut};
use std::cell::{RefCell, RefMut, Cell};
use std::rc::{Rc, Weak};

include!("builder.rs");
include!("dom.rs");
include!("elements.rs");
include!("region.rs");
