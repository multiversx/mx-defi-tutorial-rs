// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            9
// Async Callback (empty):               1
// Total number of exported functions:  11

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    escrow
    (
        init => init
        upgrade => upgrade
        createOffer => create_offer
        cancelOffer => cancel_offer
        acceptOffer => accept_offer
        listOffers => get_offers
        getUserBalance => user_balance
        created_offers => created_offers
        wanted_offers => wanted_offers
        offers => offers
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
