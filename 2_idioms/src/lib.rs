mod errors;

use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::Deref;

pub use errors::VMError;

// Maximum products within a VendingMachine slot
const MAX_SLOT_CAPACITY: usize = 10;

pub mod vm_states {
    #[derive(Debug, PartialEq)]
    pub struct ReadyToServe; // accepting orders and coins

    #[derive(Debug, PartialEq)]
    pub struct SessionActive; // when currently in use
    
    #[derive(Debug, PartialEq)]
    pub struct Finished; // accepting orders and coins
}

/// Just a builder pattern implementation
///
pub struct VMBuilder {
    slots: Vec<ProductSlot>,
    coins_per_stack: usize,
}
impl Default for VMBuilder {
    fn default() -> Self {
        VMBuilder {
            // Product slots
            slots: vec![],
            // Coin stacks, where coins are kept
            coins_per_stack: 50,
        }
    }
}
impl VMBuilder {
    pub fn new() -> Self { Default::default() }

    pub fn set_coins_per_stack(mut self, c: usize) -> Self {
        self.coins_per_stack = c;
        self
    }

    pub fn add_products(mut self, products: Vec<ProductSlot>) -> Self {
        self.slots = products;
        self
    }

    pub fn build(self) -> VendingMachine<vm_states::ReadyToServe> {
        let mut vm = VendingMachine::<_> {
            slots: HashMap::new(),
            coin_stack: HashMap::new(),
            balance: 0,
            change_to_give: vec![],
            change_subs: HashMap::new(),
            _state: PhantomData,
        };

        // "Fill" every coin stack with some coins
        units::Coin::make_iter().for_each(|c| {
            vm.coin_stack.insert(c, self.coins_per_stack);
        });

        // Fill in product slots
        self.slots.into_iter().for_each(|s| {
            vm.slots.insert(s.product.0.clone(), s);
        });

        vm
    }
}

/// Vending Machine
///
/// Instantiating new VM allows user to call methods
/// to make "virtual" purchases of filled products.
///
#[derive(Debug, PartialEq)]
pub struct VendingMachine<S> {
    slots: HashMap<units::ProductName, ProductSlot>,
    coin_stack: HashMap<units::Coin, usize>,
    balance: usize,
    change_to_give: Vec<units::Coin>,
    change_subs: HashMap<units::Coin, usize>,
    _state: PhantomData<S>,
}

/// Just some boilerplate to change VM into a new state, moving old fields
/// into a new state
impl<S> VendingMachine<S> {
    fn compose_from_state<T>(&mut self) -> VendingMachine<T> {
        VendingMachine::<T> {
            slots: std::mem::take(&mut self.slots),
            coin_stack: std::mem::take(&mut self.coin_stack),
            balance: self.balance,
            change_to_give: std::mem::take(&mut self.change_to_give),
            change_subs: std::mem::take(&mut self.change_subs),
            _state: PhantomData,
        }
    }

    pub fn check_balance(&self) -> usize {
        self.balance
    }
}

impl VendingMachine<vm_states::ReadyToServe> {
    pub fn begin(mut self) -> VendingMachine<vm_states::SessionActive> {
        self.compose_from_state::<vm_states::SessionActive>()
    }
}

impl VendingMachine<vm_states::SessionActive> {
    pub fn insert_coins(&mut self, coins: Vec<units::Coin>) {
        coins.into_iter().for_each(|c| {
            // count balance also taking into account
            // amount of inserted coins and their denomination
            self.balance += c as usize;
            self.coin_stack
                .entry(c)
                .and_modify(|ic| *ic += 1)
                .or_insert(1);
        });

        // println!("After insert: {:?}", self.coin_stack);
    }

    /// Make purchase
    ///
    /// Performs checks on:
    /// - emptyness of slots (if any products are available)
    /// - available products in a given slot comparing to the requested amount of products
    /// - ability to give the change (rest) to the user, and reject in other case
    pub fn purchase<PN: Into<units::ProductName>>(
        &mut self,
        product_name: PN,
        amount: usize,
    ) -> Result<Purchase, VMError> {
        let product_name = product_name.into();
        let slot = self
            .slots
            .get_mut(&product_name)
            .ok_or(VMError::SlotEmpty)?;

        // Check if enough products available for purchase
        //
        if amount > slot.amount {
            return Err(VMError::NotEnoughItemsInSlot);
        }

        // Check if current balance is sufficient
        //
        let complete_price = *slot.price.as_ref() * amount;
        if complete_price > self.balance {
            return Err(VMError::NotEnoughCoinsGiven);
        }

        // Check if change/rest can be given
        // update balance
        //
        let mut price = complete_price;
        let mut balance_rest = self.balance - complete_price;
        let mut change_to_give = vec![];
        for coin in units::Coin::make_iter().rev() {
            let contains_count: usize = balance_rest / &coin.into();

            if contains_count != 0 && balance_rest != 0 {
                balance_rest -= &coin.into() * contains_count;
                (0..contains_count)
                    .for_each(|_| change_to_give.push(coin));
            }

            if price == 0 {
                continue;
            }

            let sub = *self.change_subs.get(&coin).unwrap_or(&0);
            let available = *self.coin_stack.get(&coin).unwrap() - sub;
            let fits_as_many: usize = price / &coin.into();

            if fits_as_many == 0 {
                continue;
            }

            let coin_count: usize = if available >= fits_as_many {
                fits_as_many
            } else {
                available
            };

            price -= coin_count * &coin.into();
            self.change_subs
                .entry(coin)
                .and_modify(|counter| *counter += coin_count)
                .or_insert(coin_count);
        }

        if price > 0 || balance_rest > 0 {
            return Err(VMError::CannotGiveChange);
        }

        // Update balance and slots
        self.balance -= complete_price;
        slot.amount -= amount;
        self.change_to_give = change_to_give;

        Ok(Purchase {
            products: (0..amount).map(|_| Product(product_name.clone())).collect(),
        })
    }

    /// Get through with the buying session
    ///
    /// Return some change if needed
    pub fn finish(&mut self) -> (VendingMachine<vm_states::Finished>, Vec<units::Coin>) {
        // move into new state
        let mut machine = self.compose_from_state::<vm_states::Finished>();

        // If no need to give change
        //
        if machine.balance == 0 {
            return (machine, vec![]);
        }

        machine.balance = 0;
        std::mem::take(&mut machine.change_subs);
        let change = std::mem::take(&mut machine.change_to_give);

        (machine, change)
    }
}

/// Performs final checks on:
/// - if there any available products left in the slots
/// - if there is no available coins left in stack returns Error
///     if no available coins, returns Error
/// - otherwise moves into <ReadyToServe> state
impl VendingMachine<vm_states::Finished> {
    pub fn restart(&mut self) -> Result<VendingMachine<vm_states::SessionActive>, VMError> {
        // If there is not a single Product in any of slots left
        //

        // println!("{:?}", self.slots);
        if !self.slots.iter().any(|(_prod_name, slot)| slot.amount > 0) {
            return Err(VMError::NoProducts);
        }

        // If no coins in the stack left there is a risk we cannot
        // give a proper change back
        //
        if !self.coin_stack.iter().any(|(_coin, count)| *count > 0) {
            return Err(VMError::NoCoinsInMachine);
        }

        Ok(self.compose_from_state::<vm_states::SessionActive>())
    }
}

#[derive(Debug, PartialEq)]
pub struct Purchase {
    pub products: Vec<Product>,
}

#[derive(Debug, PartialEq)]
pub struct ProductSlot {
    product: Product,
    amount: usize,
    price: units::ProductPrice,
}
impl ProductSlot {
    pub fn new<S>(name: S, amount: usize, price: usize) -> Result<Self, VMError>
    where
        S: Into<String>,
    {
        if amount > MAX_SLOT_CAPACITY {
            return Err(VMError::SlotCapacityExceeded);
        }

        Ok(Self {
            product: Product::new(name.into()),
            amount,
            price: units::ProductPrice::new(price)?,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Product(pub units::ProductName);
impl Product {
    pub fn new(name: String) -> Self {
        Self(units::ProductName(name))
    }
}
impl Deref for Product {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0 .0[..]
    }
}

pub mod units {
    use super::*;

    #[derive(Debug, PartialEq)]
    pub struct ProductPrice(usize);
    impl ProductPrice {
        pub fn new(price: usize) -> Result<Self, VMError> {
            if price > u16::MAX.into() {
                return Err(VMError::PriceTooBig);
            }
            Ok(Self(price))
        }
    }
    impl AsRef<usize> for ProductPrice {
        fn as_ref(&self) -> &usize {
            &self.0
        }
    }

    #[derive(Debug, Eq, Hash, PartialEq, Default, Clone)]
    pub struct ProductName(pub String);
    impl AsRef<str> for ProductName {
        fn as_ref(&self) -> &str {
            &self.0[..]
        }
    }
    impl From<&str> for ProductName {
        fn from(value: &str) -> Self {
            Self(value.to_owned())
        }
    }

    #[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
    #[repr(usize)]
    pub enum Coin {
        One = 1,
        Two = 2,
        Five = 5,
        Ten = 10,
        Twenty = 20,
        Fifty = 50,
    }
    impl Coin {
        pub fn make_iter() -> std::array::IntoIter<Coin, 6> {
            [
                Coin::One,
                Coin::Two,
                Coin::Five,
                Coin::Ten,
                Coin::Twenty,
                Coin::Fifty,
            ]
            .into_iter()
        }
    }
    impl From<Coin> for usize {
        fn from(value: Coin) -> usize {
            value as usize
        }
    }

    use core::iter::Sum;
    impl Sum<Coin> for usize {
        fn sum<I: Iterator<Item = Coin>>(iter: I) -> usize {
            iter.fold(0, |acc, c| acc + c as usize)
        }
    }
}
