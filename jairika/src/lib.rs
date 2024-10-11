// Copyright 2024 Koseka
// SPDX-License-Identifier: Apache-2.0

//! This library is the core of Jairika, a blazing fast cross-language machine learning framework.
//! Developers can use this library to build and train machine learning models that are backed by a
//! SurrealKV instance. The library is designed to be simple and easy to use, with a focus on
//! performance and scalability.
//!
//! ## Features
//!
//! - **Simple API**: The library provides a simple and easy-to-use API that allows developers to
//!

mod api;
mod error;
mod storage;

pub use crate::api::{init, Model};
pub(crate) use crate::error::{Error, Result};
pub(crate) use crate::storage::Store;
