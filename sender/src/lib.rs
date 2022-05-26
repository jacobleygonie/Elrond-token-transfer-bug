#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::contract]
pub trait Sender {
  #[init]
  fn init(&self) {}

  #[endpoint(sendTokens)]
  fn send_tokens(
    &self,
    token_id: TokenIdentifier,
    amount: BigUint,
    receiver: ManagedAddress,
    data: &[u8]
  ) {
    self.send().direct(&receiver, &token_id, 0, &amount, data);
  }
}
