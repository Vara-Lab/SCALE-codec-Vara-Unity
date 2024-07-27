#![no_std]
use gstd::{async_main, msg, prelude::*};
use io::*;

// 1. Create the main state as a static variable.
static mut STATE: Option<CustomStruct> = None;

// Create a public State
#[derive(Clone, Default)]
pub struct CustomStruct {
    pub id:u32,
    pub active:bool,
    pub description:String,
    pub power:u128
}

// Create a implementation on State
impl CustomStruct {
    fn firstmethod(&mut self,input:u32) -> Result<Events, Errors> {
       
        self.id = input.clone();


        Ok(Events::FirstEvent)
    }

    async fn secondmethod(&mut self, input: bool) -> Result<Events, Errors> {
        // Update your state with a String input
        self.active = input.clone();

        Ok(Events::FirstEvent)
    }

    async fn thirdmethod(&mut self, input: String) -> Result<Events, Errors> {
        // Update your state with a u128 input
        self.description = input;

        Ok(Events::FirstEvent)
    }

    async fn fourthmethod(&mut self, input: u128) -> Result<Events, Errors> {
        
         // Update your state with a String input
         self.power = input.clone();

         Ok(Events::FirstEvent)
    }

    
}

// 3. Create the init() function of your contract.
#[no_mangle]
extern "C" fn init() {
    let config: InitStruct = msg::load().expect("Unable to decode InitStruct");

    if config.ft_program_id.is_zero() {
        panic!("InitStruct program address can't be 0");
    }

    let state = CustomStruct {
        ..Default::default()
    };

    unsafe { STATE = Some(state) };
}

// 4.Create the main() function of your contract.
#[async_main]
async fn main() {
    // We load the input message
    let action: Actions = msg::load().expect("Could not load Action");

    let state: &mut CustomStruct =
        unsafe { STATE.as_mut().expect("The contract is not initialized") };

    // We receive an action from the user and update the state. Example:
    let reply = match action {
    
        Actions::Id(input) => state.firstmethod(input), // Here, we call the implementation
        Actions::Active(input) => state.secondmethod(input).await, // Here, we call the implementation
        Actions::Description(input) => state.thirdmethod(input).await, // Here, we call the implementation
        Actions::Power(input) => state.fourthmethod(input).await, // Here, we call the implementation
    };
    msg::reply(reply, 0).expect("Error in sending a reply");
}

// 5. Create the state() function of your contract.
#[no_mangle]
extern "C" fn state() {
    let state = unsafe { STATE.take().expect("Unexpected error in taking state") };
    
    msg::reply::<IoCustomStruct>(state.into(), 0).expect("Error on sharinf state");
}

// Implementation of the From trait for converting CustomStruct to IoCustomStruct
impl From<CustomStruct> for IoCustomStruct {
    // Conversion method
    fn from(value: CustomStruct) -> Self {
        // Destructure the CustomStruct object into its individual fields
        let CustomStruct {
            id,
    active,
    description,
    power
} = value;


        // Create a new IoCustomStruct object using the destructured fields
        Self {
            id,
            active,
         description,
    power
        }
    }

}