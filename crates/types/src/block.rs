use crate::certificate::Certificate;
use crate::hash::Hash32;
use crate::sortition::seed::Seed;
use crate::transaction::transaction::RawTransaction;
use crate::{address::Address, transaction::Transaction};
use minicbor::bytes::ByteVec;
use minicbor::{
    decode::{Decoder, Error as DecodeError},
    encode::{Encoder, Error as EncodeError, Write},
    Decode, Encode,
};

#[derive(Encode, Decode)]
#[cbor(map)]
pub struct BlockHeader {
    #[n(1)]
    version: i8,
    #[n(2)]
    unix_time: i64,
    #[n(3)]
    prev_block_hash: Hash32,
    #[n(4)]
    prev_cert_hash: Hash32,
    #[n(5)]
    state_root: Hash32,
    #[n(6)]
    tx_root: Hash32,
    #[n(7)]
    sortition_seed: Seed,
    #[n(8)]
    proposer_address: Address,
}

pub struct Transactions(Vec<Transaction>);
impl Transactions {
    fn with_capacity(capacity: usize) -> Self {
        Transactions(Vec::with_capacity(capacity))
    }

    pub fn push(&mut self, tx: Transaction) {
        self.0.push(tx)
    }

    pub(crate) fn decode<'b>(
        d: &mut Decoder<'b>,
    ) -> std::result::Result<Transactions, DecodeError> {
        // TODO: format error: https://gitlab.com/twittner/minicbor/-/issues/16
        let raw_txs: Vec<RawTransaction> = d.decode()?;
        let mut txs = Transactions::with_capacity(raw_txs.len());
        for raw_tx in raw_txs {
            txs.push(Transaction::from_raw_transaction(raw_tx)
                .map_err(|_| DecodeError::Message(&"decoding error"))?);
        }

        Ok(txs)
    }

    pub(crate) fn encode<W: Write>(
        txs: &Transactions,
        e: &mut Encoder<W>,
    ) -> std::result::Result<(), EncodeError<W::Error>> {
        // TODO: format error: https://gitlab.com/twittner/minicbor/-/issues/16
        let mut raw_txs = Vec::<RawTransaction>::with_capacity(txs.len());
        for tx in &txs.0 {
            raw_txs.push(tx.to_raw_transaction()
            .map_err(|_| EncodeError::Message(&"encoding error"))?);
        }

        e.encode(raw_txs)?;
        Ok(())
    }

    pub fn len(&self) ->usize{
        self.0.len()
    }
}

#[derive(Encode, Decode)]
#[cbor(map)]
pub struct Block {
    #[n(1)]
    header: BlockHeader,
    #[n(2)]
    prev_cert: Certificate,
    #[n(3)]
    #[cbor(with = "Transactions")]
    txs: Transactions,
}

impl Block {
    crate::impl_from_to_bytes!(Block);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoding() {
        let buf1 = hex::decode(
            "a301a80101021a622ace1d035820307cdf86a913baf3fbf00165dcea2428b7ba91507178cf1da3d7fc5f4498c65d045820bc0d63c3292fd7c8bf5b5a95e5c1f7d9f70f57e23130960ac6dd0d44e0bd4a50055820cb0ccc31a192186e5e01645aab5ce886bc9d1560a01e8100617c3fb7d6370cb80658209a664dbc4f48f92bcc4ae364f80a81ec4124ba78bc1484b0782fceed1e505aae075830b4b43dd593ffb8938e710dc6dacc9982182b5dec5f6a5d1c4a4d516f0f85130fd32d38d34e1aa3baed1f51a9e06dec140855019913b0689eefc8349183ec71fcf7fddfd2f035a502a50158206fd848bda84b325277eed41da0c36b84ac8b4efe96606e87c1a8634b2ac63d78020703840a120c10048112055830b0f50fa16f29747513c29ee2880812e119a3300b226f8bf3bf2f25a04157f6b2ec7a7093e882f176219a61d4b58c12fd0384a9010102449bbae37103186e041903e80501065833a30155017c6646c1c19839192e8c76b9da6f8ead5a31e7a40255014f6c19966599ce7796cd8c6d4c7dfeb24eac1941031903e8076c746573742073656e642d747808586083f16a0f72be00f653f364fe3756fcd4d569b276256c9e0719c12c6def2a82e92649475b00a699bd3046523b9c993e9d07f7b4bdcca90121b609d4f7ce3174c50ae4d88e39132a45d826ab7aca66a63083dce33ec7382c29384085ad62dd9aae0958308b4a3aef75c1bf419e79a33763d49ba3770138278627829539c42abebfc580f357f75a7ed825414cfd35c80634695690a9010102442fab6d2903186e041903e80501065833a301550198e01d56f0c0c346e5a45b6cddde89c98ae7ce4b02550192c557e2850f632e9901b52e1f3502d994776686031903e8076c746573742073656e642d7478085860b522ce35ce82962c20b5618990490a1f60bae007a582443303de0942625a16a5deb2bf816a43a867b499cf2c5dd3a33e057d6972872f3f741bdb72c8ad2cbb148f91055046324bfdedaa51c671a6d8cb61a3070bba07ff25b32d15326655313a095830b3ee7fa1dd553c0822da3e7cc3a6ffdf79dc7cbc7f10548801c691e0a2e2b0bea473c4e154e4749c3e559c6bf6d03065a90101024498981dac03186e041903e80501065833a3015501e302b94cbeeddcdfae85853ed1ff3a03f311769e02550174b82826f6e3988bbd9d825295cda46d3bea68a2031903e8076c746573742073656e642d7478085860b2bdf71b26b4eae9f41bf6bafc429e72ee26af86daef1f2fa2c2befb9d7ca959686b0bda8d05f9e260ee8b81ce93540704dfe47855fe6787d3705fcb60403eb52b54cac6e9719484c33822064fb1bab829bd759f64c82a48ecbf79c845b16927095830b9439e12f760f17ee4bc63aec971302920fb58232717057c69e796ad3f4c5a50fa01462c657d14b73c759f6c96c267daa9010102442946f5d803186e041903e80501065833a3015501636d35bc986e7a77ec1780d0f230975243b683a70255018878552180638549221000395df8254d9355a8c9031903e8076c746573742073656e642d7478085860a19dc11452d6104fe8b9ad89bcc9c3f9f896c0dfb2ae908438fabf84bf1ab678a534a8881b13d0558fdbb60965abc09d148614c148de96af20ef3b006c1b8524bc03cc00e20a195f42e2cdef2325113eea190e97b9bfc48ebe94df0cb5af4c84095830b994008a76e30aebc14330c5307b61d0424513ca771253b8d4849fea3eeaed37d3eb5e56a50d9005500859c84f09aab7",
        )
        .unwrap()
        .to_vec();
        let blk = Block::from_bytes(buf1.as_slice()).unwrap();
        let buf2 = blk.to_bytes().unwrap();
        assert_eq!(buf1, buf2);
    }
}
