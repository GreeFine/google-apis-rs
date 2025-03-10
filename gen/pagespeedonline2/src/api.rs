use std::collections::HashMap;
use std::cell::RefCell;
use std::default::Default;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;
use std::thread::sleep;

use http::Uri;
use hyper::client::connect;
use tokio::io::{AsyncRead, AsyncWrite};
use tower_service;
use crate::client;

// ##############
// UTILITIES ###
// ############




// ########
// HUB ###
// ######

/// Central instance to access all Pagespeedonline related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_pagespeedonline2 as pagespeedonline2;
/// use pagespeedonline2::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use pagespeedonline2::{Pagespeedonline, oauth2, hyper, hyper_rustls};
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Pagespeedonline::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.pagespeedapi().runpagespeed("url")
///              .strategy("gubergren")
///              .screenshot(false)
///              .add_rule("dolor")
///              .locale("ea")
///              .filter_third_party_resources(true)
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
#[derive(Clone)]
pub struct Pagespeedonline<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: oauth2::authenticator::Authenticator<S>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, S> client::Hub for Pagespeedonline<S> {}

impl<'a, S> Pagespeedonline<S> {

    pub fn new(client: hyper::Client<S, hyper::body::Body>, authenticator: oauth2::authenticator::Authenticator<S>) -> Pagespeedonline<S> {
        Pagespeedonline {
            client,
            auth: authenticator,
            _user_agent: "google-api-rust-client/4.0.1".to_string(),
            _base_url: "https://www.googleapis.com/pagespeedonline/v2/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn pagespeedapi(&'a self) -> PagespeedapiMethods<'a, S> {
        PagespeedapiMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/4.0.1`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/pagespeedonline/v2/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiFormatStringV2 {
    /// List of arguments for the format string.
    pub args: Option<Vec<PagespeedApiFormatStringV2Args>>,
    /// A localized format string with {{FOO}} placeholders, where 'FOO' is the key of the argument whose value should be substituted. For HYPERLINK arguments, the format string will instead contain {{BEGIN_FOO}} and {{END_FOO}} for the argument with key 'FOO'.
    pub format: Option<String>,
}

impl client::Part for PagespeedApiFormatStringV2 {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiImageV2 {
    /// Image data base64 encoded.
    pub data: Option<String>,
    /// Height of screenshot in pixels.
    pub height: Option<i32>,
    /// Unique string key, if any, identifying this image.
    pub key: Option<String>,
    /// Mime type of image data (e.g. "image/jpeg").
    pub mime_type: Option<String>,
    /// The region of the page that is captured by this image, with dimensions measured in CSS pixels.
    pub page_rect: Option<PagespeedApiImageV2PageRect>,
    /// Width of screenshot in pixels.
    pub width: Option<i32>,
}

impl client::Part for PagespeedApiImageV2 {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [runpagespeed pagespeedapi](PagespeedapiRunpagespeedCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Result {
    /// The captcha verify result
    #[serde(rename="captchaResult")]
    pub captcha_result: Option<String>,
    /// Localized PageSpeed results. Contains a ruleResults entry for each PageSpeed rule instantiated and run by the server.
    #[serde(rename="formattedResults")]
    pub formatted_results: Option<ResultFormattedResults>,
    /// Canonicalized and final URL for the document, after following page redirects (if any).
    pub id: Option<String>,
    /// List of rules that were specified in the request, but which the server did not know how to instantiate.
    #[serde(rename="invalidRules")]
    pub invalid_rules: Option<Vec<String>>,
    /// Kind of result.
    pub kind: Option<String>,
    /// Summary statistics for the page, such as number of JavaScript bytes, number of HTML bytes, etc.
    #[serde(rename="pageStats")]
    pub page_stats: Option<ResultPageStats>,
    /// Response code for the document. 200 indicates a normal page load. 4xx/5xx indicates an error.
    #[serde(rename="responseCode")]
    pub response_code: Option<i32>,
    /// A map with one entry for each rule group in these results.
    #[serde(rename="ruleGroups")]
    pub rule_groups: Option<HashMap<String, ResultRuleGroups>>,
    /// Base64-encoded screenshot of the page that was analyzed.
    pub screenshot: Option<PagespeedApiImageV2>,
    /// Title of the page, as displayed in the browser's title bar.
    pub title: Option<String>,
    /// The version of PageSpeed used to generate these results.
    pub version: Option<ResultVersion>,
}

impl client::ResponseResult for Result {}


/// List of arguments for the format string.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiFormatStringV2Args {
    /// The placeholder key for this arg, as a string.
    pub key: Option<String>,
    /// The screen rectangles being referred to, with dimensions measured in CSS pixels. This is only ever used for SNAPSHOT_RECT arguments. If this is absent for a SNAPSHOT_RECT argument, it means that that argument refers to the entire snapshot.
    pub rects: Option<Vec<PagespeedApiFormatStringV2ArgsRects>>,
    /// Secondary screen rectangles being referred to, with dimensions measured in CSS pixels. This is only ever used for SNAPSHOT_RECT arguments.
    pub secondary_rects: Option<Vec<PagespeedApiFormatStringV2ArgsSecondaryRects>>,
    /// Type of argument. One of URL, STRING_LITERAL, INT_LITERAL, BYTES, DURATION, VERBATIM_STRING, PERCENTAGE, HYPERLINK, or SNAPSHOT_RECT.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// Argument value, as a localized string.
    pub value: Option<String>,
}

impl client::NestedType for PagespeedApiFormatStringV2Args {}
impl client::Part for PagespeedApiFormatStringV2Args {}


/// The screen rectangles being referred to, with dimensions measured in CSS pixels. This is only ever used for SNAPSHOT_RECT arguments. If this is absent for a SNAPSHOT_RECT argument, it means that that argument refers to the entire snapshot.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiFormatStringV2ArgsRects {
    /// The height of the rect.
    pub height: Option<i32>,
    /// The left coordinate of the rect, in page coordinates.
    pub left: Option<i32>,
    /// The top coordinate of the rect, in page coordinates.
    pub top: Option<i32>,
    /// The width of the rect.
    pub width: Option<i32>,
}

impl client::NestedType for PagespeedApiFormatStringV2ArgsRects {}
impl client::Part for PagespeedApiFormatStringV2ArgsRects {}


/// Secondary screen rectangles being referred to, with dimensions measured in CSS pixels. This is only ever used for SNAPSHOT_RECT arguments.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiFormatStringV2ArgsSecondaryRects {
    /// The height of the rect.
    pub height: Option<i32>,
    /// The left coordinate of the rect, in page coordinates.
    pub left: Option<i32>,
    /// The top coordinate of the rect, in page coordinates.
    pub top: Option<i32>,
    /// The width of the rect.
    pub width: Option<i32>,
}

impl client::NestedType for PagespeedApiFormatStringV2ArgsSecondaryRects {}
impl client::Part for PagespeedApiFormatStringV2ArgsSecondaryRects {}


/// The region of the page that is captured by this image, with dimensions measured in CSS pixels.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiImageV2PageRect {
    /// The height of the rect.
    pub height: Option<i32>,
    /// The left coordinate of the rect, in page coordinates.
    pub left: Option<i32>,
    /// The top coordinate of the rect, in page coordinates.
    pub top: Option<i32>,
    /// The width of the rect.
    pub width: Option<i32>,
}

impl client::NestedType for PagespeedApiImageV2PageRect {}
impl client::Part for PagespeedApiImageV2PageRect {}


/// Localized PageSpeed results. Contains a ruleResults entry for each PageSpeed rule instantiated and run by the server.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResultFormattedResults {
    /// The locale of the formattedResults, e.g. "en_US".
    pub locale: Option<String>,
    /// Dictionary of formatted rule results, with one entry for each PageSpeed rule instantiated and run by the server.
    #[serde(rename="ruleResults")]
    pub rule_results: Option<HashMap<String, ResultFormattedResultsRuleResults>>,
}

impl client::NestedType for ResultFormattedResults {}
impl client::Part for ResultFormattedResults {}


/// The enum-like identifier for this rule. For instance "EnableKeepAlive" or "AvoidCssImport". Not localized.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResultFormattedResultsRuleResults {
    /// List of rule groups that this rule belongs to. Each entry in the list is one of "SPEED" or "USABILITY".
    pub groups: Option<Vec<String>>,
    /// Localized name of the rule, intended for presentation to a user.
    #[serde(rename="localizedRuleName")]
    pub localized_rule_name: Option<String>,
    /// The impact (unbounded floating point value) that implementing the suggestions for this rule would have on making the page faster. Impact is comparable between rules to determine which rule's suggestions would have a higher or lower impact on making a page faster. For instance, if enabling compression would save 1MB, while optimizing images would save 500kB, the enable compression rule would have 2x the impact of the image optimization rule, all other things being equal.
    #[serde(rename="ruleImpact")]
    pub rule_impact: Option<f64>,
    /// A brief summary description for the rule, indicating at a high level what should be done to follow the rule and what benefit can be gained by doing so.
    pub summary: Option<PagespeedApiFormatStringV2>,
    /// List of blocks of URLs. Each block may contain a heading and a list of URLs. Each URL may optionally include additional details.
    #[serde(rename="urlBlocks")]
    pub url_blocks: Option<Vec<ResultFormattedResultsRuleResultsUrlBlocks>>,
}

impl client::NestedType for ResultFormattedResultsRuleResults {}
impl client::Part for ResultFormattedResultsRuleResults {}


/// List of blocks of URLs. Each block may contain a heading and a list of URLs. Each URL may optionally include additional details.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResultFormattedResultsRuleResultsUrlBlocks {
    /// Heading to be displayed with the list of URLs.
    pub header: Option<PagespeedApiFormatStringV2>,
    /// List of entries that provide information about URLs in the url block. Optional.
    pub urls: Option<Vec<ResultFormattedResultsRuleResultsUrlBlocksUrls>>,
}

impl client::NestedType for ResultFormattedResultsRuleResultsUrlBlocks {}
impl client::Part for ResultFormattedResultsRuleResultsUrlBlocks {}


/// List of entries that provide information about URLs in the url block. Optional.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResultFormattedResultsRuleResultsUrlBlocksUrls {
    /// List of entries that provide additional details about a single URL. Optional.
    pub details: Option<Vec<PagespeedApiFormatStringV2>>,
    /// A format string that gives information about the URL, and a list of arguments for that format string.
    pub result: Option<PagespeedApiFormatStringV2>,
}

impl client::NestedType for ResultFormattedResultsRuleResultsUrlBlocksUrls {}
impl client::Part for ResultFormattedResultsRuleResultsUrlBlocksUrls {}


/// Summary statistics for the page, such as number of JavaScript bytes, number of HTML bytes, etc.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResultPageStats {
    /// Number of uncompressed response bytes for CSS resources on the page.
    #[serde(rename="cssResponseBytes")]
    pub css_response_bytes: Option<String>,
    /// Number of response bytes for flash resources on the page.
    #[serde(rename="flashResponseBytes")]
    pub flash_response_bytes: Option<String>,
    /// Number of uncompressed response bytes for the main HTML document and all iframes on the page.
    #[serde(rename="htmlResponseBytes")]
    pub html_response_bytes: Option<String>,
    /// Number of response bytes for image resources on the page.
    #[serde(rename="imageResponseBytes")]
    pub image_response_bytes: Option<String>,
    /// Number of uncompressed response bytes for JS resources on the page.
    #[serde(rename="javascriptResponseBytes")]
    pub javascript_response_bytes: Option<String>,
    /// Number of CSS resources referenced by the page.
    #[serde(rename="numberCssResources")]
    pub number_css_resources: Option<i32>,
    /// Number of unique hosts referenced by the page.
    #[serde(rename="numberHosts")]
    pub number_hosts: Option<i32>,
    /// Number of JavaScript resources referenced by the page.
    #[serde(rename="numberJsResources")]
    pub number_js_resources: Option<i32>,
    /// Number of HTTP resources loaded by the page.
    #[serde(rename="numberResources")]
    pub number_resources: Option<i32>,
    /// Number of static (i.e. cacheable) resources on the page.
    #[serde(rename="numberStaticResources")]
    pub number_static_resources: Option<i32>,
    /// Number of response bytes for other resources on the page.
    #[serde(rename="otherResponseBytes")]
    pub other_response_bytes: Option<String>,
    /// Number of uncompressed response bytes for text resources not covered by other statistics (i.e non-HTML, non-script, non-CSS resources) on the page.
    #[serde(rename="textResponseBytes")]
    pub text_response_bytes: Option<String>,
    /// Total size of all request bytes sent by the page.
    #[serde(rename="totalRequestBytes")]
    pub total_request_bytes: Option<String>,
}

impl client::NestedType for ResultPageStats {}
impl client::Part for ResultPageStats {}


/// The name of this rule group: one of "SPEED" or "USABILITY".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResultRuleGroups {
    /// The score (0-100) for this rule group, which indicates how much better a page could be in that category (e.g. how much faster, or how much more usable). A high score indicates little room for improvement, while a lower score indicates more room for improvement.
    pub score: Option<i32>,
}

impl client::NestedType for ResultRuleGroups {}
impl client::Part for ResultRuleGroups {}


/// The version of PageSpeed used to generate these results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResultVersion {
    /// The major version number of PageSpeed used to generate these results.
    pub major: Option<i32>,
    /// The minor version number of PageSpeed used to generate these results.
    pub minor: Option<i32>,
}

impl client::NestedType for ResultVersion {}
impl client::Part for ResultVersion {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *pagespeedapi* resources.
/// It is not used directly, but through the `Pagespeedonline` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_pagespeedonline2 as pagespeedonline2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use pagespeedonline2::{Pagespeedonline, oauth2, hyper, hyper_rustls};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Pagespeedonline::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `runpagespeed(...)`
/// // to build up your call.
/// let rb = hub.pagespeedapi();
/// # }
/// ```
pub struct PagespeedapiMethods<'a, S>
    where S: 'a {

    hub: &'a Pagespeedonline<S>,
}

impl<'a, S> client::MethodsBuilder for PagespeedapiMethods<'a, S> {}

impl<'a, S> PagespeedapiMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Runs PageSpeed analysis on the page at the specified URL, and returns PageSpeed scores, a list of suggestions to make that page faster, and other information.
    /// 
    /// # Arguments
    ///
    /// * `url` - The URL to fetch and analyze
    pub fn runpagespeed(&self, url: &str) -> PagespeedapiRunpagespeedCall<'a, S> {
        PagespeedapiRunpagespeedCall {
            hub: self.hub,
            _url: url.to_string(),
            _strategy: Default::default(),
            _screenshot: Default::default(),
            _rule: Default::default(),
            _locale: Default::default(),
            _filter_third_party_resources: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Runs PageSpeed analysis on the page at the specified URL, and returns PageSpeed scores, a list of suggestions to make that page faster, and other information.
///
/// A builder for the *runpagespeed* method supported by a *pagespeedapi* resource.
/// It is not used directly, but through a `PagespeedapiMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_pagespeedonline2 as pagespeedonline2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use pagespeedonline2::{Pagespeedonline, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Pagespeedonline::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.pagespeedapi().runpagespeed("url")
///              .strategy("amet")
///              .screenshot(true)
///              .add_rule("sed")
///              .locale("ut")
///              .filter_third_party_resources(true)
///              .doit().await;
/// # }
/// ```
pub struct PagespeedapiRunpagespeedCall<'a, S>
    where S: 'a {

    hub: &'a Pagespeedonline<S>,
    _url: String,
    _strategy: Option<String>,
    _screenshot: Option<bool>,
    _rule: Vec<String>,
    _locale: Option<String>,
    _filter_third_party_resources: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for PagespeedapiRunpagespeedCall<'a, S> {}

impl<'a, S> PagespeedapiRunpagespeedCall<'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Result)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "pagespeedonline.pagespeedapi.runpagespeed",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(8 + self._additional_params.len());
        params.push(("url", self._url.to_string()));
        if let Some(value) = self._strategy {
            params.push(("strategy", value.to_string()));
        }
        if let Some(value) = self._screenshot {
            params.push(("screenshot", value.to_string()));
        }
        if self._rule.len() > 0 {
            for f in self._rule.iter() {
                params.push(("rule", f.to_string()));
            }
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        if let Some(value) = self._filter_third_party_resources {
            params.push(("filter_third_party_resources", value.to_string()));
        }
        for &field in ["alt", "url", "strategy", "screenshot", "rule", "locale", "filter_third_party_resources"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "runPagespeed";
        
        let key = dlg.api_key();
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }


        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone());


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
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
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


    /// The URL to fetch and analyze
    ///
    /// Sets the *url* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn url(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, S> {
        self._url = new_value.to_string();
        self
    }
    /// The analysis strategy to use
    ///
    /// Sets the *strategy* query property to the given value.
    pub fn strategy(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, S> {
        self._strategy = Some(new_value.to_string());
        self
    }
    /// Indicates if binary data containing a screenshot should be included
    ///
    /// Sets the *screenshot* query property to the given value.
    pub fn screenshot(mut self, new_value: bool) -> PagespeedapiRunpagespeedCall<'a, S> {
        self._screenshot = Some(new_value);
        self
    }
    /// A PageSpeed rule to run; if none are given, all rules are run
    ///
    /// Append the given value to the *rule* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_rule(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, S> {
        self._rule.push(new_value.to_string());
        self
    }
    /// The locale used to localize formatted results
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, S> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// Indicates if third party resources should be filtered out before PageSpeed analysis.
    ///
    /// Sets the *filter_third_party_resources* query property to the given value.
    pub fn filter_third_party_resources(mut self, new_value: bool) -> PagespeedapiRunpagespeedCall<'a, S> {
        self._filter_third_party_resources = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PagespeedapiRunpagespeedCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> PagespeedapiRunpagespeedCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


