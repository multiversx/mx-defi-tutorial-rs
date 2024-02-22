#![no_std]

multiversx_sc::imports!();

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait Escrow {
    #[init]
    fn init(&self) {}

    #[payable("EGLD")]
    #[endpoint(deposit)]
    fn deposit(&self) {}

    #[only_owner]
    #[endpoint(deposit)]
    fn withdraw(&self) {
       let balance = &self
            .blockchain()
            .get_sc_balance(&EgldOrEsdtTokenIdentifier::egld(), 0);

        self.send()
            .direct_egld(&self.blockchain().get_caller(), balance);
    }
}
