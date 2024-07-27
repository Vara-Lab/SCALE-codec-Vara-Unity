use io::*;
use gstd::prelude::*;
use primitive_types::U256;

#[gmeta::metawasm]
pub mod metafns {
    pub type State = IoCustomStruct;

    pub fn info(state: IoCustomStruct) ->IoCustomStruct {
       state
    }

}