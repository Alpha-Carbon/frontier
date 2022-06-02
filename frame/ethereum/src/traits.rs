pub trait GasPayment {
	// check if account has support token and amount to pay the gas fee
	fn check_support_token() -> bool;
}
