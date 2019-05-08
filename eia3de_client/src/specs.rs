//! Specs utils

use specs::{
    prelude::*,
    shred::{Resource, SetupHandler},
};

pub trait ManualSetup {
    fn setup(res: &mut Resources);
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ManualSetupHandler;

impl<T> SetupHandler<T> for ManualSetupHandler
where
    T: Resource + ManualSetup,
{
    fn setup(res: &mut Resources) {
        if res.has_value::<T>() {
            return;
        }

        T::setup(res)
    }
}
