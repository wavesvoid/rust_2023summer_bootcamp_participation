use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum VMError {
    #[error("Slot can't have more than 10 items in it")]
    SlotCapacityExceeded,

    #[error("Price cannot be bigger than u16::MAX")]
    BadSlotItemPrice,

    #[error("Price must be specified so it can be divided by 5")]
    PriceTooBig,

    #[error("No products in slot")]
    SlotEmpty,

    #[error("You requested more product items than available in slot")]
    NotEnoughItemsInSlot,

    #[error("Need more coins")]
    NotEnoughCoinsGiven,

    #[error("There are not enough coins to match whole sum.")]
    CannotGiveChange,

    #[error("Machine empty on products")]
    NoProducts,

    #[error("There are not enough coins to match whole sum.")]
    NoCoinsInMachine,
}
