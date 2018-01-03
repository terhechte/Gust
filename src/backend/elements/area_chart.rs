/* Author: Samuel Resendez
 * Contains most of the needed elements for
 * area_charts
 */

use backend::elements::general::*;


use serde::ser::{Serializer, SerializeStruct, Serialize};

#[derive(Serialize)]
pub struct AreaChartSignal {
    name: String,
    value: String,
    bind: SignalBinding,
}

impl AreaChartSignal {
    pub fn default() -> AreaChartSignal {
        AreaChartSignal {
            name: String::from("interpolate"),
            value: String::from("monotone"),
            bind: SignalBinding::default(),
        }
    }
}
#[derive(Serialize)]
pub struct SignalBinding {
    input: String,
    options: Vec<String>,
}
impl SignalBinding {
    pub fn default() -> SignalBinding {
        SignalBinding {
            input: String::from("select"),
            options: vec![
                String::from("basis"),
                String::from("cardinal"),
                String::from("linear"),
                String::from("monotone"),
            ],
        }
    }
}
#[derive(Serialize)]
pub struct AreaChartData {
    name: String,
    values: Vec<AreaChartDataEntry>,
}
#[derive(Serialize)]
pub struct AreaChartDataEntry {
    u: i32,
    v: i32,
}


#[derive(Serialize)]
pub struct AreaChartAxis {
    orient: Orientation,
    scale: String,
}
pub struct AreaChartMark {
    mark_type: String,
    from: String,
    encode: AreaChartEncoding,
}
#[derive(Serialize)]
pub struct AreaChartEncoding {
    enter: AreaChartEnter,
    update: AreaChartUpdate,
    hover: AreaChartHover,
}
#[derive(Serialize)]
pub struct AreaChartEnter {
    x: JSONDict,
    y: JSONDict,
    y2: JSONDict,
    fill: JSONDict,
}
#[derive(Serialize)]
pub struct AreaChartUpdate {
    interpolate: KeyVal,
    fillOpacity: QualKeyVal,
}

#[derive(Serialize)]
#[ignore(warn_snake_case)]
pub struct AreaChartHover {
    fillOpacity: QualKeyVal,
}

pub struct AreaChartScale {
    name: String,
    scale_type: String,
    range: String,
    zero: bool,
    domain: JSONDict,
}

impl AreaChartScale {
    pub fn default_x() -> AreaChartScale {
        AreaChartScale {
            name: String::from("xscale"),
            scale_type: String::from("linear"),
            zero: false,
            range: String::from("width"),
            domain: JSONDict::create("data", "table", "field", "u"),
        }

    }
    pub fn default_y() -> AreaChartScale {
        AreaChartScale {
            name: String::from("yscale"),
            scale_type: String::from("linear"),
            zero: true,
            range: String::from("height"),
            domain: JSONDict::create("data", "table", "field", "v"),
        }

    }
}


impl Serialize for AreaChartScale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("area_chart", 10)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("type", &self.scale_type)?;
        s.serialize_field("range", &self.range)?;
        s.serialize_field("zero", &self.zero)?;
        s.serialize_field("domain", &self.domain)?;

        s.end()

    }
}