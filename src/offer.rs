multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub type OfferId = u64;

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct Offer<M: ManagedTypeApi> {
    pub creator: ManagedAddress<M>,
    pub offered_payment: EsdtTokenPayment<M>,
    pub accepted_payment: EsdtTokenPayment<M>,
    pub accepted_address: ManagedAddress<M>,
}

impl<M: ManagedTypeApi> Offer<M> {
    pub fn new(
        creator: ManagedAddress<M>,
        offered_payment: EsdtTokenPayment<M>,
        accepted_payment: EsdtTokenPayment<M>,
        accepted_address: ManagedAddress<M>,
    ) -> Self {
        Offer {
            creator,
            offered_payment,
            accepted_payment,
            accepted_address,
        }
    }
}

#[multiversx_sc::module]
pub trait OfferModule {
    #[view(getCreatedOffers)]
    fn get_created_offers(
        &self,
        address: ManagedAddress,
    ) -> MultiValueEncoded<MultiValue2<OfferId, Offer<Self::Api>>> {
        let mut result = MultiValueEncoded::new();

        for offer_id in self.created_offers(&address).iter() {
            result.push(self.get_offer_result(offer_id));
        }

        result
    }

    #[view(getWantedOffers)]
    fn get_wanted_offers(
        &self,
        address: ManagedAddress,
    ) -> MultiValueEncoded<MultiValue2<OfferId, Offer<Self::Api>>> {
        let mut result = MultiValueEncoded::new();

        for offer_id in self.wanted_offers(&address).iter() {
            result.push(self.get_offer_result(offer_id));
        }

        result
    }

    fn get_offer_result(&self, offer_id: OfferId) -> MultiValue2<OfferId, Offer<Self::Api>> {
        let offer = self.offers(offer_id).get();

        MultiValue2::from((offer_id, offer))
    }

    #[storage_mapper("createdOffers")]
    fn created_offers(&self, address: &ManagedAddress) -> UnorderedSetMapper<OfferId>;

    #[storage_mapper("wantedOffers")]
    fn wanted_offers(&self, address: &ManagedAddress) -> UnorderedSetMapper<OfferId>;

    #[view(getOffer)]
    #[storage_mapper("offers")]
    fn offers(&self, id: OfferId) -> SingleValueMapper<Offer<Self::Api>>;

    #[view(getLastOfferId)]
    #[storage_mapper("lastOfferId")]
    fn last_offer_id(&self) -> SingleValueMapper<OfferId>;
}
