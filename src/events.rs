multiversx_sc::imports!();

use crate::offer::{self, Offer};

#[multiversx_sc::module]
pub trait EventsModule: offer::OfferModule {
    fn emit_create_offer_event(&self, offer: &Offer<Self::Api>) {
        let epoch = self.blockchain().get_block_epoch();
        let timestamp = self.blockchain().get_block_timestamp();
        self.create_offer_event(
            &offer.creator,
            &offer.accepted_address,
            epoch,
            timestamp,
            offer,
        );
    }

    fn emit_cancel_offer_event(&self, offer: &Offer<Self::Api>) {
        let epoch = self.blockchain().get_block_epoch();
        let timestamp = self.blockchain().get_block_timestamp();
        self.cancel_offer_event(
            &offer.creator,
            &offer.accepted_address,
            epoch,
            timestamp,
            offer,
        );
    }

    fn emit_accept_offer_event(&self, offer: &Offer<Self::Api>) {
        let epoch = self.blockchain().get_block_epoch();
        let timestamp = self.blockchain().get_block_timestamp();
        self.accept_offer_event(
            &offer.creator,
            &offer.accepted_address,
            epoch,
            timestamp,
            offer,
        );
    }

    #[event("createOffer")]
    fn create_offer_event(
        &self,
        #[indexed] creator: &ManagedAddress,
        #[indexed] buyer: &ManagedAddress,
        #[indexed] epoch: u64,
        #[indexed] timestamp: u64,
        offer: &Offer<Self::Api>,
    );

    #[event("cancelOffer")]
    fn cancel_offer_event(
        &self,
        #[indexed] creator: &ManagedAddress,
        #[indexed] buyer: &ManagedAddress,
        #[indexed] epoch: u64,
        #[indexed] timestamp: u64,
        offer: &Offer<Self::Api>,
    );

    #[event("acceptOffer")]
    fn accept_offer_event(
        &self,
        #[indexed] creator: &ManagedAddress,
        #[indexed] buyer: &ManagedAddress,
        #[indexed] epoch: u64,
        #[indexed] timestamp: u64,
        offer: &Offer<Self::Api>,
    );
}
