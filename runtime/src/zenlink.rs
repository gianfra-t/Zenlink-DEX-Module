
use super::{
    parameter_types, Convert, ModuleId, Vec, vec,
    Runtime, MessageBroker, Balances, ZenlinkProtocol,
    AccountId, ParachainInfo, Origin, Call, Event
};

use zenlink_protocol::{
    Sibling, AccountId32Aliases, ParaChainWhiteList, Junction, LocationInverter,
    MultiLocation, NetworkId, Origin as ZenlinkOrigin, ParentIsDefault,
    RelayChainAsNative, SiblingParachainAsNative, SiblingParachainConvertsVia,
    SignedAccountId32AsNative, SovereignSignedViaLocation, Transactor, XcmCfg, XcmExecutor
};

parameter_types! {
    pub const RococoNetwork: NetworkId = NetworkId::Polkadot;
    pub const DEXModuleId: ModuleId = ModuleId(*b"zlk_dex2");
    pub RelayChainOrigin: Origin = ZenlinkOrigin::Relay.into();
    pub Ancestry: MultiLocation = Junction::Parachain {
        id: ParachainInfo::parachain_id().into()
    }.into();

    pub SiblingParachains: Vec<MultiLocation> = vec![
        MultiLocation::X2(Junction::Parent, Junction::Parachain { id: 200 }),
        MultiLocation::X2(Junction::Parent, Junction::Parachain { id: 300 })
    ];
}

pub struct AccountId32Converter;
impl Convert<AccountId, [u8; 32]> for AccountId32Converter {
    fn convert(account_id: AccountId) -> [u8; 32] {
        account_id.into()
    }
}

type LocationConverter = (
    ParentIsDefault<AccountId>,
    SiblingParachainConvertsVia<Sibling, AccountId>,
    AccountId32Aliases<RococoNetwork, AccountId>,
);

pub type LocalAssetTransactor = Transactor<
    Balances,
    ZenlinkProtocol,
    LocationConverter,
    AccountId,
    ParachainInfo
>;

type LocalOriginConverter = (
    SovereignSignedViaLocation<LocationConverter, Origin>,
    RelayChainAsNative<RelayChainOrigin, Origin>,
    SiblingParachainAsNative<ZenlinkOrigin, Origin>,
    SignedAccountId32AsNative<RococoNetwork, Origin>,
);

pub struct XcmConfig;

impl XcmCfg for XcmConfig {
    type Call = Call;
    type XcmSender = ZenlinkProtocol;
    // How to withdraw and deposit an asset.
    type AssetTransactor = LocalAssetTransactor;
    type OriginConverter = LocalOriginConverter;
    type IsReserve = ParaChainWhiteList<SiblingParachains>;
    type IsTeleporter = ();
    type LocationInverter = LocationInverter<Ancestry>;
}


impl zenlink_protocol::Config for Runtime {
    type Event = Event;
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type UpwardMessageSender = MessageBroker;
    type HrmpMessageSender = MessageBroker;
    type NativeCurrency = Balances;
    type AccountIdConverter = LocationConverter;
    type AccountId32Converter = AccountId32Converter;
    type ParaId = ParachainInfo;
    type ModuleId = DEXModuleId;
}
