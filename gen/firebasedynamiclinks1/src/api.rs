use std::collections::HashMap;
use std::cell::RefCell;
use std::default::Default;
use std::collections::BTreeMap;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;
use std::thread::sleep;

use crate::client;

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// View and administer all your Firebase data and settings
    Firebase,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Firebase => "https://www.googleapis.com/auth/firebase",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Firebase
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all FirebaseDynamicLinks related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_firebasedynamiclinks1 as firebasedynamiclinks1;
/// use firebasedynamiclinks1::api::GetIosPostInstallAttributionRequest;
/// use firebasedynamiclinks1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use firebasedynamiclinks1::FirebaseDynamicLinks;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = FirebaseDynamicLinks::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GetIosPostInstallAttributionRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().install_attribution(req)
///              .doit().await;
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::Io(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
pub struct FirebaseDynamicLinks<> {
    client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>,
    auth: oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, > client::Hub for FirebaseDynamicLinks<> {}

impl<'a, > FirebaseDynamicLinks<> {

    pub fn new(client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>, authenticator: oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>) -> FirebaseDynamicLinks<> {
        FirebaseDynamicLinks {
            client,
            auth: authenticator,
            _user_agent: "google-api-rust-client/2.0.3".to_string(),
            _base_url: "https://firebasedynamiclinks.googleapis.com/".to_string(),
            _root_url: "https://firebasedynamiclinks.googleapis.com/".to_string(),
        }
    }

    pub fn managed_short_links(&'a self) -> ManagedShortLinkMethods<'a> {
        ManagedShortLinkMethods { hub: &self }
    }
    pub fn methods(&'a self) -> MethodMethods<'a> {
        MethodMethods { hub: &self }
    }
    pub fn short_links(&'a self) -> ShortLinkMethods<'a> {
        ShortLinkMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/2.0.3`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://firebasedynamiclinks.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://firebasedynamiclinks.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Tracking parameters supported by Dynamic Link.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyticsInfo {
    /// Google Play Campaign Measurements.
    #[serde(rename="googlePlayAnalytics")]
    pub google_play_analytics: Option<GooglePlayAnalytics>,
    /// iTunes Connect App Analytics.
    #[serde(rename="itunesConnectAnalytics")]
    pub itunes_connect_analytics: Option<ITunesConnectAnalytics>,
}

impl client::Part for AnalyticsInfo {}


/// Android related attributes to the Dynamic Link.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidInfo {
    /// Link to open on Android if the app is not installed.
    #[serde(rename="androidFallbackLink")]
    pub android_fallback_link: Option<String>,
    /// If specified, this overrides the ‘link’ parameter on Android.
    #[serde(rename="androidLink")]
    pub android_link: Option<String>,
    /// Minimum version code for the Android app. If the installed app’s version code is lower, then the user is taken to the Play Store.
    #[serde(rename="androidMinPackageVersionCode")]
    pub android_min_package_version_code: Option<String>,
    /// Android package name of the app.
    #[serde(rename="androidPackageName")]
    pub android_package_name: Option<String>,
}

impl client::Part for AndroidInfo {}


/// Request to create a managed Short Dynamic Link.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create managed short links](ManagedShortLinkCreateCall) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateManagedShortLinkRequest {
    /// Information about the Dynamic Link to be shortened. [Learn more](https://firebase.google.com/docs/reference/dynamic-links/link-shortener).
    #[serde(rename="dynamicLinkInfo")]
    pub dynamic_link_info: Option<DynamicLinkInfo>,
    /// Full long Dynamic Link URL with desired query parameters specified. For example, "https://sample.app.goo.gl/?link=http://www.google.com&apn=com.sample", [Learn more](https://firebase.google.com/docs/reference/dynamic-links/link-shortener).
    #[serde(rename="longDynamicLink")]
    pub long_dynamic_link: Option<String>,
    /// Link name to associate with the link. It's used for marketer to identify manually-created links in the Firebase console (https://console.firebase.google.com/). Links must be named to be tracked.
    pub name: Option<String>,
    /// Google SDK version. Version takes the form "$major.$minor.$patch"
    #[serde(rename="sdkVersion")]
    pub sdk_version: Option<String>,
    /// Short Dynamic Link suffix. Optional.
    pub suffix: Option<Suffix>,
}

impl client::RequestValue for CreateManagedShortLinkRequest {}


/// Response to create a short Dynamic Link.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create managed short links](ManagedShortLinkCreateCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateManagedShortLinkResponse {
    /// Short Dynamic Link value. e.g. https://abcd.app.goo.gl/wxyz
    #[serde(rename="managedShortLink")]
    pub managed_short_link: Option<ManagedShortLink>,
    /// Preview link to show the link flow chart. (debug info.)
    #[serde(rename="previewLink")]
    pub preview_link: Option<String>,
    /// Information about potential warnings on link creation.
    pub warning: Option<Vec<DynamicLinkWarning>>,
}

impl client::ResponseResult for CreateManagedShortLinkResponse {}


/// Request to create a short Dynamic Link.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create short links](ShortLinkCreateCall) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateShortDynamicLinkRequest {
    /// Information about the Dynamic Link to be shortened. [Learn more](https://firebase.google.com/docs/reference/dynamic-links/link-shortener).
    #[serde(rename="dynamicLinkInfo")]
    pub dynamic_link_info: Option<DynamicLinkInfo>,
    /// Full long Dynamic Link URL with desired query parameters specified. For example, "https://sample.app.goo.gl/?link=http://www.google.com&apn=com.sample", [Learn more](https://firebase.google.com/docs/reference/dynamic-links/link-shortener).
    #[serde(rename="longDynamicLink")]
    pub long_dynamic_link: Option<String>,
    /// Google SDK version. Version takes the form "$major.$minor.$patch"
    #[serde(rename="sdkVersion")]
    pub sdk_version: Option<String>,
    /// Short Dynamic Link suffix. Optional.
    pub suffix: Option<Suffix>,
}

impl client::RequestValue for CreateShortDynamicLinkRequest {}


/// Response to create a short Dynamic Link.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create short links](ShortLinkCreateCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateShortDynamicLinkResponse {
    /// Preview link to show the link flow chart. (debug info.)
    #[serde(rename="previewLink")]
    pub preview_link: Option<String>,
    /// Short Dynamic Link value. e.g. https://abcd.app.goo.gl/wxyz
    #[serde(rename="shortLink")]
    pub short_link: Option<String>,
    /// Information about potential warnings on link creation.
    pub warning: Option<Vec<DynamicLinkWarning>>,
}

impl client::ResponseResult for CreateShortDynamicLinkResponse {}


/// Desktop related attributes to the Dynamic Link.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DesktopInfo {
    /// Link to open on desktop.
    #[serde(rename="desktopFallbackLink")]
    pub desktop_fallback_link: Option<String>,
}

impl client::Part for DesktopInfo {}


/// Signals associated with the device making the request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// Device model name.
    #[serde(rename="deviceModelName")]
    pub device_model_name: Option<String>,
    /// Device language code setting.
    #[serde(rename="languageCode")]
    pub language_code: Option<String>,
    /// Device language code setting obtained by executing JavaScript code in WebView.
    #[serde(rename="languageCodeFromWebview")]
    pub language_code_from_webview: Option<String>,
    /// Device language code raw setting. iOS does returns language code in different format than iOS WebView. For example WebView returns en_US, but iOS returns en-US. Field below will return raw value returned by iOS.
    #[serde(rename="languageCodeRaw")]
    pub language_code_raw: Option<String>,
    /// Device display resolution height.
    #[serde(rename="screenResolutionHeight")]
    pub screen_resolution_height: Option<String>,
    /// Device display resolution width.
    #[serde(rename="screenResolutionWidth")]
    pub screen_resolution_width: Option<String>,
    /// Device timezone setting.
    pub timezone: Option<String>,
}

impl client::Part for DeviceInfo {}


/// Dynamic Link event stat.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicLinkEventStat {
    /// The number of times this event occurred.
    pub count: Option<String>,
    /// Link event.
    pub event: Option<String>,
    /// Requested platform.
    pub platform: Option<String>,
}

impl client::Part for DynamicLinkEventStat {}


/// Information about a Dynamic Link.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicLinkInfo {
    /// Parameters used for tracking. See all tracking parameters in the [documentation](https://firebase.google.com/docs/dynamic-links/create-manually).
    #[serde(rename="analyticsInfo")]
    pub analytics_info: Option<AnalyticsInfo>,
    /// Android related information. See Android related parameters in the [documentation](https://firebase.google.com/docs/dynamic-links/create-manually).
    #[serde(rename="androidInfo")]
    pub android_info: Option<AndroidInfo>,
    /// Desktop related information. See desktop related parameters in the [documentation](https://firebase.google.com/docs/dynamic-links/create-manually).
    #[serde(rename="desktopInfo")]
    pub desktop_info: Option<DesktopInfo>,
    /// E.g. https://maps.app.goo.gl, https://maps.page.link, https://g.co/maps More examples can be found in description of getNormalizedUriPrefix in j/c/g/firebase/dynamiclinks/uri/DdlDomain.java Will fallback to dynamic_link_domain is this field is missing
    #[serde(rename="domainUriPrefix")]
    pub domain_uri_prefix: Option<String>,
    /// Dynamic Links domain that the project owns, e.g. abcd.app.goo.gl [Learn more](https://firebase.google.com/docs/dynamic-links/android/receive) on how to set up Dynamic Link domain associated with your Firebase project. Required if missing domain_uri_prefix.
    #[serde(rename="dynamicLinkDomain")]
    pub dynamic_link_domain: Option<String>,
    /// iOS related information. See iOS related parameters in the [documentation](https://firebase.google.com/docs/dynamic-links/create-manually).
    #[serde(rename="iosInfo")]
    pub ios_info: Option<IosInfo>,
    /// The link your app will open, You can specify any URL your app can handle. This link must be a well-formatted URL, be properly URL-encoded, and use the HTTP or HTTPS scheme. See 'link' parameters in the [documentation](https://firebase.google.com/docs/dynamic-links/create-manually). Required.
    pub link: Option<String>,
    /// Information of navigation behavior of a Firebase Dynamic Links.
    #[serde(rename="navigationInfo")]
    pub navigation_info: Option<NavigationInfo>,
    /// Parameters for social meta tag params. Used to set meta tag data for link previews on social sites.
    #[serde(rename="socialMetaTagInfo")]
    pub social_meta_tag_info: Option<SocialMetaTagInfo>,
}

impl client::Part for DynamicLinkInfo {}


/// Analytics stats of a Dynamic Link for a given timeframe.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get link stats](MethodGetLinkStatCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicLinkStats {
    /// Dynamic Link event stats.
    #[serde(rename="linkEventStats")]
    pub link_event_stats: Option<Vec<DynamicLinkEventStat>>,
}

impl client::ResponseResult for DynamicLinkStats {}


/// Dynamic Links warning messages.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicLinkWarning {
    /// The warning code.
    #[serde(rename="warningCode")]
    pub warning_code: Option<String>,
    /// The document describing the warning, and helps resolve.
    #[serde(rename="warningDocumentLink")]
    pub warning_document_link: Option<String>,
    /// The warning message to help developers improve their requests.
    #[serde(rename="warningMessage")]
    pub warning_message: Option<String>,
}

impl client::Part for DynamicLinkWarning {}


/// Request for iSDK to execute strong match flow for post-install attribution. This is meant for iOS requests only. Requests from other platforms will not be honored.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [install attribution](MethodInstallAttributionCall) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetIosPostInstallAttributionRequest {
    /// App installation epoch time (https://en.wikipedia.org/wiki/Unix_time). This is a client signal for a more accurate weak match.
    #[serde(rename="appInstallationTime")]
    pub app_installation_time: Option<String>,
    /// APP bundle ID.
    #[serde(rename="bundleId")]
    pub bundle_id: Option<String>,
    /// Device information.
    pub device: Option<DeviceInfo>,
    /// iOS version, ie: 9.3.5. Consider adding "build".
    #[serde(rename="iosVersion")]
    pub ios_version: Option<String>,
    /// App post install attribution retrieval information. Disambiguates mechanism (iSDK or developer invoked) to retrieve payload from clicked link.
    #[serde(rename="retrievalMethod")]
    pub retrieval_method: Option<String>,
    /// Google SDK version. Version takes the form "$major.$minor.$patch"
    #[serde(rename="sdkVersion")]
    pub sdk_version: Option<String>,
    /// Possible unique matched link that server need to check before performing fingerprint match. If passed link is short server need to expand the link. If link is long server need to vslidate the link.
    #[serde(rename="uniqueMatchLinkToCheck")]
    pub unique_match_link_to_check: Option<String>,
    /// Strong match page information. Disambiguates between default UI and custom page to present when strong match succeeds/fails to find cookie.
    #[serde(rename="visualStyle")]
    pub visual_style: Option<String>,
}

impl client::RequestValue for GetIosPostInstallAttributionRequest {}


/// Response for iSDK to execute strong match flow for post-install attribution.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [install attribution](MethodInstallAttributionCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetIosPostInstallAttributionResponse {
    /// The minimum version for app, specified by dev through ?imv= parameter. Return to iSDK to allow app to evaluate if current version meets this.
    #[serde(rename="appMinimumVersion")]
    pub app_minimum_version: Option<String>,
    /// The confidence of the returned attribution.
    #[serde(rename="attributionConfidence")]
    pub attribution_confidence: Option<String>,
    /// The deep-link attributed post-install via one of several techniques (fingerprint, copy unique).
    #[serde(rename="deepLink")]
    pub deep_link: Option<String>,
    /// User-agent specific custom-scheme URIs for iSDK to open. This will be set according to the user-agent tha the click was originally made in. There is no Safari-equivalent custom-scheme open URLs. ie: googlechrome://www.example.com ie: firefox://open-url?url=http://www.example.com ie: opera-http://example.com
    #[serde(rename="externalBrowserDestinationLink")]
    pub external_browser_destination_link: Option<String>,
    /// The link to navigate to update the app if min version is not met. This is either (in order): 1) fallback link (from ?ifl= parameter, if specified by developer) or 2) AppStore URL (from ?isi= parameter, if specified), or 3) the payload link (from required link= parameter).
    #[serde(rename="fallbackLink")]
    pub fallback_link: Option<String>,
    /// Invitation ID attributed post-install via one of several techniques (fingerprint, copy unique).
    #[serde(rename="invitationId")]
    pub invitation_id: Option<String>,
    /// Instruction for iSDK to attemmpt to perform strong match. For instance, if browser does not support/allow cookie or outside of support browsers, this will be false.
    #[serde(rename="isStrongMatchExecutable")]
    pub is_strong_match_executable: Option<bool>,
    /// Describes why match failed, ie: "discarded due to low confidence". This message will be publicly visible.
    #[serde(rename="matchMessage")]
    pub match_message: Option<String>,
    /// Which IP version the request was made from.
    #[serde(rename="requestIpVersion")]
    pub request_ip_version: Option<String>,
    /// Entire FDL (short or long) attributed post-install via one of several techniques (fingerprint, copy unique).
    #[serde(rename="requestedLink")]
    pub requested_link: Option<String>,
    /// The entire FDL, expanded from a short link. It is the same as the requested_link, if it is long. Parameters from this should not be used directly (ie: server can default utm_[campaign|medium|source] to a value when requested_link lack them, server determine the best fallback_link when requested_link specifies >1 fallback links).
    #[serde(rename="resolvedLink")]
    pub resolved_link: Option<String>,
    /// Scion campaign value to be propagated by iSDK to Scion at post-install.
    #[serde(rename="utmCampaign")]
    pub utm_campaign: Option<String>,
    /// Scion content value to be propagated by iSDK to Scion at app-reopen.
    #[serde(rename="utmContent")]
    pub utm_content: Option<String>,
    /// Scion medium value to be propagated by iSDK to Scion at post-install.
    #[serde(rename="utmMedium")]
    pub utm_medium: Option<String>,
    /// Scion source value to be propagated by iSDK to Scion at post-install.
    #[serde(rename="utmSource")]
    pub utm_source: Option<String>,
    /// Scion term value to be propagated by iSDK to Scion at app-reopen.
    #[serde(rename="utmTerm")]
    pub utm_term: Option<String>,
}

impl client::ResponseResult for GetIosPostInstallAttributionResponse {}


/// Request for iSDK to get reopen attribution for app universal link open deeplinking. This endpoint is meant for only iOS requests.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reopen attribution](MethodReopenAttributionCall) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetIosReopenAttributionRequest {
    /// APP bundle ID.
    #[serde(rename="bundleId")]
    pub bundle_id: Option<String>,
    /// FDL link to be verified from an app universal link open. The FDL link can be one of: 1) short FDL. e.g. .page.link/, or 2) long FDL. e.g. .page.link/?{query params}, or 3) Invite FDL. e.g. .page.link/i/
    #[serde(rename="requestedLink")]
    pub requested_link: Option<String>,
    /// Google SDK version. Version takes the form "$major.$minor.$patch"
    #[serde(rename="sdkVersion")]
    pub sdk_version: Option<String>,
}

impl client::RequestValue for GetIosReopenAttributionRequest {}


/// Response for iSDK to get reopen attribution for app universal link open deeplinking. This endpoint is meant for only iOS requests.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reopen attribution](MethodReopenAttributionCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetIosReopenAttributionResponse {
    /// The deep-link attributed the app universal link open. For both regular FDL links and invite FDL links.
    #[serde(rename="deepLink")]
    pub deep_link: Option<String>,
    /// Optional invitation ID, for only invite typed requested FDL links.
    #[serde(rename="invitationId")]
    pub invitation_id: Option<String>,
    /// FDL input value of the "&imv=" parameter, minimum app version to be returned to Google Firebase SDK running on iOS-9.
    #[serde(rename="iosMinAppVersion")]
    pub ios_min_app_version: Option<String>,
    /// The entire FDL, expanded from a short link. It is the same as the requested_link, if it is long.
    #[serde(rename="resolvedLink")]
    pub resolved_link: Option<String>,
    /// Scion campaign value to be propagated by iSDK to Scion at app-reopen.
    #[serde(rename="utmCampaign")]
    pub utm_campaign: Option<String>,
    /// Scion content value to be propagated by iSDK to Scion at app-reopen.
    #[serde(rename="utmContent")]
    pub utm_content: Option<String>,
    /// Scion medium value to be propagated by iSDK to Scion at app-reopen.
    #[serde(rename="utmMedium")]
    pub utm_medium: Option<String>,
    /// Scion source value to be propagated by iSDK to Scion at app-reopen.
    #[serde(rename="utmSource")]
    pub utm_source: Option<String>,
    /// Scion term value to be propagated by iSDK to Scion at app-reopen.
    #[serde(rename="utmTerm")]
    pub utm_term: Option<String>,
}

impl client::ResponseResult for GetIosReopenAttributionResponse {}


/// Parameters for Google Play Campaign Measurements. [Learn more](https://developers.google.com/analytics/devguides/collection/android/v4/campaigns#campaign-params)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePlayAnalytics {
    /// [AdWords autotagging parameter](https://support.google.com/analytics/answer/1033981?hl=en); used to measure Google AdWords ads. This value is generated dynamically and should never be modified.
    pub gclid: Option<String>,
    /// Campaign name; used for keyword analysis to identify a specific product promotion or strategic campaign.
    #[serde(rename="utmCampaign")]
    pub utm_campaign: Option<String>,
    /// Campaign content; used for A/B testing and content-targeted ads to differentiate ads or links that point to the same URL.
    #[serde(rename="utmContent")]
    pub utm_content: Option<String>,
    /// Campaign medium; used to identify a medium such as email or cost-per-click.
    #[serde(rename="utmMedium")]
    pub utm_medium: Option<String>,
    /// Campaign source; used to identify a search engine, newsletter, or other source.
    #[serde(rename="utmSource")]
    pub utm_source: Option<String>,
    /// Campaign term; used with paid search to supply the keywords for ads.
    #[serde(rename="utmTerm")]
    pub utm_term: Option<String>,
}

impl client::Part for GooglePlayAnalytics {}


/// Parameters for iTunes Connect App Analytics.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ITunesConnectAnalytics {
    /// Affiliate token used to create affiliate-coded links.
    pub at: Option<String>,
    /// Campaign text that developers can optionally add to any link in order to track sales from a specific marketing campaign.
    pub ct: Option<String>,
    /// iTune media types, including music, podcasts, audiobooks and so on.
    pub mt: Option<String>,
    /// Provider token that enables analytics for Dynamic Links from within iTunes Connect.
    pub pt: Option<String>,
}

impl client::Part for ITunesConnectAnalytics {}


/// iOS related attributes to the Dynamic Link..
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IosInfo {
    /// iOS App Store ID.
    #[serde(rename="iosAppStoreId")]
    pub ios_app_store_id: Option<String>,
    /// iOS bundle ID of the app.
    #[serde(rename="iosBundleId")]
    pub ios_bundle_id: Option<String>,
    /// Custom (destination) scheme to use for iOS. By default, we’ll use the bundle ID as the custom scheme. Developer can override this behavior using this param.
    #[serde(rename="iosCustomScheme")]
    pub ios_custom_scheme: Option<String>,
    /// Link to open on iOS if the app is not installed.
    #[serde(rename="iosFallbackLink")]
    pub ios_fallback_link: Option<String>,
    /// iPad bundle ID of the app.
    #[serde(rename="iosIpadBundleId")]
    pub ios_ipad_bundle_id: Option<String>,
    /// If specified, this overrides the ios_fallback_link value on iPads.
    #[serde(rename="iosIpadFallbackLink")]
    pub ios_ipad_fallback_link: Option<String>,
    /// iOS minimum version.
    #[serde(rename="iosMinimumVersion")]
    pub ios_minimum_version: Option<String>,
}

impl client::Part for IosInfo {}


/// Managed Short Link.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create managed short links](ManagedShortLinkCreateCall) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedShortLink {
    /// Creation timestamp of the short link.
    #[serde(rename="creationTime")]
    pub creation_time: Option<String>,
    /// Attributes that have been flagged about this short url.
    #[serde(rename="flaggedAttribute")]
    pub flagged_attribute: Option<Vec<String>>,
    /// Full Dyamic Link info
    pub info: Option<DynamicLinkInfo>,
    /// Short durable link url, for example, "https://sample.app.goo.gl/xyz123". Required.
    pub link: Option<String>,
    /// Link name defined by the creator. Required.
    #[serde(rename="linkName")]
    pub link_name: Option<String>,
    /// Visibility status of link.
    pub visibility: Option<String>,
}

impl client::Resource for ManagedShortLink {}


/// Information of navigation behavior.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NavigationInfo {
    /// If this option is on, FDL click will be forced to redirect rather than show an interstitial page.
    #[serde(rename="enableForcedRedirect")]
    pub enable_forced_redirect: Option<bool>,
}

impl client::Part for NavigationInfo {}


/// Parameters for social meta tag params. Used to set meta tag data for link previews on social sites.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SocialMetaTagInfo {
    /// A short description of the link. Optional.
    #[serde(rename="socialDescription")]
    pub social_description: Option<String>,
    /// An image url string. Optional.
    #[serde(rename="socialImageLink")]
    pub social_image_link: Option<String>,
    /// Title to be displayed. Optional.
    #[serde(rename="socialTitle")]
    pub social_title: Option<String>,
}

impl client::Part for SocialMetaTagInfo {}


/// Short Dynamic Link suffix.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Suffix {
    /// Only applies to Option.CUSTOM.
    #[serde(rename="customSuffix")]
    pub custom_suffix: Option<String>,
    /// Suffix option.
    pub option: Option<String>,
}

impl client::Part for Suffix {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *managedShortLink* resources.
/// It is not used directly, but through the `FirebaseDynamicLinks` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_firebasedynamiclinks1 as firebasedynamiclinks1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use firebasedynamiclinks1::FirebaseDynamicLinks;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = FirebaseDynamicLinks::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`
/// // to build up your call.
/// let rb = hub.managed_short_links();
/// # }
/// ```
pub struct ManagedShortLinkMethods<'a>
    where  {

    hub: &'a FirebaseDynamicLinks<>,
}

impl<'a> client::MethodsBuilder for ManagedShortLinkMethods<'a> {}

impl<'a> ManagedShortLinkMethods<'a> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a managed short Dynamic Link given either a valid long Dynamic Link or details such as Dynamic Link domain, Android and iOS app information. The created short Dynamic Link will not expire. This differs from CreateShortDynamicLink in the following ways: - The request will also contain a name for the link (non unique name for the front end). - The response must be authenticated with an auth token (generated with the admin service account). - The link will appear in the FDL list of links in the console front end. The Dynamic Link domain in the request must be owned by requester's Firebase project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: CreateManagedShortLinkRequest) -> ManagedShortLinkCreateCall<'a> {
        ManagedShortLinkCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *shortLink* resources.
/// It is not used directly, but through the `FirebaseDynamicLinks` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_firebasedynamiclinks1 as firebasedynamiclinks1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use firebasedynamiclinks1::FirebaseDynamicLinks;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = FirebaseDynamicLinks::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`
/// // to build up your call.
/// let rb = hub.short_links();
/// # }
/// ```
pub struct ShortLinkMethods<'a>
    where  {

    hub: &'a FirebaseDynamicLinks<>,
}

impl<'a> client::MethodsBuilder for ShortLinkMethods<'a> {}

impl<'a> ShortLinkMethods<'a> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a short Dynamic Link given either a valid long Dynamic Link or details such as Dynamic Link domain, Android and iOS app information. The created short Dynamic Link will not expire. Repeated calls with the same long Dynamic Link or Dynamic Link information will produce the same short Dynamic Link. The Dynamic Link domain in the request must be owned by requester's Firebase project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: CreateShortDynamicLinkRequest) -> ShortLinkCreateCall<'a> {
        ShortLinkCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all free methods, which are not associated with a particular resource.
/// It is not used directly, but through the `FirebaseDynamicLinks` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_firebasedynamiclinks1 as firebasedynamiclinks1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use firebasedynamiclinks1::FirebaseDynamicLinks;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = FirebaseDynamicLinks::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_link_stats(...)`, `install_attribution(...)` and `reopen_attribution(...)`
/// // to build up your call.
/// let rb = hub.methods();
/// # }
/// ```
pub struct MethodMethods<'a>
    where  {

    hub: &'a FirebaseDynamicLinks<>,
}

impl<'a> client::MethodsBuilder for MethodMethods<'a> {}

impl<'a> MethodMethods<'a> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches analytics stats of a short Dynamic Link for a given duration. Metrics include number of clicks, redirects, installs, app first opens, and app reopens.
    /// 
    /// # Arguments
    ///
    /// * `dynamicLink` - Dynamic Link URL. e.g. https://abcd.app.goo.gl/wxyz
    pub fn get_link_stats(&self, dynamic_link: &str) -> MethodGetLinkStatCall<'a> {
        MethodGetLinkStatCall {
            hub: self.hub,
            _dynamic_link: dynamic_link.to_string(),
            _sdk_version: Default::default(),
            _duration_days: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get iOS strong/weak-match info for post-install attribution.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn install_attribution(&self, request: GetIosPostInstallAttributionRequest) -> MethodInstallAttributionCall<'a> {
        MethodInstallAttributionCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get iOS reopen attribution for app universal link open deeplinking.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn reopen_attribution(&self, request: GetIosReopenAttributionRequest) -> MethodReopenAttributionCall<'a> {
        MethodReopenAttributionCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Creates a managed short Dynamic Link given either a valid long Dynamic Link or details such as Dynamic Link domain, Android and iOS app information. The created short Dynamic Link will not expire. This differs from CreateShortDynamicLink in the following ways: - The request will also contain a name for the link (non unique name for the front end). - The response must be authenticated with an auth token (generated with the admin service account). - The link will appear in the FDL list of links in the console front end. The Dynamic Link domain in the request must be owned by requester's Firebase project.
///
/// A builder for the *create* method supported by a *managedShortLink* resource.
/// It is not used directly, but through a `ManagedShortLinkMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_firebasedynamiclinks1 as firebasedynamiclinks1;
/// use firebasedynamiclinks1::api::CreateManagedShortLinkRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use firebasedynamiclinks1::FirebaseDynamicLinks;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = FirebaseDynamicLinks::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CreateManagedShortLinkRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.managed_short_links().create(req)
///              .doit().await;
/// # }
/// ```
pub struct ManagedShortLinkCreateCall<'a>
    where  {

    hub: &'a FirebaseDynamicLinks<>,
    _request: CreateManagedShortLinkRequest,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ManagedShortLinkCreateCall<'a> {}

impl<'a> ManagedShortLinkCreateCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, CreateManagedShortLinkResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "firebasedynamiclinks.managedShortLinks.create",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/managedShortLinks:create";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Firebase.as_ref().to_string(), ());
        }


        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: CreateManagedShortLinkRequest) -> ManagedShortLinkCreateCall<'a> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ManagedShortLinkCreateCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ManagedShortLinkCreateCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Firebase`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ManagedShortLinkCreateCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a short Dynamic Link given either a valid long Dynamic Link or details such as Dynamic Link domain, Android and iOS app information. The created short Dynamic Link will not expire. Repeated calls with the same long Dynamic Link or Dynamic Link information will produce the same short Dynamic Link. The Dynamic Link domain in the request must be owned by requester's Firebase project.
///
/// A builder for the *create* method supported by a *shortLink* resource.
/// It is not used directly, but through a `ShortLinkMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_firebasedynamiclinks1 as firebasedynamiclinks1;
/// use firebasedynamiclinks1::api::CreateShortDynamicLinkRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use firebasedynamiclinks1::FirebaseDynamicLinks;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = FirebaseDynamicLinks::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CreateShortDynamicLinkRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.short_links().create(req)
///              .doit().await;
/// # }
/// ```
pub struct ShortLinkCreateCall<'a>
    where  {

    hub: &'a FirebaseDynamicLinks<>,
    _request: CreateShortDynamicLinkRequest,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ShortLinkCreateCall<'a> {}

impl<'a> ShortLinkCreateCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, CreateShortDynamicLinkResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "firebasedynamiclinks.shortLinks.create",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/shortLinks";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Firebase.as_ref().to_string(), ());
        }


        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: CreateShortDynamicLinkRequest) -> ShortLinkCreateCall<'a> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ShortLinkCreateCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ShortLinkCreateCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Firebase`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ShortLinkCreateCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Fetches analytics stats of a short Dynamic Link for a given duration. Metrics include number of clicks, redirects, installs, app first opens, and app reopens.
///
/// A builder for the *getLinkStats* method.
/// It is not used directly, but through a `MethodMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_firebasedynamiclinks1 as firebasedynamiclinks1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use firebasedynamiclinks1::FirebaseDynamicLinks;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = FirebaseDynamicLinks::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().get_link_stats("dynamicLink")
///              .sdk_version("magna")
///              .duration_days("no")
///              .doit().await;
/// # }
/// ```
pub struct MethodGetLinkStatCall<'a>
    where  {

    hub: &'a FirebaseDynamicLinks<>,
    _dynamic_link: String,
    _sdk_version: Option<String>,
    _duration_days: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for MethodGetLinkStatCall<'a> {}

impl<'a> MethodGetLinkStatCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, DynamicLinkStats)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "firebasedynamiclinks.getLinkStats",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("dynamicLink", self._dynamic_link.to_string()));
        if let Some(value) = self._sdk_version {
            params.push(("sdkVersion", value.to_string()));
        }
        if let Some(value) = self._duration_days {
            params.push(("durationDays", value.to_string()));
        }
        for &field in ["alt", "dynamicLink", "sdkVersion", "durationDays"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{dynamicLink}/linkStats";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Firebase.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{dynamicLink}", "dynamicLink")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["dynamicLink"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Dynamic Link URL. e.g. https://abcd.app.goo.gl/wxyz
    ///
    /// Sets the *dynamic link* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dynamic_link(mut self, new_value: &str) -> MethodGetLinkStatCall<'a> {
        self._dynamic_link = new_value.to_string();
        self
    }
    /// Google SDK version. Version takes the form "$major.$minor.$patch"
    ///
    /// Sets the *sdk version* query property to the given value.
    pub fn sdk_version(mut self, new_value: &str) -> MethodGetLinkStatCall<'a> {
        self._sdk_version = Some(new_value.to_string());
        self
    }
    /// The span of time requested in days.
    ///
    /// Sets the *duration days* query property to the given value.
    pub fn duration_days(mut self, new_value: &str) -> MethodGetLinkStatCall<'a> {
        self._duration_days = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MethodGetLinkStatCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> MethodGetLinkStatCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Firebase`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> MethodGetLinkStatCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Get iOS strong/weak-match info for post-install attribution.
///
/// A builder for the *installAttribution* method.
/// It is not used directly, but through a `MethodMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_firebasedynamiclinks1 as firebasedynamiclinks1;
/// use firebasedynamiclinks1::api::GetIosPostInstallAttributionRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use firebasedynamiclinks1::FirebaseDynamicLinks;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = FirebaseDynamicLinks::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GetIosPostInstallAttributionRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().install_attribution(req)
///              .doit().await;
/// # }
/// ```
pub struct MethodInstallAttributionCall<'a>
    where  {

    hub: &'a FirebaseDynamicLinks<>,
    _request: GetIosPostInstallAttributionRequest,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for MethodInstallAttributionCall<'a> {}

impl<'a> MethodInstallAttributionCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GetIosPostInstallAttributionResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "firebasedynamiclinks.installAttribution",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/installAttribution";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Firebase.as_ref().to_string(), ());
        }


        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: GetIosPostInstallAttributionRequest) -> MethodInstallAttributionCall<'a> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MethodInstallAttributionCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> MethodInstallAttributionCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Firebase`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> MethodInstallAttributionCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Get iOS reopen attribution for app universal link open deeplinking.
///
/// A builder for the *reopenAttribution* method.
/// It is not used directly, but through a `MethodMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_firebasedynamiclinks1 as firebasedynamiclinks1;
/// use firebasedynamiclinks1::api::GetIosReopenAttributionRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use firebasedynamiclinks1::FirebaseDynamicLinks;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = FirebaseDynamicLinks::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GetIosReopenAttributionRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().reopen_attribution(req)
///              .doit().await;
/// # }
/// ```
pub struct MethodReopenAttributionCall<'a>
    where  {

    hub: &'a FirebaseDynamicLinks<>,
    _request: GetIosReopenAttributionRequest,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for MethodReopenAttributionCall<'a> {}

impl<'a> MethodReopenAttributionCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GetIosReopenAttributionResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "firebasedynamiclinks.reopenAttribution",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/reopenAttribution";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Firebase.as_ref().to_string(), ());
        }


        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: GetIosReopenAttributionRequest) -> MethodReopenAttributionCall<'a> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MethodReopenAttributionCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> MethodReopenAttributionCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Firebase`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> MethodReopenAttributionCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


