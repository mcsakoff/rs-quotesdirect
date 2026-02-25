//! # Functions to generate FIX messages for Quotes Direct API
//!
//! Documentation:
//!  - [External Interfaces & Protocols](https://help.cqg.com/apihelp/#!Documents/externalinterfacesprotocols.htm)
//!  - [Preamble and Header Formats](https://help.cqg.com/apihelp/#!Documents/preambleandheaderformats.htm)
//!
use chrono::Utc;

const SOH: &str = "\x01"; // Start Of Heading

/// This message type is used to logon to Security Definition Server and Replay Server.
///
/// [Session/Administrative Messages](https://help.cqg.com/apihelp/#!Documents/sessionadministrativemessagesquotesdirect.htm)
///
/// # Examples
///
/// ```
/// use quotesdirectlib::fix::login;
/// let msg = login(1, "user", "password", 60);
/// ```
#[must_use]
pub fn login(sequence: u32, user: &str, password: &str, heartbeat_interval_sec: u32) -> Vec<u8> {
    let header = header(sequence, "A");
    fixit(&format!(
        "{header}|108={heartbeat_interval_sec}|553={user}|554={password}"
    ))
}

/// This message type is used to logout from Security Definition Server and Replay Server.
///
/// [Session/Administrative Messages](https://help.cqg.com/apihelp/#!Documents/sessionadministrativemessagesquotesdirect.htm)
///
/// # Examples
///
/// ```
/// use quotesdirectlib::fix::logout;
/// let msg = logout(2, "application terminated");
/// ```
#[must_use]
pub fn logout(sequence: u32, message: &str) -> Vec<u8> {
    let header = header(sequence, "5");
    fixit(&format!("{header}|58={message}"))
}

/// Security Definition Request message from customer to API
///
/// [Security Definition Request](https://help.cqg.com/apihelp/#!Documents/securitydefinitionrequestcfromcustomertocqg.htm)
///
/// # Examples
///
/// ```
/// use quotesdirectlib::fix::request;
/// let msg = request(3, 85);
/// ```
#[must_use]
pub fn request(sequence: u32, feed_id: u32) -> Vec<u8> {
    let header = header(sequence, "c");
    fixit(&format!("{header}|1180={feed_id}"))
}

fn header(sequence: u32, msg_type: &str) -> String {
    let sending_time = Utc::now().format("%Y%m%d-%H:%M:%S").to_string();
    format!("35={msg_type}|34={sequence}|49=CQG|52={sending_time}")
}

fn fixit(text: &str) -> Vec<u8> {
    let length = text.len() + 1;
    let text = format!("8=FIX.5.0.SP2|9={length}|{text}|").replace('|', SOH);
    let crc = text
        .as_bytes()
        .iter()
        .fold(0u8, |acc, c| acc.wrapping_add(*c));
    format!("{text}10={crc}\x01").as_bytes().to_vec()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_crc() {
        let msg = "35=A|34=1|49=CQG|52=20250620-10:22:47|108=60|553=user|554=password".to_owned();
        let raw = "8=FIX.5.0.SP2|9=67|35=A|34=1|49=CQG|52=20250620-10:22:47|108=60|553=user|554=password|10=117|".to_owned()
            .replace('|', SOH);

        assert_eq!(fixit(&msg), raw.as_bytes());
    }

    fn fix_to_hashmap(raw: Vec<u8>) -> HashMap<String, String> {
        String::from_utf8(raw)
            .unwrap()
            .split(SOH)
            .filter_map(|s| {
                if s.is_empty() {
                    None
                } else {
                    let (key, val) = s.split_once('=').unwrap();
                    Some((String::from(key), String::from(val)))
                }
            })
            .collect()
    }

    #[test]
    fn test_login() {
        let raw = login(1, "user", "password", 60);
        // 8=FIX.5.0.SP2|9=67|35=A|34=1|49=CQG|52=20250620-10:22:47|108=60|553=user|554=password|10=117|
        let msg = fix_to_hashmap(raw);
        assert_eq!(msg["8"], "FIX.5.0.SP2");
        assert_eq!(msg["9"], "67");
        assert_eq!(msg["35"], "A");
        assert_eq!(msg["34"], "1");
        assert_eq!(msg["49"], "CQG");
        assert_eq!(msg["108"], "60");
        assert_eq!(msg["553"], "user");
        assert_eq!(msg["554"], "password");
    }

    #[test]
    fn test_logout() {
        let raw = logout(2, "Logout");
        // 8=FIX.5.0.SP2|9=48|35=5|34=2|49=CQG|52=20250620-10:51:03|58=Logout|10=98|
        let msg = fix_to_hashmap(raw);
        assert_eq!(msg["8"], "FIX.5.0.SP2");
        assert_eq!(msg["9"], "48");
        assert_eq!(msg["35"], "5");
        assert_eq!(msg["34"], "2");
        assert_eq!(msg["49"], "CQG");
        assert_eq!(msg["58"], "Logout");
    }

    #[test]
    fn test_request() {
        let raw = request(3, 85);
        // 8=FIX.5.0.SP2|9=46|35=c|34=3|49=CQG|52=20250620-10:53:14|1180=85|10=227|
        let msg = fix_to_hashmap(raw);
        assert_eq!(msg["8"], "FIX.5.0.SP2");
        assert_eq!(msg["9"], "46");
        assert_eq!(msg["35"], "c");
        assert_eq!(msg["34"], "3");
        assert_eq!(msg["49"], "CQG");
        assert_eq!(msg["1180"], "85");
    }
}
