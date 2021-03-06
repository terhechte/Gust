
use std::collections::HashMap;
use serde::ser::{Serialize, Serializer, SerializeStruct};


/// Orientation is an enum for the different possible axis values
#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Orientation {
    Top,
    Left,
    Bottom,
    Right,
}

/// A small helper struct that represents a json object of the following type:
/// { key:  val}, where both key and val are strings (&str, and String)
pub struct KeyVal {
    pub key: &'static str,
    pub val: String,
}
impl KeyVal {
    pub fn new(key: &'static str, val: &str) -> KeyVal {
        KeyVal {
            key: key,
            val: val.to_string(),
        }
    }
}

impl Serialize for KeyVal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("keyval", 1)?;
        s.serialize_field(&self.key, &self.val)?;
        s.end()
    }
}

///
/// QualKeyVal is very similar to KeyVal, except it stores a float32 instead of a string
/// as it's value: {key, val} (&str, f32)
pub struct QualKeyVal {
    key: &'static str,
    val: f32,
}

impl QualKeyVal {
    pub fn new(key: &'static str, val: f32) -> QualKeyVal {
        QualKeyVal { key, val }
    }
}
impl Serialize for QualKeyVal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("qualkey", 1)?;
        s.serialize_field(self.key, &self.val)?;
        s.end()
    }
}

///JSONDict is a helper structure which represents an arbitrary json dictionary of the following form:
/// `
/// {
///     foo: bar,
///     baz: foobar,
///     ...
///  }
/// `
/// It can hold an arbitrary quantity of these, and serializes appropriately. The helper functions provided
/// are mostly convenience functions for some of the common types of json dictionaries which show up in the Vega
/// specification
pub struct JSONDict {
    str_vals: HashMap<&'static str, String>,
    i32_vals: HashMap<&'static str, i32>,
}

impl JSONDict {
    /// create creates a general, 4 string json object that backs a lot of different parts of the
    /// Vega specification. Since Serialize really only accepts hardcoded keys, this causes some
    /// limitations as to the implementation of JSONDict
    pub fn create(x_key: &'static str, x_val: &str, y_key: &'static str, y_val: &str) -> JSONDict {
        let mut d = JSONDict {
            str_vals: HashMap::new(),
            i32_vals: HashMap::new(),
        };
        d.str_vals.insert(x_key, x_val.to_string());
        d.str_vals.insert(y_key, y_val.to_string());
        d
    }

    /// band_create adds one tuple of type (String, String), and another of type (String, i32),
    /// that get serialized. often times bands and other values that describe a visualization show up
    /// and use this structure, hence a dedicated constructor to make it easier.
    pub fn band_create(
        x_key: &'static str,
        x_val: &str,
        y_key: &'static str,
        y_val: i32,
    ) -> JSONDict {
        let mut d = JSONDict {
            str_vals: HashMap::new(),
            i32_vals: HashMap::new(),
        };
        d.str_vals.insert(x_key, x_val.to_string());
        d.i32_vals.insert(y_key, y_val);
        d
    }

    pub fn tri_create(
        x_key: &'static str,
        x_val: &str,
        y_key: &'static str,
        y_val: i32,
        z_key: &'static str,
        z_val: i32,
    ) -> JSONDict {
        let mut d = JSONDict {
            str_vals: HashMap::new(),
            i32_vals: HashMap::new(),
        };
        d.str_vals.insert(x_key, String::from(x_val));
        d.i32_vals.insert(y_key, y_val);
        d.i32_vals.insert(z_key, z_val);
        d

    }
}

impl Serialize for JSONDict {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("json_dict", 10)?;
        for (k, v) in &self.str_vals {
            s.serialize_field(k, &v)?;
        }
        for (k, v) in &self.i32_vals {
            s.serialize_field(k, &v)?;
        }
        s.end()
    }
}
