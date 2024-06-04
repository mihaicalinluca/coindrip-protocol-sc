// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

pub struct CoinDripProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for CoinDripProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = CoinDripProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        CoinDripProxyMethods { wrapped_tx: tx }
    }
}

pub struct CoinDripProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> CoinDripProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init(
        self,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> CoinDripProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn create_stream<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<u64>,
        Arg2: ProxyArg<u64>,
        Arg3: ProxyArg<OptionalValue<bool>>,
    >(
        self,
        recipient: Arg0,
        start_time: Arg1,
        end_time: Arg2,
        _can_cancel: Arg3,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("createStream")
            .argument(&recipient)
            .argument(&start_time)
            .argument(&end_time)
            .argument(&_can_cancel)
            .original_result()
    }

    ///  
    /// Calculates the recipient balance based on the amount stream so far and the already claimed amount 
    /// |xxxx|*******|--| 
    /// S            C  E 
    /// S = start time 
    /// xxxx = already claimed amount 
    /// C = current time 
    /// E = end time 
    /// The zone marked with "****..." represents the recipient balance 
    pub fn recipient_balance<
        Arg0: ProxyArg<u64>,
    >(
        self,
        stream_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("recipientBalance")
            .argument(&stream_id)
            .original_result()
    }

    /// Calculates the sender balance based on the recipient balance and the claimed balance 
    /// |----|-------|**| 
    /// S   L.C      C  E 
    /// S = start time 
    /// L.C = last claimed amount 
    /// C = current time 
    /// E = end time 
    /// The zone marked with "**" represents the sender balance 
    pub fn sender_balance<
        Arg0: ProxyArg<u64>,
    >(
        self,
        stream_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("senderBalance")
            .argument(&stream_id)
            .original_result()
    }

    /// This endpoint can be used by the recipient of the stream to claim the stream amount of tokens 
    pub fn claim_from_stream<
        Arg0: ProxyArg<u64>,
    >(
        self,
        stream_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("claimFromStream")
            .argument(&stream_id)
            .original_result()
    }

    /// This endpoint can be used the by sender or recipient of a stream to cancel the stream. 
    /// !!! The stream needs to be cancelable (a property that is set when the stream is created by the sender) 
    pub fn cancel_stream<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<OptionalValue<bool>>,
    >(
        self,
        stream_id: Arg0,
        _with_claim: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("cancelStream")
            .argument(&stream_id)
            .argument(&_with_claim)
            .original_result()
    }

    /// After a stream was cancelled, you can call this endpoint to claim the streamed tokens as a recipient or the remaining tokens as a sender 
    /// This endpoint is especially helpful when the recipient/sender is a non-payable smart contract 
    /// For convenience, this endpoint is automatically called by default from the cancel_stream endpoint (is not instructed otherwise by the "_with_claim" param) 
    pub fn claim_from_stream_after_cancel<
        Arg0: ProxyArg<u64>,
    >(
        self,
        stream_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("claimFromStreamAfterCancel")
            .argument(&stream_id)
            .original_result()
    }

    pub fn get_stream<
        Arg0: ProxyArg<u64>,
    >(
        self,
        stream_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, Stream<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getStreamData")
            .argument(&stream_id)
            .original_result()
    }

    pub fn streams_list<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, u64>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getStreamListByAddress")
            .argument(&address)
            .original_result()
    }

    pub fn last_stream_id(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getLastStreamId")
            .original_result()
    }
}

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub struct Stream<Api>
where
    Api: ManagedTypeApi,
{
    pub sender: ManagedAddress<Api>,
    pub recipient: ManagedAddress<Api>,
    pub payment_token: EgldOrEsdtTokenIdentifier<Api>,
    pub payment_nonce: u64,
    pub deposit: BigUint<Api>,
    pub claimed_amount: BigUint<Api>,
    pub can_cancel: bool,
    pub start_time: u64,
    pub end_time: u64,
    pub balances_after_cancel: Option<BalancesAfterCancel<Api>>,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct BalancesAfterCancel<Api>
where
    Api: ManagedTypeApi,
{
    pub sender_balance: BigUint<Api>,
    pub recipient_balance: BigUint<Api>,
}