//! socket device module

use crate::base::Id;

/// common configure of socket device
pub trait VZSocketDeviceConfiguration {
    fn id(&self) -> Id;
}
