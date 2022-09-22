
//1) create VA from nft collection & item Id's --> Done
//2) create tokens
//3) Use onboarding do_buy
//4) transfer tokens to owners
use super::*;
use enum_iterator::all;

impl<T: Config> Pallet<T> {
pub fn virtual_account(collection_id: T::NftCollectionId, item_id: T::NftItemId) -> DispatchResult {
    //Create virtual account
    let text0 = format!("{:?}_{:?}_account",collection_id,item_id.clone());
    let bytes = text0.as_bytes();
    let array: &[u8;8] = &bytes[0..8].try_into().unwrap();
    let account: T::AccountId = PalletId(*array).into_account_truncating();

    //Store account inside storage
    Ownership::<T>::new(collection_id.clone(),item_id.clone(),account.clone()).ok();

    //The virtual account needs some initial funds to pay for asset creation fees
    //These funds could be provided by the FairSquare FeesAccount maintained in the 
    //Onboarding pallet.
    let fees_account = Onboarding::Pallet::<T>::account_id();
    let fees = T::Fees::get();
    let res = <T as pallet::Config>::Currency::transfer(
        &fees_account,
        &account,
        fees,
        ExistenceRequirement::AllowDeath,
    );
    debug_assert!(res.is_ok());   

    Ok(())
}

pub fn nft_transaction(collection_id: T::NftCollectionId, item_id: T::NftItemId,virtual_id:T::AccountId) -> DispatchResult {
    
    //Get collection
        let collection_vec = all::<Nft::PossibleCollections>().collect::<Vec<_>>();
        let _infos = Onboarding::Houses::<T>::get(collection_id.clone(),item_id.clone()).unwrap();
        let mut coll_id = Nft::PossibleCollections::HOUSES;
        for i in collection_vec.iter() {
            let val:T::NftCollectionId=i.value().into();
            if val == collection_id{
                coll_id = *i;                
            }
        }
    //Execute NFT and money transfer
        Onboarding::Pallet::do_buy(coll_id,item_id.clone(),virtual_id.clone(),_infos).ok();        

Ok(())

}

pub fn owner_and_shares(collection_id: T::NftCollectionId, item_id: T::NftItemId) -> Vec<(T::AccountId, Percent)>{

    //Get owners and their reserved contribution to the bid
    
    let reservation_infos = HousingFund::Reservations::<T>::get((collection_id.clone(),item_id.clone())).unwrap();
    let vec0 = reservation_infos.contributions;
    let price = reservation_infos.amount;
    let mut vec = Vec::new() ;
    for i in vec0.iter(){
        let price0 = Self::balance_to_u64_option(price).unwrap();
        let contribution = Self::balance_to_u64_option(i.1.clone()).unwrap();
        let share = Percent::from_rational(contribution,price0);
        debug_assert!(!share.is_zero()); 
        vec.push((i.0.clone(),share));

    }
    vec
}



pub fn create_tokens(origin: OriginFor<T>,collection_id: T::NftCollectionId, item_id: T::NftItemId,account: T::AccountId) -> DispatchResult{

    //Get token class Id:    
    let to = T::Lookup::unlookup(account.clone());    
    TokenId::<T>::mutate(|val|{
        let val0 = val.clone();
        *val = val0+1;
    });
    ensure!(Virtual::<T>::get(collection_id,item_id).is_some(),Error::<T>::InvalidValue);
    let token_id = Virtual::<T>::get(collection_id,item_id).unwrap().token_id;
    
    //Create token class
    let res = Assets::Pallet::<T>::force_create(origin.clone(),token_id.clone().into(),to.clone(),false,One::one());
    debug_assert!(res.is_ok());  
    
    //Set class metadata

    //mint 100 tokens
    let res0 = Assets::Pallet::<T>::mint(RawOrigin::Signed(account.clone()).into(),token_id.clone().into(),to,Self::u32_to_balance_option(100).unwrap());
    debug_assert!(res0.is_ok());
    

    Ok(())

}

pub fn distribute_tokens(account:T::AccountId,collection_id: T::NftCollectionId, item_id: T::NftItemId) -> DispatchResult{
    let shares = Self::owner_and_shares(collection_id.clone(),item_id.clone());  
    ensure!(Virtual::<T>::get(collection_id,item_id).is_some(),Error::<T>::InvalidValue);  
    let token_id = Virtual::<T>::get(collection_id,item_id).unwrap().token_id;
    let total_tokens = Assets::Pallet::<T>::total_supply(token_id.into());
    debug_assert!(total_tokens == Self::u32_to_balance_option(100).unwrap());
    
    let from = T::Lookup::unlookup(account.clone());
    let origin:OriginFor<T> = RawOrigin::Signed(account.clone()).into();
    
    for share in shares.iter(){
        let amount:<T as Assets::Config>::Balance = share.clone().1* total_tokens.clone();        
        debug_assert!(!amount.clone().is_zero());
        let to = T::Lookup::unlookup(share.clone().0);
        Assets::Pallet::<T>::force_transfer(origin.clone(),token_id.clone().into(),from.clone(),to,amount).ok();
    }

    Ok(())
}


// Conversion of u32 to Balance
pub fn u32_to_balance_option(input: u32) -> Option<T::Balance> {
    input.try_into().ok()
}

// Conversion of BalanceOf<T> to u64
pub fn balance_to_u64_option(input: HousingFund::BalanceOf<T>) -> Option<u64> {
    input.try_into().ok()
}

// Conversion of BalanceOf<T> to f64
pub fn balance_to_float_option(input: HousingFund::BalanceOf<T>) -> Option<f64> {
    let integer:u64 = input.try_into().ok().unwrap();
    let float = integer as f64;
    Some(float)
}

}