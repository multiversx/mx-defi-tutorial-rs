#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod config;

use crate::config::Offer;

#[multiversx_sc::contract]
pub trait Escrow: config::ConfigModule {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("*")]
    #[endpoint(createOffer)]
    fn create_offer(
        &self,
        accepted_token: TokenIdentifier,
        accepted_nonce: u64,
        accepted_amount: BigUint,
        opt_accepted_address: OptionalValue<ManagedAddress>,
    ) -> u64 {
        let payment = self.call_value().single_esdt();
        require!(
            payment.token_nonce > 0 && payment.amount == 1,
            "ESDT is not an NFT"
        );

        let caller = self.blockchain().get_caller();
        let new_offer_id = self.get_new_offer_id();
        let accepted_address_option = opt_accepted_address.into_option();

        self.created_offers(&caller).insert(new_offer_id);
        if accepted_address_option.is_some() {
            let accepted_address = accepted_address_option.clone().unwrap();
            self.wanted_offers(&accepted_address).insert(new_offer_id);
        }

        let accepted_payment =
            EsdtTokenPayment::new(accepted_token, accepted_nonce, accepted_amount);
        let offer = Offer::new(caller, payment, accepted_payment, accepted_address_option);
        self.offers(new_offer_id).set(offer);

        new_offer_id
    }

    #[endpoint(cancelOffer)]
    fn cancel_offer(&self, offer_id: u64) {
        let offer = self.get_offer_by_id(offer_id);
        let caller = self.blockchain().get_caller();

        require!(
            offer.creator == caller,
            "Only the offer creator can cancel it"
        );

        if offer.opt_accepted_address.is_some() {
            let accepted_address = offer.opt_accepted_address.unwrap();
            self.wanted_offers(&accepted_address).swap_remove(&offer_id);
        }

        self.created_offers(&caller).swap_remove(&offer_id);
        self.offers(offer_id).clear();

        self.send().direct_esdt(
            &offer.creator,
            &offer.offered_payment.token_identifier,
            offer.offered_payment.token_nonce,
            &offer.offered_payment.amount,
        );
    }

    #[payable("*")]
    #[endpoint(acceptOffer)]
    fn accept_offer(&self, offer_id: u64) {
        let caller = self.blockchain().get_caller();
        let offer = self.get_offer_by_id(offer_id);
        let payment = self.call_value().single_esdt();
        require!(
            payment == offer.accepted_payment,
            "Incorrect payment for offer"
        );
        if offer.opt_accepted_address.is_some() {
            let accepted_address = offer.opt_accepted_address.unwrap();
            require!(accepted_address == caller, "Incorrect caller");
            self.wanted_offers(&accepted_address).swap_remove(&offer_id);
        }

        self.created_offers(&offer.creator).swap_remove(&offer_id);
        self.offers(offer_id).clear();

        self.send().direct_esdt(
            &offer.creator,
            &payment.token_identifier,
            payment.token_nonce,
            &payment.amount,
        );
        self.send().direct_esdt(
            &caller,
            &offer.offered_payment.token_identifier,
            offer.offered_payment.token_nonce,
            &offer.offered_payment.amount,
        );
    }

    fn get_offer_by_id(&self, offer_id: u64) -> Offer<Self::Api> {
        let offer_mapper = self.offers(offer_id);
        require!(!offer_mapper.is_empty(), "Offer does not exist");

        offer_mapper.get()
    }

    fn get_new_offer_id(&self) -> u64 {
        let last_offer_id_mapper = self.last_offer_id();
        let new_offer_id = last_offer_id_mapper.get() + 1;
        last_offer_id_mapper.set(new_offer_id);

        new_offer_id
    }
}
