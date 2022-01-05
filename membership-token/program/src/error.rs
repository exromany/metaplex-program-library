//! Module provide program defined errors

use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
    // 6000
    #[msg("No valid signer present")]
    NoValidSignerPresent,
    // 6001
    #[msg("Some string variable is longer than allowed")]
    StringIsTooLong,
    // 6002
    #[msg("Name string variable is longer than allowed")]
    NameIsTooLong,
    // 6003
    #[msg("Description string variable is longer than allowed")]
    DescriptionIsTooLong,
    // 6004
    #[msg("Provided supply is gt than available")]
    SupplyIsGtThanAvailable,
    // 6005
    #[msg("Supply is not provided")]
    SupplyIsNotProvided,
    // 6006
    #[msg("Derived key invalid")]
    DerivedKeyInvalid,
    #[msg("Market is not started")]
    MarketIsNotStarted,
    #[msg("User reach buy limit")]
    UserReachBuyLimit,
    #[msg("Math overflow")]
    MathOverflow,
    // 6007
    #[msg("Invalid selling resource owner provided")]
    SellingResourceOwnerInvalid,
    // 6008
    #[msg("PublicKeyMismatch")]
    PublicKeyMismatch,
    // 6009
    #[msg("Pieces in one wallet cannot be greater than Max Supply value")]
    PiecesInOneWalletIsTooMuch,
    // 6010
    #[msg("StartDate cannot be in the past")]
    StartDateIsInPast,
    // 6011
    #[msg("EndDate should not be earlier than StartDate")]
    EndDateIsEarlierThanBeginDate,
    // 6012
    #[msg("Incorrect account owner")]
    IncorrectOwner,
}