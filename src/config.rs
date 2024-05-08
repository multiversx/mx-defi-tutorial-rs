multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct Offer<M: ManagedTypeApi> {
    pub creator: ManagedAddress<M>,
    pub offered_payment: EsdtTokenPayment<M>,
    pub accepted_payment: EsdtTokenPayment<M>,
    pub opt_accepted_address: Option<ManagedAddress<M>>,
}

impl<M: ManagedTypeApi> Offer<M> {
    pub fn new(
        creator: ManagedAddress<M>,
        offered_payment: EsdtTokenPayment<M>,
        accepted_payment: EsdtTokenPayment<M>,
        opt_accepted_address: Option<ManagedAddress<M>>,
    ) -> Self {
        Offer {
            creator,
            offered_payment,
            accepted_payment,
            opt_accepted_address,
        }
    }
}

#[multiversx_sc::module]
pub trait ConfigModule {
    #[view(getCreatedOffers)]
    fn get_created_offers(
        &self,
        address: ManagedAddress,
    ) -> MultiValueEncoded<MultiValue2<u64, Offer<Self::Api>>> {
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
    ) -> MultiValueEncoded<MultiValue2<u64, Offer<Self::Api>>> {
        let mut result = MultiValueEncoded::new();

        for offer_id in self.wanted_offers(&address).iter() {
            result.push(self.get_offer_result(offer_id));
        }

        result
    }

    fn get_offer_result(&self, offer_id: u64) -> MultiValue2<u64, Offer<Self::Api>> {
        let offer = self.offers(offer_id).get();

        MultiValue2::from((offer_id, offer))
    }

    #[storage_mapper("createdOffers")]
    fn created_offers(&self, address: &ManagedAddress) -> UnorderedSetMapper<u64>;

    #[storage_mapper("wantedOffers")]
    fn wanted_offers(&self, address: &ManagedAddress) -> UnorderedSetMapper<u64>;

    #[view(getOffer)]
    #[storage_mapper("offers")]
    fn offers(&self, id: u64) -> SingleValueMapper<Offer<Self::Api>>;

    #[view(getLastOfferId)]
    #[storage_mapper("lastOfferId")]
    fn last_offer_id(&self) -> SingleValueMapper<u64>;
}
