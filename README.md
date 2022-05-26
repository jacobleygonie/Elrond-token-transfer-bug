# Elrond token transfer bug

This repo reproduces a bug with the _direct_ function of the Send API (https://docs.elrond.com/developers/developer-reference/elrond-wasm-api-functions/). More specifically, the _direct_ function takes 4 arguments:

`direct<D>(to: &ManagedAddress, token: &TokenIdentifier, nonce: u64, amount: &BigUint, data: D)`

We find that the direct function fails when both the following conditions hold:

- The first argument _To_ is a **smart contract address**.
- The last argument _Data_ is **non-empty**.

## Summary of the contracts and tests

To showcase the problem, we implement two contracts:

- The _sender_ has a method _sendTokens_ that takes as input both the receiver address and the data to associate to the token transfer.
- The _receiver_ has no mehod.

In Mandos tests we consider the four exclusive possibilities:

1. The receiver is a user and the data is empty.
2. The receiver is a user and the data is non-empty.
3. The receiver is a contract and the data is empty.
4. The receiver is a contract and the data is non-empty.

All the tests pass, except the last on for which the error message is
`5 (out of gas). Message: not enough gas`

## Reproducing the bug

The contracts and tests can be compiled as follows.

```
cd sender
erdpy contract build
cd ../receiver
erdpy contract build
erdpy contract test
```
