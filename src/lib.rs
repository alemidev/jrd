//! # JRD
//!
//! from [Packetizer](https://www.packetizer.com/json/jrd/):
//! > The JSON Resource Descriptor (JRD) is a simple [JSON](https://www.techabulary.com/j/json/) object that describes a "resource" on the Internet,
//! > where a "resource" is any entity on the Internet that is identified via a URI or IRI.
//! > For example, a person's account URI (e.g., acct:bob@example.com) is a resource.
//! > So are all web URIs (e.g., https://www.packetizer.com/).
//!
//!
//! > The JSON Resource Descriptor, originally introduced in [RFC 6415](https://www.packetizer.com/rfc/rfc6415/) and 
//! > based on the Extensible Resource Descriptor (XRD) format, was adopted for use 
//! > in the [WebFinger](https://www.techabulary.com/w/webfinger/) protocol, though its use is not restricted 
//! > to WebFinger or RFC 6415. 
//!
//! This tiny crate provides a struct representation of JRDs, [JsonResourceDescriptor], together with
//! [serde::Serialize] and [serde::Deserialize] implementations.
//!
//! All documentation is copied as-is from packetizer.com.
//!
//! # usage
//! ```rust
//! let jrd_string = r#"{
//!   "subject": "acct:paulej@packetizer.com",
//!   "properties": {
//!     "http://packetizer.com/ns/name": "Paul E. Jones"
//!   },
//!   "links": [
//!     {
//!       "rel": "http://webfinger.net/rel/profile-page",
//!       "href": "http://www.packetizer.com/people/paulej/"
//!     },
//!     {
//!       "rel": "http://packetizer.com/rel/blog",
//!       "type": "text/html",
//!       "href": "http://www.packetizer.com/people/paulej/blog/",
//!       "titles": {
//!         "en-us": "Paul E. Jones' Blog"
//!       }
//!     }
//!   ]
//! }"#;
//! 
//! let jrd_struct = jrd::JsonResourceDescriptor {
//!   subject: "acct:paulej@packetizer.com".into(),
//!   aliases: Vec::new(),
//!   properties: [("http://packetizer.com/ns/name".to_string(), "Paul E. Jones".to_string())].into(),
//!   expires: None,
//!   links: vec![
//!     jrd::JsonResourceDescriptorLink {
//!       rel: "http://webfinger.net/rel/profile-page".into(),
//!       href: Some("http://www.packetizer.com/people/paulej/".into()),
//!       link_type: None,
//!       titles: jrd::Map::default(),
//!       properties: jrd::Map::default(),
//!     },
//!     jrd::JsonResourceDescriptorLink {
//!       rel: "http://packetizer.com/rel/blog".into(),
//!       href: Some("http://www.packetizer.com/people/paulej/blog/".into()),
//!       link_type: Some("text/html".into()),
//!       titles: [("en-us".to_string(), "Paul E. Jones' Blog".to_string())].into(),
//!       properties: jrd::Map::default(),
//!     },
//!   ],
//! };
//!
//! // deserialize
//! assert_eq!(serde_json::from_str::<jrd::JsonResourceDescriptor>(jrd_string).unwrap(), jrd_struct);
//!
//! // serialize
//! assert_eq!(serde_json::to_string_pretty(&jrd_struct).unwrap(), jrd_string)
//! ```


pub type Map = std::collections::BTreeMap<String, String>;
pub type Time = chrono::DateTime<chrono::Utc>;


/// The JSON Resource Descriptor (JRD) is a simple JSON object that describes a "resource" on the Internet, where a "resource" is any entity on the Internet that is identified via a URI or IRI.
/// 
/// For example, a person's account URI (e.g., acct:bob@example.com) is a resource. So are all web URIs (e.g., https://www.packetizer.com/).
/// The JSON Resource Descriptor, originally introduced in [RFC 6415](https://www.packetizer.com/rfc/rfc6415/) and based on the Extensible Resource Descriptor (XRD) format,
/// was adopted for use in the WebFinger protocol, though its use is not restricted to WebFinger or [RFC 6415](https://www.packetizer.com/rfc/rfc6415/).
/// A JRD object comprises the following name/value pairs:
///  * expires
///  * subject
///  * aliases
///  * properties
///  * links
///
/// A JRD describes a URI or IRI by returning structured information about the identifier.
/// For example, a JRD that describes a person named Paul might include information about Paul's full name, homepage, blog, etc.
///
/// Here is an example JRD that describes a user named Paul:
/// ```json
/// {
///   "subject" : "acct:paulej@packetizer.com",
///   "properties" :
///   {
///     "http://packetizer.com/ns/name" : "Paul E. Jones"
///   },
///   "links" :
///   [
///     {
///       "rel" : "http://webfinger.net/rel/profile-page",
///       "href" : "http://www.packetizer.com/people/paulej/"
///     },
///     {
///       "rel" : "http://packetizer.com/rel/blog",
///       "type" : "text/html",
///       "href" : "http://www.packetizer.com/people/paulej/blog/",
///       "titles" :
///       {
///         "en-us" : "Paul E. Jones' Blog"
///       }
///     }
///   ]
/// }
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct JsonResourceDescriptor {
	/// The value of the "subject" member is an URI that identifies the entity that the JRD describes.
	/// The “subject” member SHOULD be present in the JRD. 
	pub subject: String,

	/// The “aliases” array is an array of zero or more URI strings that identify the same entity as the “subject” URI.
	/// The “aliases” member is OPTIONAL in the JRD. 
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub aliases: Vec<String>,

	/// The “properties” object comprises zero or more name/value pairs whose names are URIs (referred to as “property identifiers”) and whose values are strings or null.
	///
	/// Properties are used to convey additional information about the subject of the JRD. As an example, consider this use of “properties”:
	///  ```json
	///      "properties" : { "http://packetizer.com/ns/name" : "Bob Smith" }
	///  ```
	/// The “properties” member is optional. 
	#[serde(default, skip_serializing_if = "Map::is_empty")]
	pub properties: Map,

	/// The value of the “expires” member is a string that indicates the date and time after which the JRD SHOULD be considered expired and no longer utilized.
	///
	/// This format is formally defined in RFC 3339. The “expires” member MUST NOT use fractional seconds and MUST express time only Universal Coordinate Time via the “Z” designation on the end of the string.
	/// An example of the “expires” member is:
	/// ```json
	///     "expires" : "2012-11-16T19:41:35Z"
	/// ```
	///
	/// The “expires” member is OPTIONAL in the JRD, but should be honored if present.
	/// Note: The “expires” member is not defined for use with WebFinger, but is defined for Host Metadata ([RFC 6415](https://www.packetizer.com/rfc/rfc6415/)). For WebFinger, this member MUST NOT be transmitted and ignored if received. 
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expires: Option<Time>,

	/// The “links” array has any number of member objects (see [JsonResourceDescriptorLink]), each of which represents a link.
	///
	/// The “links” array is OPTIONAL in the JRD.
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub links: Vec<JsonResourceDescriptorLink>,
}

/// Each of these link objects can have the following members:
///  * rel
///  * type
///  * href
///  * titles
///  * properties
///
/// The “rel” and “href” members are strings representing the link's relation type and the target URI, respectively. The context of the link is the “subject”.
/// The “type” member is a string indicating what the media type of the result of dereferencing the link ought to be.
/// The order of elements in the “links” array MAY be interpreted as indicating an of preference. Thus, if there are two or more link relations having the same “rel” value, the first link relation would indicate the user’s preferred link.
#[derive(Debug, Clone, Eq, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct JsonResourceDescriptorLink {
	/// The value of the “rel” member is a string that is either a URI or a registered relation type (see [RFC 8288](https://www.packetizer.com/rfc/rfc8288/)).
	///
	/// The value of the “rel” member MUST contain exactly one URI or registered relation type. The URI or registered relation type identifies the type of the link relation.
	/// The other members of the object have meaning only once the type of link relation is understood.
	/// In some instances, the link relation will have associated semantics enabling the client to query for other resources on the Internet.
	/// In other instances, the link relation will have associated semantics enabling the client to utilize the other members of the link relation object without fetching additional external resources.
	/// URI link relation type values are compared using the "Simple String Comparison" algorithm of Section 6.2.1 of [RFC 3986](https://www.packetizer.com/rfc/rfc3986/).
	/// 
	/// The “rel” member MUST be present in a link relation object. 
	pub rel: String,


	/// The value of the “type” member is a string that indicates the media type of the target resource (see [RFC 6838](https://www.packetizer.com/rfc/rfc6838/)).
	/// The “type” member is OPTIONAL in a link relation object. 
	#[serde(rename = "type", skip_serializing_if = "Option::is_none")]
	pub link_type: Option<String>,

	/// The value of the “href” member is a string that contains an URI pointing to the linked resource.
	/// The “href” member is optional in the link relation object. 
	#[serde(skip_serializing_if = "Option::is_none")]
	pub href: Option<String>,

	/// The “titles” object comprises zero or more name/value pairs whose names are a language tag or the string “und”.
	///
	/// The string is human-readable and describes the link relation.
	/// More than one title for the link relation MAY be provided for the benefit of users who utilize the link relation, and, if used, a language identifier SHOULD be duly used as the name.
	/// If the language is unknown or unspecified, then the name is “und”.
	/// 
	/// A JRD SHOULD NOT include more than one title named with the same language tag (or “und”) within the link relation object.
	/// Meaning is undefined if a link relation object includes more than one title identified with the same language tag (or “und”), though this MUST NOT be treated as an error.
	/// A client MAY select whichever title or titles it wishes to utilize.
	/// 
	/// Here is an example of the “titles” object:
	/// ```json
	///   "titles" :
	///   {
	///     "und" : "The Magical World of Steve",
	///     "en-us" : "The Magical World of Steve",
	///     "fr" : "Le Monde Magique de Steve"
	///   }
	/// ```
	/// The “titles” member is OPTIONAL in a link relation object. 
	#[serde(default, skip_serializing_if = "Map::is_empty")]
	pub titles: Map,

	/// The “properties” object within the link relation object comprises zero or more name/value pairs whose names are URIs (referred to as “property identifiers”) and whose values are strings or null.
	///
	/// Properties are used to convey additional information about the link relation.
	/// As an example, consider this use of “properties”:
	/// ```json
	///   "properties" : { "http://packetizer.com/ns/port" : "993" }
	/// ```
	/// The “properties” member is optional in a link relation object. 
	#[serde(default, skip_serializing_if = "Map::is_empty")]
	pub properties: Map,
}
