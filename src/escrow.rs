#![no_std]

multiversx_sc::imports!();

pub mod errors;
pub mod events;
pub mod offer;

use crate::{errors::*, offer::Offer};
use offer::OfferId;

#[multiversx_sc::contract]
pub trait Escrow: offer::OfferModule + events::EventsModule {
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
        accepted_address: ManagedAddress,
    ) -> OfferId {
        let payment = self.call_value().single_esdt();
        let caller = self.blockchain().get_caller();
        let new_offer_id = self.get_new_offer_id();

        self.created_offers(&caller).insert(new_offer_id);
        self.wanted_offers(&accepted_address).insert(new_offer_id);

        let accepted_payment =
            EsdtTokenPayment::new(accepted_token, accepted_nonce, accepted_amount);
        let offer = Offer::new(caller, payment, accepted_payment, accepted_address);
        self.offers(new_offer_id).set(&offer);

        self.emit_create_offer_event(&offer);

        new_offer_id
    }

    #[endpoint(cancelOffer)]
    fn cancel_offer(&self, offer_id: OfferId) {
        let offer = self.get_offer_by_id(offer_id);
        let caller = self.blockchain().get_caller();

        require!(offer.creator == caller, ERROR_ONLY_CREATOR);

        self.created_offers(&caller).swap_remove(&offer_id);
        self.wanted_offers(&offer.accepted_address)
            .swap_remove(&offer_id);
        self.offers(offer_id).clear();

        self.send().direct_esdt(
            &offer.creator,
            &offer.offered_payment.token_identifier,
            offer.offered_payment.token_nonce,
            &offer.offered_payment.amount,
        );

        self.emit_cancel_offer_event(&offer);
    }

    #[payable("*")]
    #[endpoint(acceptOffer)]
    fn accept_offer(&self, offer_id: OfferId) {
        let caller = self.blockchain().get_caller();
        let offer = self.get_offer_by_id(offer_id);
        let payment = self.call_value().single_esdt();
        require!(offer.accepted_address == caller, ERROR_INCORRECT_CALLER);
        require!(payment == offer.accepted_payment, ERROR_INCORRECT_PAYMENT);

        self.created_offers(&offer.creator).swap_remove(&offer_id);
        self.wanted_offers(&offer.accepted_address)
            .swap_remove(&offer_id);
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

        self.emit_accept_offer_event(&offer);
    }

    fn get_offer_by_id(&self, offer_id: OfferId) -> Offer<Self::Api> {
        let offer_mapper = self.offers(offer_id);
        require!(!offer_mapper.is_empty(), ERROR_OFFER_DOES_NOT_EXIST);

        offer_mapper.get()
    }

    fn get_new_offer_id(&self) -> OfferId {
        let last_offer_id_mapper = self.last_offer_id();
        let new_offer_id = last_offer_id_mapper.get() + 1;
        last_offer_id_mapper.set(new_offer_id);

        new_offer_id
    }
}
