use std::hash::{Hash, Hasher};

use metrohash::MetroHash;
use std::borrow::{BorrowMut, Borrow};
use std::sync::{Arc, Mutex, MutexGuard};
use std::ops::{Deref, DerefMut};
use std::cell::{RefCell, RefMut, Cell};
use std::rc::{Rc, Weak};
use std::marker::PhantomData;

include!("builder.rs");
include!("dom.rs");
include!("elements.rs");
include!("region.rs");
