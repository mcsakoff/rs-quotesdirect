//! # FAST messages definitions for Quotes Direct API
//!
//! The messages definitions represents the messages used by the Quotes Direct API
//! and defined in the `templates.xml` file. It uses the `fastlib` and `serde` crates
//! for the serialization and deserialization of the messages.
//!
//! Message reference:
//! - [Session/Administrative Messages](https://help.cqg.com/apihelp/#!Documents/sessionadministrativemessagesquotesdirect.htm)
//! - [Application Messages](https://help.cqg.com/apihelp/#!Documents/applicationmessagesquotesdirect.htm)
//!
//! ## Usage
//!
//! ```rust,ignore
//! use fastlib::Decoder;
//! use quotesdirectlib::fast::{Message, TEMPLATES_XML};
//!
//! // Create a decoder from XML templates.
//! let mut decoder = Decoder::new_from_xml(TEMPLATES_XML)?;
//!
//! // Raw data that contains one message.
//! let raw_data: Vec<u8> = vec![ ... ];
//!
//! // Deserialize a message.
//! let msg: Message = fastlib::from_slice(&mut decoder, &raw_data)?;
//! // Process the message.
//! match msg {
//!     Message::MDIncRefresh(_) => {}
//!     Message::MDSecurityDefinition(_) => {}
//!     Message::MDSnapshotFullRefresh(_) => {}
//!     Message::MDSecurityDefinitionRequest(_) => {}
//!     Message::MDSecurityStatus(_) => {}
//!     _ => {
//!         // Other messages
//!     }
//! }
//! ```
//!
use fastlib::Decimal;
use serde::{Deserialize, Serialize};

pub const TEMPLATES_XML: &str = include_str!("../templates.xml");

//
// Quotes Direct messages
//
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Message {
    MDIncRefresh(IncRefresh),
    MDSecurityDefinition(SecurityDefinition),
    MDSnapshotFullRefresh(SnapshotFullRefresh),
    MDHeartbeat(Heartbeat),
    MDLogon(Logon),
    MDLogout(Logout),
    MDSecurityDefinitionRequest(SecurityDefinitionRequest),
    SequenceReset(SequenceReset),
    MDSecurityStatus(SecurityStatus),
    News(News),
    ApplicationMessageRequestAck(ApplicationMessageRequestAck),
    UserNotification(UserNotification),
}

//
// <template dictionary="1" id="1" name="MDIncRefresh" />
//
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct IncRefresh {
    pub message_type: String,
    #[serde(flatten)]
    pub msg_header: MsgHeader,
    pub trade_date: Option<u32>,
    #[serde(rename = "MDEntries")]
    pub md_entries: Vec<MDEntry>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MDEntry {
    #[serde(rename = "MDUpdateAction")]
    pub md_update_action: Option<u32>,
    #[serde(rename = "MDPriceLevel")]
    pub md_price_level: Option<u32>,
    #[serde(rename = "MDEntryType")]
    pub md_entry_type: String,
    #[serde(rename = "SecurityID")]
    pub security_id: u32,
    #[serde(rename = "SecurityIDSource")]
    pub security_id_source: u32,
    pub rpt_seq: u32,
    #[serde(rename = "MDEntryPx")]
    pub md_entry_px: Option<Decimal>,
    #[serde(rename = "MDEntryTime")]
    pub md_entry_time: u32,
    #[serde(rename = "MDEntrySize")]
    pub md_entry_size: Option<i32>,
    pub quote_condition: Option<String>,
    #[serde(rename = "MDQuoteType")]
    pub md_quote_type: Option<u32>,
    pub trade_condition: Option<String>,
    pub trade_volume: Option<u32>,
    pub aggressor_side: Option<u32>,
    #[serde(rename = "MDWorkupState")]
    pub md_workup_state: Option<u32>,
    pub parties: Option<Vec<Party>>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct Party {
    #[serde(rename = "PartyID")]
    pub party_id: u32,
    #[serde(rename = "PartyIDSource")]
    pub party_id_source: String,
}

//
// <template dictionary="2" id="2" name="MDSecurityDefinition" />
//
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SecurityDefinition {
    pub message_type: String,
    #[serde(flatten)]
    pub msg_header: MsgHeader,
    pub tot_num_reports: u32,
    pub events: Option<Vec<Event>>,
    pub security_group: Option<String>,
    pub symbol: Option<String>,
    pub security_name: String,
    pub security_desc: String,
    #[serde(rename = "SecurityID")]
    pub security_id: u32,
    #[serde(rename = "SecurityIDSource")]
    pub security_id_source: u32,
    #[serde(rename = "CFICode")]
    pub cfi_code: String,
    pub security_exchange: Option<String>,
    #[serde(rename = "CQGSecurityName")]
    pub cqg_security_name: Option<String>,
    pub strike_price: Option<Decimal>,
    pub strike_currency: Option<String>,
    pub currency: Option<String>,
    pub settl_currency: Option<String>,
    #[serde(rename = "MDFeedTypes")]
    pub md_feed_types: Option<Vec<FeedType>>,
    pub instr_attrib: Option<Vec<InstrAttrib>>,
    pub maturity_month_year: Option<u64>,
    pub min_price_increment: Option<f64>,
    pub min_price_increment_amount: Option<f64>,
    pub display_factor: Option<Decimal>,
    #[serde(rename = "ApplID")]
    pub appl_id: String,
    pub most_active_flag: Option<String>,
    pub connections: Vec<Connection>,
    pub trading_sessions: Vec<TradingSession>,
    pub underlyings: Option<Vec<Underlying>>,
    pub security_sub_type: Option<String>,
    pub legs: Option<Vec<Leg>>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Event {
    pub event_type: u32,
    pub event_date: u64,
    pub event_time: u64,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FeedType {
    #[serde(rename = "MDFeedType")]
    pub feed_type: String,
    pub market_depth: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct InstrAttrib {
    pub instr_attrib_type: u64,
    pub instr_attrib_value: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Connection {
    pub connection_type: u32,
    #[serde(rename = "ConnectionIPAddress")]
    pub connection_ip_address: String,
    pub connection_port_number: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TradingSession {
    pub trade_date: u64,
    pub trad_ses_start_time: u64,
    pub trad_ses_open_time: u64,
    pub trad_ses_close_time: u64,
    pub trad_ses_end_time: u64,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Underlying {
    #[serde(rename = "UnderlyingSecurityID")]
    pub security_id: u32,
    #[serde(rename = "UnderlyingSecurityIDSource")]
    pub security_id_source: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Leg {
    pub leg_symbol: String,
    pub leg_security_desc: String,
    pub leg_ratio_qty: Decimal,
    #[serde(rename = "LegSecurityID")]
    pub leg_security_id: u32,
    #[serde(rename = "LegSecurityIDSource")]
    pub leg_security_id_source: u32,
    pub leg_side: u32,
    pub leg_security_group: String,
    #[serde(rename = "LegCFICode")]
    pub leg_cfi_code: String,
    pub leg_currency: String,
    pub leg_maturity_month_year: u64,
    pub leg_strike_price: Decimal,
}

//
// <template dictionary="3" id="3" name="MDSnapshotFullRefresh">
//
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SnapshotFullRefresh {
    pub message_type: String,
    #[serde(flatten)]
    pub msg_header: MsgHeader,
    pub last_msg_seq_num_processed: u32,
    pub tot_num_reports: u32,
    pub rpt_seq: u32,
    #[serde(rename = "SecurityID")]
    pub security_id: u32,
    #[serde(rename = "SecurityIDSource")]
    pub security_id_source: u32,
    #[serde(rename = "MDSecurityTradingStatus")]
    pub md_security_trading_status: Option<u32>,
    #[serde(rename = "MDEntries")]
    pub md_entries: Vec<MDEntrySnapshot>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct MDEntrySnapshot {
    #[serde(rename = "MDEntryType")]
    pub md_entry_type: String,
    #[serde(rename = "MDEntryPx")]
    pub md_entry_px: Option<Decimal>,
    #[serde(rename = "MDEntrySize")]
    pub md_entry_size: Option<i32>,
    pub quote_condition: Option<String>,
    #[serde(rename = "MDPriceLevel")]
    pub md_price_level: Option<u32>,
    #[serde(rename = "MDWorkupState")]
    pub md_workup_state: Option<u32>,
}

//
// <template dictionary="4" id="4" name="MDHeartbeat" />
//
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Heartbeat {
    pub message_type: String,
    #[serde(flatten)]
    pub msg_header: MsgHeader,
}

//
// <template dictionary="5" id="5" name="MDLogon" />
//
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Logon {
    pub message_type: String,
    #[serde(flatten)]
    pub msg_header: MsgHeader,
    pub encrypt_method: u32,
    pub heartbeat_int: u32,
}

//
// <template dictionary="6" id="6" name="MDLogout" />
//
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Logout {
    pub message_type: String,
    #[serde(flatten)]
    pub msg_header: MsgHeader,
    pub text: Option<String>,
}

//
// <template dictionary="7" id="7" name="MDSecurityDefinitionRequest">
//
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SecurityDefinitionRequest {
    pub message_type: String,
    #[serde(flatten)]
    pub msg_header: MsgHeader,
    #[serde(rename = "ApplID")]
    pub appl_id: String,
    pub text: Option<String>,
}

//
// <template dictionary="8" id="8" name="SequenceReset">
//
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SequenceReset {
    pub message_type: String,
    #[serde(flatten)]
    pub msg_header: MsgHeader,
    pub new_seq_no: u32,
}

//
// <template dictionary="9" id="9" name="MDSecurityStatus" />
//
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SecurityStatus {
    pub message_type: String,
    #[serde(flatten)]
    pub msg_header: MsgHeader,
    #[serde(rename = "SecurityID")]
    pub security_id: Option<u32>,
    #[serde(rename = "SecurityIDSource")]
    pub security_id_source: Option<u32>,
    pub symbol: Option<String>,
    pub security_trading_status: Option<u32>,
}

//
// <template dictionary="10" id="10" name="News" />
//
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct News {
    pub message_type: String,
    #[serde(flatten)]
    pub msg_header: MsgHeader,
    pub message_encoding: String,
    #[serde(rename = "ApplID")]
    pub appl_id: String,
    #[serde(rename = "NewsID")]
    pub news_id: String,
    #[serde(rename = "NewsSourceID")]
    pub news_source_id: u32,
    pub last_fragment: Option<String>,
    #[serde(rename = "NewsRefIDs")]
    pub news_ref_ids: Option<Vec<NewsRefID>>,
    pub orig_time: Option<u64>,
    pub urgency: Option<String>,
    pub news_branding: Option<String>,
    pub accession_number: Option<String>,
    pub encoded_headline: Option<Vec<u8>>,
    pub encoded_text: Option<Vec<u8>>,
    pub news_categories: Option<Vec<NewsCategory>>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct NewsRefID {
    #[serde(rename = "NewsRefID")]
    pub news_ref_id: String,
    #[serde(rename = "NewsRefType")]
    pub news_ref_type: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct NewsCategory {
    pub category_class: u32,
    pub category_code: String,
}

//
// <template dictionary="11" id="11" name="ApplicationMessageRequestAck" />
//
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ApplicationMessageRequestAck {
    pub message_type: String,
    #[serde(flatten)]
    pub msg_header: MsgHeader,
    #[serde(rename = "ApplResponseID")]
    pub appl_response_id: String,
    #[serde(rename = "ApplReqID")]
    pub appl_req_id: String,
    #[serde(rename = "ApplIDs")]
    pub appl_ids: Vec<ApplID>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ApplID {
    #[serde(rename = "RefApplID")]
    pub ref_appl_id: String,
    pub appl_response_error: Option<u32>,
    pub raw_data: Option<Vec<u8>>,
    #[serde(rename = "NewsSourceID")]
    pub news_source_id: Option<u32>,
    pub connections: Vec<Connection>,
}

//
// <template dictionary="12" id="12" name="UserNotification" />
//
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UserNotification {
    pub message_type: String,
    #[serde(flatten)]
    pub msg_header: MsgHeader,
    pub user_status: u32,
    pub text: String,
}

//
// <template name="MsgHeader" />
//
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct MsgHeader {
    #[serde(rename = "ApplVerID")]
    pub appl_ver_id: String,
    #[serde(rename = "SenderCompID")]
    pub sender_comp_id: String,
    #[serde(rename = "MsgSeqNum")]
    pub msg_seq_num: u32,
    #[serde(rename = "SendingTime")]
    pub sending_time: u64,
}
