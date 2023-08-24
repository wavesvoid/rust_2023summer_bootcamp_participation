use step_2::{
    // Purchase,
    units::{Coin, ProductName},
    vm_states,
    Product,
    ProductSlot,
    VMBuilder,
    VMError,
    VendingMachine,
};

fn new_machine() -> VendingMachine<vm_states::ReadyToServe> {
    VMBuilder::new()
        .set_coins_per_stack(1)
        .add_products(vec![
            // name , amount, price
            ProductSlot::new("Chocolate", 4, 15).unwrap(),
            ProductSlot::new("Chips", 9, 15).unwrap(),
            ProductSlot::new("Cake", 9, 25).unwrap(),
            ProductSlot::new("Coke", 10, 5).unwrap(),
            ProductSlot::new("CharleSTONED", 9, 5).unwrap(),
            ProductSlot::new("Doomed", 10, 1).unwrap(),
        ])
        .build()
}

#[test]
fn test1_vm() {
    let mut vmachine = new_machine().begin();
    let product1_name = "Coke";

    vmachine.insert_coins(vec![Coin::Twenty]);

    let purchase_1 = vmachine.purchase(product1_name, 4);
    let (mut vmachine, rest_1) = vmachine.finish();

    assert_eq!(
        purchase_1.unwrap().products,
        (0..4)
            .into_iter()
            .map(|_| Product(product1_name.into()))
            .collect::<Vec<_>>()
    );
    assert_eq!(rest_1, vec![]);
}

#[test]
fn test2_vm() {
    let mut vmachine = new_machine().begin();
    let product2_name = "Chocolate";

    // 51
    vmachine.insert_coins(vec![Coin::Twenty, Coin::Twenty, Coin::Ten, Coin::One]);
    //  15 * 4 = 60
    let purchase_2_fail = vmachine.purchase(product2_name, 4);

    // 51 + 50 + 20 = 121
    vmachine.insert_coins(vec![Coin::Fifty, Coin::Twenty]);

    // 121 - 60 = 61
    let purchase_2_success = vmachine.purchase(product2_name, 4);
    let (mut vmachine, rest_2) = vmachine.finish();

    assert_eq!(
        purchase_2_success.unwrap().products,
        (0..4)
            .into_iter()
            .map(|_| Product(product2_name.into()))
            .collect::<Vec<_>>()
    );
    assert_eq!(purchase_2_fail.unwrap_err(), VMError::NotEnoughCoinsGiven);
    assert_eq!(rest_2, vec![Coin::Fifty, Coin::Ten, Coin::One]);
}

#[test]
fn test3_vm() {
    let mut vmachine = new_machine().begin();
    let product1_name = "Unknonw";
    let product2_name = "CharleSTONED";
    let product3_name = "Doomed";

    vmachine.insert_coins(vec![Coin::Fifty]);

    // Hints
    // // ProductSlot::new("Coke", 10, 5).unwrap(),
    // // ProductSlot::new("CharleSTONED", 9, 5).unwrap(),

    let purchase_1_fail = vmachine.purchase(product1_name, 2);
    let purchase_2_fail = vmachine.purchase(product2_name, 18);
    let purchase_3_succ = vmachine.purchase(product3_name, 1);
    let purchase_3_fail = vmachine.purchase(product3_name, 1);
    let (mut vmachine, rest1) = vmachine.finish();

    let mut vmachine = vmachine.restart().unwrap();
    let purchase_4_fail = vmachine.purchase(product3_name, 1);

    assert_eq!(purchase_1_fail.unwrap_err(), VMError::SlotEmpty);
    assert_eq!(purchase_2_fail.unwrap_err(), VMError::NotEnoughItemsInSlot);
    assert_eq!(purchase_3_fail.unwrap_err(), VMError::CannotGiveChange);

    assert_eq!(
        purchase_3_succ.unwrap().products,
        vec![Product(product3_name.into())]
    );

    assert_eq!(
        rest1,
        vec![Coin::Twenty, Coin::Twenty, Coin::Five, Coin::Two, Coin::Two]
    );
}

#[test]
fn test4_vm() {
    let mut vmachine = new_machine().begin();
    vmachine.insert_coins(vec![Coin::Five]);

    let purchase_71_succ = vmachine.purchase("Doomed", 1);
    let purchase_72_succ = vmachine.purchase("Doomed", 1);

    let purchase_73_fail = vmachine.purchase("Doomed", 1);
    let (_vmachine, rest_7) = vmachine.finish();

    assert_eq!(
        purchase_71_succ.unwrap().products,
        vec![Product("Doomed".into())]
    );

    assert_eq!(purchase_72_succ.unwrap_err(), VMError::CannotGiveChange);
    assert_eq!(purchase_73_fail.unwrap_err(), VMError::CannotGiveChange);

    assert_eq!(rest_7, vec![Coin::Two, Coin::Two]);
}

// Test error when trying to restart machine when there are 0 products
//
#[test]
fn test5_vm() {
    let mut vmachine = VMBuilder::new()
        .set_coins_per_stack(1)
        .add_products(vec![
            // name , amount, price
            ProductSlot::new("Chocolate", 3, 1).unwrap(),
        ])
        .build()
        .begin();

    vmachine.insert_coins(vec![Coin::Two, Coin::One]);

    let purchase_1_succ = vmachine.purchase("Chocolate", 3);

    let (mut vmachine, rest_7) = vmachine.finish();
    let newmachine = vmachine.restart();

    assert_eq!(
        purchase_1_succ.unwrap().products,
        vec![
            Product("Chocolate".into()),
            Product("Chocolate".into()),
            Product("Chocolate".into())
        ]
    );

    assert_eq!(newmachine.unwrap_err(), VMError::NoProducts);
    // assert_eq!(purchase_73_fail.is_err(), VMError::CannotGiveChange);

    assert_eq!(rest_7, vec![]);
}

// Test error when trying to restart machine when there are 0 coins
//
#[test]
fn test6_vm() {
    let mut vmachine = VMBuilder::new()
        .set_coins_per_stack(0)
        .add_products(vec![
            // name , amount, price
            ProductSlot::new("Chocolate", 3, 1).unwrap(),
        ])
        .build()
        .begin();

    let (mut vmachine, rest_7) = vmachine.finish();
    let newmachine = vmachine.restart();

    assert_eq!(newmachine.unwrap_err(), VMError::NoCoinsInMachine);
}
