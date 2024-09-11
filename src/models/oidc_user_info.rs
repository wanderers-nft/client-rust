/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.15.0
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// OidcUserInfo : OpenID Connect Userinfo
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OidcUserInfo {
    /// End-User's birthday, represented as an ISO 8601:2004 [ISO8601‑2004] YYYY-MM-DD format. The year MAY be 0000, indicating that it is omitted. To represent only the year, YYYY format is allowed. Note that depending on the underlying platform's date related function, providing just year can result in varying month and day, so the implementers need to take this factor into account to correctly process the dates.
    #[serde(rename = "birthdate", skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<String>,
    /// End-User's preferred e-mail address. Its value MUST conform to the RFC 5322 [RFC5322] addr-spec syntax. The RP MUST NOT rely upon this value being unique, as discussed in Section 5.7.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// True if the End-User's e-mail address has been verified; otherwise false. When this Claim Value is true, this means that the OP took affirmative steps to ensure that this e-mail address was controlled by the End-User at the time the verification was performed. The means by which an e-mail address is verified is context-specific, and dependent upon the trust framework or contractual agreements within which the parties are operating.
    #[serde(rename = "email_verified", skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
    /// Surname(s) or last name(s) of the End-User. Note that in some cultures, people can have multiple family names or no family name; all can be present, with the names being separated by space characters.
    #[serde(rename = "family_name", skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    /// End-User's gender. Values defined by this specification are female and male. Other values MAY be used when neither of the defined values are applicable.
    #[serde(rename = "gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// Given name(s) or first name(s) of the End-User. Note that in some cultures, people can have multiple given names; all can be present, with the names being separated by space characters.
    #[serde(rename = "given_name", skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    /// End-User's locale, represented as a BCP47 [RFC5646] language tag. This is typically an ISO 639-1 Alpha-2 [ISO639‑1] language code in lowercase and an ISO 3166-1 Alpha-2 [ISO3166‑1] country code in uppercase, separated by a dash. For example, en-US or fr-CA. As a compatibility note, some implementations have used an underscore as the separator rather than a dash, for example, en_US; Relying Parties MAY choose to accept this locale syntax as well.
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// Middle name(s) of the End-User. Note that in some cultures, people can have multiple middle names; all can be present, with the names being separated by space characters. Also note that in some cultures, middle names are not used.
    #[serde(rename = "middle_name", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    /// End-User's full name in displayable form including all name parts, possibly including titles and suffixes, ordered according to the End-User's locale and preferences.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Casual name of the End-User that may or may not be the same as the given_name. For instance, a nickname value of Mike might be returned alongside a given_name value of Michael.
    #[serde(rename = "nickname", skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// End-User's preferred telephone number. E.164 [E.164] is RECOMMENDED as the format of this Claim, for example, +1 (425) 555-1212 or +56 (2) 687 2400. If the phone number contains an extension, it is RECOMMENDED that the extension be represented using the RFC 3966 [RFC3966] extension syntax, for example, +1 (604) 555-1234;ext=5678.
    #[serde(rename = "phone_number", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// True if the End-User's phone number has been verified; otherwise false. When this Claim Value is true, this means that the OP took affirmative steps to ensure that this phone number was controlled by the End-User at the time the verification was performed. The means by which a phone number is verified is context-specific, and dependent upon the trust framework or contractual agreements within which the parties are operating. When true, the phone_number Claim MUST be in E.164 format and any extensions MUST be represented in RFC 3966 format.
    #[serde(rename = "phone_number_verified", skip_serializing_if = "Option::is_none")]
    pub phone_number_verified: Option<bool>,
    /// URL of the End-User's profile picture. This URL MUST refer to an image file (for example, a PNG, JPEG, or GIF image file), rather than to a Web page containing an image. Note that this URL SHOULD specifically reference a profile photo of the End-User suitable for displaying when describing the End-User, rather than an arbitrary photo taken by the End-User.
    #[serde(rename = "picture", skip_serializing_if = "Option::is_none")]
    pub picture: Option<String>,
    /// Non-unique shorthand name by which the End-User wishes to be referred to at the RP, such as janedoe or j.doe. This value MAY be any valid JSON string including special characters such as @, /, or whitespace.
    #[serde(rename = "preferred_username", skip_serializing_if = "Option::is_none")]
    pub preferred_username: Option<String>,
    /// URL of the End-User's profile page. The contents of this Web page SHOULD be about the End-User.
    #[serde(rename = "profile", skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    /// Subject - Identifier for the End-User at the IssuerURL.
    #[serde(rename = "sub", skip_serializing_if = "Option::is_none")]
    pub sub: Option<String>,
    /// Time the End-User's information was last updated. Its value is a JSON number representing the number of seconds from 1970-01-01T0:0:0Z as measured in UTC until the date/time.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
    /// URL of the End-User's Web page or blog. This Web page SHOULD contain information published by the End-User or an organization that the End-User is affiliated with.
    #[serde(rename = "website", skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    /// String from zoneinfo [zoneinfo] time zone database representing the End-User's time zone. For example, Europe/Paris or America/Los_Angeles.
    #[serde(rename = "zoneinfo", skip_serializing_if = "Option::is_none")]
    pub zoneinfo: Option<String>,
}

impl OidcUserInfo {
    /// OpenID Connect Userinfo
    pub fn new() -> OidcUserInfo {
        OidcUserInfo {
            birthdate: None,
            email: None,
            email_verified: None,
            family_name: None,
            gender: None,
            given_name: None,
            locale: None,
            middle_name: None,
            name: None,
            nickname: None,
            phone_number: None,
            phone_number_verified: None,
            picture: None,
            preferred_username: None,
            profile: None,
            sub: None,
            updated_at: None,
            website: None,
            zoneinfo: None,
        }
    }
}

