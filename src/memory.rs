//! Provides a representation for memory across different frameworks.
//!
//! Memory is allocated by a device in a way that it is accessible for its computations.
//!
//! Normally you will want to use [SharedTensor][tensor] which handles synchronization
//! of the latest memory copy to the required device.
//!
//! [tensor]: ../tensor/index.html

/// Specifies Memory behavior accross frameworks.
pub trait Memory { }
