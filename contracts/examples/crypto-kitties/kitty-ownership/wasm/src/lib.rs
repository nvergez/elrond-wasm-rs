// Code generated by the mx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           21
// Async Callback:                       1
// Total number of exported functions:  23

#![no_std]
#![feature(alloc_error_handler, lang_items)]

mx_sc_wasm_adapter::allocator!();
mx_sc_wasm_adapter::panic_handler!();

mx_sc_wasm_adapter::endpoints! {
    kitty_ownership
    (
        setGeneScienceContractAddress
        setKittyAuctionContractAddress
        claim
        totalSupply
        balanceOf
        ownerOf
        approve
        transfer
        transfer_from
        tokensOfOwner
        allowAuctioning
        approveSiringAndReturnKitty
        createGenZeroKitty
        getKittyById
        isReadyToBreed
        isPregnant
        canBreedWith
        approveSiring
        breedWith
        giveBirth
        birthFee
        callBack
    )
}
