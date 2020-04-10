
#[macro_use]
extern crate wascc_codec as codec;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;



use codec::capabilities::{CapabilityProvider, Dispatcher, NullDispatcher};
use codec::core::OP_BIND_ACTOR;
use codec::deserialize;
use wascc_codec::core::CapabilityConfiguration;

use std::error::Error;
use std::sync::RwLock;

const SYSTEM_ACTOR: &str = "system";

#[cfg(not(feature = "static_plugin"))]
capability_provider!(ShellProvider, ShellProvider::new);

const CAPABILITY_ID: &str = "krustlet:broken"; 

pub struct ShellProvider {
    dispatcher: RwLock<Box<dyn Dispatcher>>,
}

impl Default for ShellProvider {
    fn default() -> Self {
        env_logger::init();

        ShellProvider { 
            dispatcher: RwLock::new(Box::new(NullDispatcher::new())),           
        }
    }
}

impl ShellProvider {
    pub fn new() -> Self {
        Self::default()
    }

    fn configure(
        &self,
        config: impl Into<CapabilityConfiguration>,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let _config = config.into();

        Ok(vec![])
    }    
}

impl CapabilityProvider for ShellProvider {
    fn capability_id(&self) -> &'static str {
        CAPABILITY_ID
    }

    // Invoked by the runtime host to give this provider plugin the ability to communicate
    // with actors
    fn configure_dispatch(&self, dispatcher: Box<dyn Dispatcher>) -> Result<(), Box<dyn Error>> {
        trace!("Dispatcher received.");
        let mut lock = self.dispatcher.write().unwrap();
        *lock = dispatcher;

        Ok(())
    }

    fn name(&self) -> &'static str {
        "Shell Capability Provider" 
    }

    // Invoked by host runtime to allow an actor to make use of the capability
    // All providers MUST handle the "configure" message, even if no work will be done
    fn handle_call(&self, actor: &str, op: &str, msg: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        trace!("Received host call from {}, operation - {}", actor, op);

        match op {            
            OP_BIND_ACTOR if actor == SYSTEM_ACTOR => {
                let cfg_vals = deserialize::<CapabilityConfiguration>(msg)?;

                self.configure(cfg_vals)            
            }
            
            shell::OP_INVOKE => {
                let result = std::process::Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .output()
                .expect("it broke");
                match result {
                    Ok(output) => {
                        Ok(vec![])
                    }
                }
                Ok(vec![])
            }
            
            _ => Err("bad dispatch".into()),
        }
    }
}

pub mod shell;