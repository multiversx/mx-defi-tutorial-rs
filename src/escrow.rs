#![no_std]

multiversx_sc::imports!();

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait Escrow {
    #[init]
    fn init(&self) {}

    #[payable("EGLD")]
    #[endpoint(deposit)]
    fn deposit(&self) {
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().egld_value();

        let new_balance = self.user_balance(caller.clone()).get() + &*payment;
        self.user_balance(caller).set(new_balance);
    }

    #[endpoint(withdraw)]
    fn withdraw(&self) {
        let caller = self.blockchain().get_caller();
        let balance = self.user_balance(caller.clone()).get();

        self.user_balance(caller.clone()).set(BigUint::from(0u64));
        self.send()
            .direct_egld(&caller, &balance);
    }

    // storage

    #[view(getUserBalance)]
    #[storage_mapper("userBalance")]
    fn user_balance(&self, address: ManagedAddress) -> SingleValueMapper<BigUint>;
}
