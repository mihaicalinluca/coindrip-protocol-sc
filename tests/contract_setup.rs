use coindrip::coindrip_proxy;
use multiversx_sc_scenario::imports::*;

pub const TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("STRM-df6f26");
pub const CODE_PATH: MxscPath = MxscPath::new("output/coindrip.mxsc.json");
pub const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
pub const FIRST_USER: TestAddress = TestAddress::new("first-user");
pub const SECOND_USER: TestAddress = TestAddress::new("second-user");
pub const THIRD_USER: TestAddress = TestAddress::new("third-user");
pub const COINDRIP_ADDRESS: TestSCAddress = TestSCAddress::new("coindrip");
pub const INITIAL_OWNER_TOKEN_BALANCE: u64 = 5_000_000u64;
pub const CURRENT_TIMESTAMP: u64 = 1668518731u64;

pub fn setup() -> ScenarioWorld {
    let mut world = world();

    world
        .account(OWNER_ADDRESS)
        .balance(101)
        .esdt_balance(TOKEN_ID, INITIAL_OWNER_TOKEN_BALANCE);
    world.account(FIRST_USER);
    world.account(SECOND_USER);
    world.account(THIRD_USER);

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .init()
        .code(CODE_PATH)
        .new_address(COINDRIP_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(new_address, COINDRIP_ADDRESS.to_address());

    world
}

pub fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(CODE_PATH, coindrip::ContractBuilder);
    blockchain
}
