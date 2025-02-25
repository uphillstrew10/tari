//  Copyright 2019 The Tari Project
//
//  Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
//  following conditions are met:
//
//  1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
//  disclaimer.
//
//  2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
//  following disclaimer in the documentation and/or other materials provided with the distribution.
//
//  3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
//  products derived from this software without specific prior written permission.
//
//  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
//  INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//  DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
//  SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//  SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
//  WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
//  USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use rand::{CryptoRng, Rng};
use std::{fmt::Debug, thread, time::Duration};
use tari_core::transactions::{
    helpers::{create_unblinded_output, TestParams as TestParamsHelpers},
    tari_amount::MicroTari,
    transaction::{OutputFeatures, TransactionInput, UnblindedOutput},
    types::{CommitmentFactory, PrivateKey, PublicKey},
};
use tari_crypto::{
    keys::{PublicKey as PublicKeyTrait, SecretKey as SecretKeyTrait},
    script,
};

pub fn assert_change<F, T>(mut func: F, to: T, poll_count: usize)
where
    F: FnMut() -> T,
    T: Eq + Debug,
{
    let mut i = 0;
    loop {
        let last_val = func();
        if last_val == to {
            break;
        }

        i += 1;
        if i >= poll_count {
            panic!(
                "Value did not change to {:?} within {}ms (last value: {:?})",
                to,
                poll_count * 100,
                last_val,
            );
        }

        thread::sleep(Duration::from_millis(100));
    }
}

pub struct TestParams {
    pub spend_key: PrivateKey,
    pub change_spend_key: PrivateKey,
    pub offset: PrivateKey,
    pub nonce: PrivateKey,
    pub public_nonce: PublicKey,
}
impl TestParams {
    pub fn new<R: Rng + CryptoRng>(rng: &mut R) -> TestParams {
        let r = PrivateKey::random(rng);
        TestParams {
            spend_key: PrivateKey::random(rng),
            change_spend_key: PrivateKey::random(rng),
            offset: PrivateKey::random(rng),
            public_nonce: PublicKey::from_secret_key(&r),
            nonce: r,
        }
    }
}

pub fn make_input<R: Rng + CryptoRng>(
    _rng: &mut R,
    val: MicroTari,
    factory: &CommitmentFactory,
) -> (TransactionInput, UnblindedOutput) {
    let utxo = create_unblinded_output(script!(Nop), OutputFeatures::default(), TestParamsHelpers::new(), val);
    (
        utxo.as_transaction_input(&factory)
            .expect("Should be able to make transaction input"),
        utxo,
    )
}

pub fn make_input_with_features<R: Rng + CryptoRng>(
    _rng: &mut R,
    value: MicroTari,
    factory: &CommitmentFactory,
    features: Option<OutputFeatures>,
) -> (TransactionInput, UnblindedOutput) {
    let utxo = create_unblinded_output(
        script!(Nop),
        features.unwrap_or_default(),
        TestParamsHelpers::new(),
        value,
    );
    (
        utxo.as_transaction_input(&factory)
            .expect("Should be able to make transaction input"),
        utxo,
    )
}
