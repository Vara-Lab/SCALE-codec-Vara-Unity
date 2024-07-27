#![no_std]

use gmeta::{In, Out, InOut, Metadata};
use gstd::{prelude::*, ActorId};

pub struct ProgramMetadata;

// 1. Define actions, events, errors and state for your metadata.
impl Metadata for ProgramMetadata {
    type Init = In<InitStruct>;
    type Handle = InOut<Actions, Result<Events, Errors>>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Out<IoCustomStruct>;
}

// 2. Create your init Struct(Optional)
#[derive(Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitStruct {
    // Example:
    pub ft_program_id: ActorId,
}

// 3. Create your own Actions
#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Actions {
    Id(u32),
    Active(bool),
    Description(String),
    Power(u128)
    
}

// Example of a custom input for an action
#[derive(Debug, Decode, Encode,  Clone, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct CustomInput {
   pub firstfield: String,
   pub secondfield: u128,
   pub thirdfield: ActorId,
}

// 4. Create your own Events
#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Events {
    // Add Events(Example)
    FirstEvent,          // Example an event with a simple input
    SecondEvent(String), // Example an event with a u128 input
    ThirdEvent(u128),    // Example an event with String input
    FourtEvent {
        first_field: ActorId,
        second_field: Vec<ActorId>, // Example an event with a custom input
    },
}

// 5. Create your own Errors
#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Errors {
    // Add errors(Example)
    FirstError,
    SecondError,
    ThirdErrors,
    FourtErrors,
}

// 6. Create your own Struct
#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct IoCustomStruct {
    pub id:u32,
    pub active:bool,
    pub description:String,
    pub power:u128
}


