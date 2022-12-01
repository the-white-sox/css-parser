use super::*;
use crate::tokenizer::*;

mod media_feature;
mod media_type;

use media_feature::*;
use media_type::*;

pub enum MediaQuery {
    MediaType(MediaType),
    MediaFeature(MediaFeature),
    Not(Box<MediaQuery>),
    And(Box<MediaQuery>, Box<MediaQuery>),
    Or(Box<MediaQuery>, Box<MediaQuery>),
}
