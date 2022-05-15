
/// Importing the `IntoEnumIterator` trait from the `enum_iterator` crate.
use enum_iterator::IntoEnumIterator;

/// Importing the `Deserialize` and `Serialize` traits from the `serde` crate.
use serde::{Deserialize, Serialize};

/// A macro that is used to automatically generate code for the `Action` enum.
#[derive(IntoEnumIterator, Copy, Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]

/// Telling the compiler that the enum should be represented as a u8.
#[repr(u8)]

/// Creating an enum called Action.
pub enum Action {

   /// Telling the compiler that the enum should be represented as a u8.
    #[serde(rename = "*")]

    All = 0, 

   /// Telling the compiler that the enum should be represented as a u8.
    #[serde(rename = "search")]

    /// Telling the compiler that the enum should be represented as a u8.
    Search = actions::SEARCH, 

   /// Renaming the function to documents.add
    #[serde(rename = "documents.add")]

    /// Telling the compiler that the enum should be represented as a u8.
    DocumentsAdd = actions::DOCUMENTS_ADD,

   /// Renaming the function to documents.get
    #[serde(rename = "documents.get")]

   /// Telling the compiler that the enum should be represented as a u8.
    DocumentsGet = actions::DOCUMENTS_GET,

  /// Renaming the function to documents.delete
    #[serde(rename = "documents.delete")]

    /// Telling the compiler that the enum should be represented as a u8.
    DocumentsDelete = actions::DOCUMENTS_DELETE, 











    
}


