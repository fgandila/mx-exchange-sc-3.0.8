#![allow(non_snake_case)]
#![allow(dead_code)]

mod proxy;

use common_structs::FarmTokenAttributes;
use multiversx_sc_snippets::imports::*;
use multiversx_sc_snippets::sdk;
use num_bigint::ToBigUint;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};


const GATEWAY: &str = sdk::blockchain::DEVNET_GATEWAY;
const STATE_FILE: &str = "state.toml";
pub const USER1_ADDRESS: &str = "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th";
pub const USER2_ADDRESS: &str = "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th";


#[tokio::main]
async fn main() {
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");
    let mut interact = ContractInteract::new().await;
    // match cmd.as_str() {
    //     "deploy" => interact.deploy().await,
    //     "enterFarm" => interact.enter_farm_endpoint().await,
    //     "claimRewards" => interact.claim_rewards_endpoint().await,
    //     "exitFarm" => interact.exit_farm_endpoint().await,
    //     "mergeFarmTokens" => interact.merge_farm_tokens_endpoint().await,
    //     "claimBoostedRewards" => interact.claim_boosted_rewards().await,
    //     "startProduceRewards" => interact.start_produce_rewards_endpoint().await,
    //     "endProduceRewards" => interact.end_produce_rewards_endpoint().await,
    //     "setPerBlockRewardAmount" => interact.set_per_block_rewards_endpoint().await,
    //     "setBoostedYieldsRewardsPercentage" => interact.set_boosted_yields_rewards_percentage().await,
    //     "calculateRewardsForGivenPosition" => interact.calculate_rewards_for_given_position().await,
    //     "getRewardPerShare" => interact.reward_per_share().await,
    //     "getRewardReserve" => interact.reward_reserve().await,
    //     "getFarmingTokenId" => interact.farming_token_id().await,
    //     "getRewardTokenId" => interact.reward_token_id().await,
    //     "getPerBlockRewardAmount" => interact.per_block_reward_amount().await,
    //     "getLastRewardBlockNonce" => interact.last_reward_block_nonce().await,
    //     "getDivisionSafetyConstant" => interact.division_safety_constant().await,
    //     "getUserTotalFarmPosition" => interact.user_total_farm_position().await,
    //     "getAllowExternalClaim" => interact.allow_external_claim().await,
    //     "getFarmPositionMigrationNonce" => interact.farm_position_migration_nonce().await,
    //     "setLockingScAddress" => interact.set_locking_sc_address().await,
    //     "setLockEpochs" => interact.set_lock_epochs().await,
    //     "getLockingScAddress" => interact.locking_sc_address().await,
    //     "getLockEpochs" => interact.lock_epochs().await,
    //     "registerFarmToken" => interact.register_farm_token().await,
    //     "getFarmTokenId" => interact.farm_token().await,
    //     "getFarmTokenSupply" => interact.farm_token_supply().await,
    //     "addToPauseWhitelist" => interact.add_to_pause_whitelist().await,
    //     "removeFromPauseWhitelist" => interact.remove_from_pause_whitelist().await,
    //     "pause" => interact.pause().await,
    //     "resume" => interact.resume().await,
    //     "getState" => interact.state().await,
    //     "addAdmin" => interact.add_admin_endpoint().await,
    //     "removeAdmin" => interact.remove_admin_endpoint().await,
    //     "updateOwnerOrAdmin" => interact.update_owner_or_admin_endpoint().await,
    //     "getPermissions" => interact.permissions().await,
    //     "addSCAddressToWhitelist" => interact.add_sc_address_to_whitelist().await,
    //     "removeSCAddressFromWhitelist" => interact.remove_sc_address_from_whitelist().await,
    //     "isSCAddressWhitelisted" => interact.is_sc_address_whitelisted().await,
    //     "set_penalty_percent" => interact.set_penalty_percent().await,
    //     "set_minimum_farming_epochs" => interact.set_minimum_farming_epochs().await,
    //     "set_burn_gas_limit" => interact.set_burn_gas_limit().await,
    //     "getPenaltyPercent" => interact.penalty_percent().await,
    //     "getMinimumFarmingEpoch" => interact.minimum_farming_epochs().await,
    //     "getBurnGasLimit" => interact.burn_gas_limit().await,
    //     "getPairContractManagedAddress" => interact.pair_contract_address().await,
    //     "collectUndistributedBoostedRewards" => interact.collect_undistributed_boosted_rewards().await,
    //     "getBoostedYieldsRewardsPercentage" => interact.boosted_yields_rewards_percentage().await,
    //     "getAccumulatedRewardsForWeek" => interact.accumulated_rewards_for_week().await,
    //     "getFarmSupplyForWeek" => interact.farm_supply_for_week().await,
    //     "getRemainingBoostedRewardsToDistribute" => interact.remaining_boosted_rewards_to_distribute().await,
    //     "getUndistributedBoostedRewards" => interact.undistributed_boosted_rewards().await,
    //     "setBoostedYieldsFactors" => interact.set_boosted_yields_factors().await,
    //     "getCurrentWeek" => interact.get_current_week().await,
    //     "getFirstWeekStartEpoch" => interact.first_week_start_epoch().await,
    //     "getLastActiveWeekForUser" => interact.get_last_active_week_for_user_view().await,
    //     "getUserEnergyForWeek" => interact.get_user_energy_for_week_view().await,
    //     "getLastGlobalUpdateWeek" => interact.last_global_update_week().await,
    //     "getTotalRewardsForWeek" => interact.total_rewards_for_week().await,
    //     "getTotalEnergyForWeek" => interact.total_energy_for_week().await,
    //     "getTotalLockedTokensForWeek" => interact.total_locked_tokens_for_week().await,
    //     "updateEnergyForUser" => interact.update_energy_for_user().await,
    //     "getCurrentClaimProgress" => interact.current_claim_progress().await,
    //     "setEnergyFactoryAddress" => interact.set_energy_factory_address().await,
    //     "getEnergyFactoryAddress" => interact.energy_factory_address().await,
    //     _ => panic!("unknown command: {}", &cmd),
    // }
}


#[derive(Debug, Default, Serialize, Deserialize)]
struct State {
    contract_address: Option<Bech32Address>
}

impl State {
        // Deserializes state from file
        pub fn load_state() -> Self {
            if Path::new(STATE_FILE).exists() {
                let mut file = std::fs::File::open(STATE_FILE).unwrap();
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                toml::from_str(&content).unwrap()
            } else {
                Self::default()
            }
        }
    
        /// Sets the contract address
        pub fn set_address(&mut self, address: Bech32Address) {
            self.contract_address = Some(address);
        }
    
        /// Returns the contract address
        pub fn current_address(&self) -> &Bech32Address {
            self.contract_address
                .as_ref()
                .expect("no known contract, deploy first")
        }
    }
    
    impl Drop for State {
        // Serializes state to file
        fn drop(&mut self) {
            let mut file = std::fs::File::create(STATE_FILE).unwrap();
            file.write_all(toml::to_string(self).unwrap().as_bytes())
                .unwrap();
        }
    }

struct ContractInteract {
    interactor: Interactor,
    wallet_address: Address,
    contract_code: BytesValue,
    state: State
}

impl ContractInteract {
    async fn new() -> Self {
        let mut interactor = Interactor::new(GATEWAY).await;
        let wallet_address = interactor.register_wallet(test_wallets::alice());
        
        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/farm-with-locked-rewards.mxsc.json",
            &InterpreterContext::default(),
        );

        ContractInteract {
            interactor,
            wallet_address,
            contract_code,
            state: State::load_state()
        }
    }

    async fn deploy(&mut self) {
        let reward_token_id = TokenIdentifier::from_esdt_bytes(&b""[..]);
        let farming_token_id = TokenIdentifier::from_esdt_bytes(&b""[..]);
        let division_safety_constant = BigUint::<StaticApi>::from(0u128);
        let pair_contract_address = bech32::decode("");
        let owner = bech32::decode("");
        let admins = MultiValueVec::from(vec![bech32::decode("")]);

        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .typed(proxy::FarmProxy)
            .init(reward_token_id, farming_token_id, division_safety_constant, pair_contract_address, owner, admins)
            .code(&self.contract_code)
            .returns(ReturnsNewAddress)
            .prepare_async()
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state
            .set_address(Bech32Address::from_bech32_string(new_address_bech32.clone()));

        println!("new address: {new_address_bech32}");
    }

    async fn enter_farm_endpoint(&mut self, address: &Bech32Address, amount: u128 ) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(amount);

        let opt_orig_caller = OptionalValue::Some(address);


        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .enter_farm_endpoint(opt_orig_caller)
            .payment((TokenIdentifier::from(token_id.as_str()), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn claim_rewards_endpoint(&mut self, user: &Address, token_nonce: u64, farm_token_amount: u128,) {
        let token_id = String::new();
        let token_amount = BigUint::<StaticApi>::from(farm_token_amount);

        let opt_orig_caller = OptionalValue::Some(user);

        let response = self
            .interactor
            .tx()
            .from(user)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .claim_rewards_endpoint(opt_orig_caller)
            .payment((TokenIdentifier::from(token_id.as_str()), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
        
    }

    async fn exit_farm_endpoint(&mut self, amount: u128) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(amount);

        let opt_orig_caller = OptionalValue::Some(bech32::decode(""));

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .exit_farm_endpoint(opt_orig_caller)
            .payment((TokenIdentifier::from(token_id.as_str()), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn merge_farm_tokens_endpoint(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let opt_orig_caller = OptionalValue::Some(bech32::decode(""));

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .merge_farm_tokens_endpoint(opt_orig_caller)
            .payment((TokenIdentifier::from(token_id.as_str()), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn claim_boosted_rewards(&mut self) {
        let opt_user = OptionalValue::Some(bech32::decode(""));

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .claim_boosted_rewards(opt_user)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn start_produce_rewards_endpoint(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .start_produce_rewards_endpoint()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn end_produce_rewards_endpoint(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .end_produce_rewards_endpoint()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn set_per_block_rewards_endpoint(&mut self) {
        let per_block_amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .set_per_block_rewards_endpoint(per_block_amount)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn set_boosted_yields_rewards_percentage(&mut self) {
        let percentage = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .set_boosted_yields_rewards_percentage(percentage)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn calculate_rewards_for_given_position(&mut self, address: Bech32Address, token_amount: u128, attributes: FarmTokenAttributes<StaticApi>) -> RustBigUint {
        let user = bech32::decode(address.to_bech32_str());
        let farm_token_amount = BigUint::<StaticApi>::from(token_amount);
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .calculate_rewards_for_given_position(user, farm_token_amount, attributes)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");

        result_value
    }

    async fn reward_per_share(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .reward_per_share()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn reward_reserve(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .reward_reserve()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn farming_token_id(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .farming_token_id()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn reward_token_id(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .reward_token_id()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn per_block_reward_amount(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .per_block_reward_amount()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn last_reward_block_nonce(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .last_reward_block_nonce()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn division_safety_constant(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .division_safety_constant()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn user_total_farm_position(&mut self) {
        let user = bech32::decode("");

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .user_total_farm_position(user)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn allow_external_claim(&mut self) {
        let user = bech32::decode("");

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .allow_external_claim(user)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn farm_position_migration_nonce(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .farm_position_migration_nonce()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn set_locking_sc_address(&mut self) {
        let new_address = bech32::decode("");

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .set_locking_sc_address(new_address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn set_lock_epochs(&mut self) {
        let lock_epochs = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .set_lock_epochs(lock_epochs)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn locking_sc_address(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .locking_sc_address()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn lock_epochs(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .lock_epochs()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn register_farm_token(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(0u128);

        let token_display_name = ManagedBuffer::new_from_bytes(&b""[..]);
        let token_ticker = ManagedBuffer::new_from_bytes(&b""[..]);
        let num_decimals = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .register_farm_token(token_display_name, token_ticker, num_decimals)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn farm_token(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .farm_token()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn farm_token_supply(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .farm_token_supply()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn add_to_pause_whitelist(&mut self) {
        let address_list = MultiValueVec::from(vec![bech32::decode("")]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .add_to_pause_whitelist(address_list)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn remove_from_pause_whitelist(&mut self) {
        let address_list = MultiValueVec::from(vec![bech32::decode("")]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .remove_from_pause_whitelist(address_list)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn pause(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .pause()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn resume(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .resume()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn state(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .state()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn add_admin_endpoint(&mut self) {
        let address = bech32::decode("");

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .add_admin_endpoint(address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn remove_admin_endpoint(&mut self) {
        let address = bech32::decode("");

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .remove_admin_endpoint(address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn update_owner_or_admin_endpoint(&mut self) {
        let previous_owner = bech32::decode("");

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .update_owner_or_admin_endpoint(previous_owner)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn permissions(&mut self) {
        let address = bech32::decode("");

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .permissions(address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn add_sc_address_to_whitelist(&mut self) {
        let address = bech32::decode("");

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .add_sc_address_to_whitelist(address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn remove_sc_address_from_whitelist(&mut self) {
        let address = bech32::decode("");

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .remove_sc_address_from_whitelist(address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn is_sc_address_whitelisted(&mut self) {
        let address = bech32::decode("");

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .is_sc_address_whitelisted(address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn set_penalty_percent(&mut self) {
        let percent = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .set_penalty_percent(percent)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn set_minimum_farming_epochs(&mut self) {
        let epochs = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .set_minimum_farming_epochs(epochs)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn set_burn_gas_limit(&mut self) {
        let gas_limit = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .set_burn_gas_limit(gas_limit)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn penalty_percent(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .penalty_percent()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn minimum_farming_epochs(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .minimum_farming_epochs()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn burn_gas_limit(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .burn_gas_limit()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn pair_contract_address(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .pair_contract_address()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn collect_undistributed_boosted_rewards(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .collect_undistributed_boosted_rewards()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn boosted_yields_rewards_percentage(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .boosted_yields_rewards_percentage()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn accumulated_rewards_for_week(&mut self) {
        let week = 0u32;

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .accumulated_rewards_for_week(week)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn farm_supply_for_week(&mut self) {
        let week = 0u32;

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .farm_supply_for_week(week)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn remaining_boosted_rewards_to_distribute(&mut self) {
        let week = 0u32;

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .remaining_boosted_rewards_to_distribute(week)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn undistributed_boosted_rewards(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .undistributed_boosted_rewards()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn set_boosted_yields_factors(&mut self) {
        let max_rewards_factor = BigUint::<StaticApi>::from(0u128);
        let user_rewards_energy_const = BigUint::<StaticApi>::from(0u128);
        let user_rewards_farm_const = BigUint::<StaticApi>::from(0u128);
        let min_energy_amount = BigUint::<StaticApi>::from(0u128);
        let min_farm_amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .set_boosted_yields_factors(max_rewards_factor, user_rewards_energy_const, user_rewards_farm_const, min_energy_amount, min_farm_amount)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn get_boosted_yields_factors(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .get_boosted_yields_factors()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn get_current_week(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .get_current_week()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn first_week_start_epoch(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .first_week_start_epoch()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn get_last_active_week_for_user_view(&mut self) {
        let user = bech32::decode("");

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .get_last_active_week_for_user_view(user)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn get_user_energy_for_week_view(&mut self) {
        let user = bech32::decode("");
        let week = 0u32;

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .get_user_energy_for_week_view(user, week)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn last_global_update_week(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .last_global_update_week()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn total_rewards_for_week(&mut self) {
        let week = 0u32;

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .total_rewards_for_week(week)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn total_energy_for_week(&mut self) {
        let week = 0u32;

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .total_energy_for_week(week)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn total_locked_tokens_for_week(&mut self) {
        let week = 0u32;

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .total_locked_tokens_for_week(week)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn update_energy_for_user(&mut self) {
        let user = bech32::decode("");

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .update_energy_for_user(user)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn current_claim_progress(&mut self) {
        let user = bech32::decode("");

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .current_claim_progress(user)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn set_energy_factory_address(&mut self) {
        let sc_address = bech32::decode("");

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .set_energy_factory_address(sc_address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn energy_factory_address(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmProxy)
            .energy_factory_address()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

}

#[tokio::test]
async fn test_deploy(){
    let mut interact = ContractInteract::new().await;

    interact
        .deploy()
        .await;
}

#[tokio::test]
async fn farm_with_no_boost_no_proxy_test(){
    DebugApi::dummy();

    let mut interact = ContractInteract::new().await;

    let first_farm_token_amount = 100_000_000;
    let first_farm_token_nonce = 1u64;
    let first_user = Bech32Address::from_bech32_string(USER1_ADDRESS.to_string());
    interact
        .enter_farm_endpoint(
            &first_user, 
            first_farm_token_amount
        )
        .await;

    // second user enter farm
    let second_farm_token_amount = 50_000_000;
    let second_farm_token_nonce = 2u64;
    let second_user = Bech32Address::from_bech32_string(USER2_ADDRESS.to_string());
    
    interact
        .enter_farm_endpoint(
            &second_user, 
            first_farm_token_amount
        )
        .await;

    let total_farm_tokens = first_farm_token_amount + second_farm_token_amount;

    let first_attributes = FarmTokenAttributes {
        reward_per_share: managed_biguint!(0),
        entering_epoch: 0,
        compounded_reward: managed_biguint!(0),
        current_farm_amount: managed_biguint!(first_farm_token_amount),
        original_owner: managed_address!(&first_user.as_address()),
    };

    let first_rewards_amt = 
        interact
            .calculate_rewards_for_given_position(
                first_user.clone(),
                first_farm_token_amount,
                first_attributes
            )
            .await;
    let first_expected_rewards_amt = first_farm_token_amount * 10_000 / total_farm_tokens;
    assert_eq!(first_rewards_amt, RustBigUint::from(first_expected_rewards_amt));

    let second_attributes = FarmTokenAttributes {
        reward_per_share: managed_biguint!(0),
        entering_epoch: 0,
        compounded_reward: managed_biguint!(0),
        current_farm_amount: managed_biguint!(second_farm_token_amount),
        original_owner: managed_address!(&second_user.as_address()),
    };

    let second_rewards_amt =
        interact
            .calculate_rewards_for_given_position(
                second_user, 
                second_farm_token_amount, 
                second_attributes)
            .await;
        
    let second_expected_rewards_amt = second_farm_token_amount * 10_000 / total_farm_tokens;
    
    assert_eq!(second_rewards_amt,RustBigUint::from(second_expected_rewards_amt));

    let first_received_reward_amt =
        interact
            .claim_rewards_endpoint(
                &first_user.as_address(),
                1,
                first_farm_token_amount
            )
            .await;
            
}

#[tokio::test]
async fn farm_position_claim_test(){
    let mut interact = ContractInteract::new().await;
    let farm_in_amount = 50_000_000;
    let first_user = Bech32Address::from_bech32_string(USER1_ADDRESS.to_string());
    
    interact
        .enter_farm_endpoint(
            &first_user, 
            farm_in_amount
        )
        .await;

    interact
        .enter_farm_endpoint(
            &first_user, 
            farm_in_amount
        )
        .await;


}