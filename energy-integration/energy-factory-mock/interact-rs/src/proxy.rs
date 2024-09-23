// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

pub struct EnergyFactoryMockProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for EnergyFactoryMockProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = EnergyFactoryMockProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        EnergyFactoryMockProxyMethods { wrapped_tx: tx }
    }
}

pub struct EnergyFactoryMockProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> EnergyFactoryMockProxyMethods<Env, From, (), Gas>
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
impl<Env, From, To, Gas> EnergyFactoryMockProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn set_user_energy<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
        Arg2: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        user: Arg0,
        energy_amount: Arg1,
        total_locked_tokens: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setUserEnergy")
            .argument(&user)
            .argument(&energy_amount)
            .argument(&total_locked_tokens)
            .original_result()
    }

    pub fn get_energy_amount_for_user<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        user: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getEnergyAmountForUser")
            .argument(&user)
            .original_result()
    }

    pub fn get_energy_entry_for_user<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        user: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, energy_factory::energy::Energy<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getEnergyEntryForUser")
            .argument(&user)
            .original_result()
    }

    pub fn set_user_energy_after_locked_token_transfer<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<energy_factory::energy::Energy<Env::Api>>,
    >(
        self,
        user: Arg0,
        energy: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setUserEnergyAfterLockedTokenTransfer")
            .argument(&user)
            .argument(&energy)
            .original_result()
    }
}
