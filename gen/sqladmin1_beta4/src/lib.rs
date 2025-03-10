// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *SQL Admin* crate version *4.0.1+20220226*, where *20220226* is the exact revision of the *sqladmin:v1beta4* schema built by the [mako](http://www.makotemplates.org/) code generator *v4.0.1*.
//! 
//! Everything else about the *SQL Admin* *v1_beta4* API can be found at the
//! [official documentation site](https://developers.google.com/cloud-sql/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/sqladmin1_beta4).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](SQLAdmin) ... 
//! 
//! * [backup runs](api::BackupRun)
//!  * [*delete*](api::BackupRunDeleteCall), [*get*](api::BackupRunGetCall), [*insert*](api::BackupRunInsertCall) and [*list*](api::BackupRunListCall)
//! * connect
//!  * [*generate ephemeral*](api::ConnectGenerateEphemeralCall) and [*get*](api::ConnectGetCall)
//! * [databases](api::Database)
//!  * [*delete*](api::DatabaseDeleteCall), [*get*](api::DatabaseGetCall), [*insert*](api::DatabaseInsertCall), [*list*](api::DatabaseListCall), [*patch*](api::DatabasePatchCall) and [*update*](api::DatabaseUpdateCall)
//! * [flags](api::Flag)
//!  * [*list*](api::FlagListCall)
//! * instances
//!  * [*add server ca*](api::InstanceAddServerCaCall), [*clone*](api::InstanceCloneCall), [*delete*](api::InstanceDeleteCall), [*demote master*](api::InstanceDemoteMasterCall), [*export*](api::InstanceExportCall), [*failover*](api::InstanceFailoverCall), [*get*](api::InstanceGetCall), [*import*](api::InstanceImportCall), [*insert*](api::InstanceInsertCall), [*list*](api::InstanceListCall), [*list server cas*](api::InstanceListServerCaCall), [*patch*](api::InstancePatchCall), [*promote replica*](api::InstancePromoteReplicaCall), [*reset ssl config*](api::InstanceResetSslConfigCall), [*restart*](api::InstanceRestartCall), [*restore backup*](api::InstanceRestoreBackupCall), [*rotate server ca*](api::InstanceRotateServerCaCall), [*start replica*](api::InstanceStartReplicaCall), [*stop replica*](api::InstanceStopReplicaCall), [*truncate log*](api::InstanceTruncateLogCall) and [*update*](api::InstanceUpdateCall)
//! * [operations](api::Operation)
//!  * [*get*](api::OperationGetCall) and [*list*](api::OperationListCall)
//! * projects
//!  * [*instances reschedule maintenance*](api::ProjectInstanceRescheduleMaintenanceCall), [*instances start external sync*](api::ProjectInstanceStartExternalSyncCall) and [*instances verify external sync settings*](api::ProjectInstanceVerifyExternalSyncSettingCall)
//! * [ssl certs](api::SslCert)
//!  * [*create ephemeral*](api::SslCertCreateEphemeralCall), [*delete*](api::SslCertDeleteCall), [*get*](api::SslCertGetCall), [*insert*](api::SslCertInsertCall) and [*list*](api::SslCertListCall)
//! * [tiers](api::Tier)
//!  * [*list*](api::TierListCall)
//! * [users](api::User)
//!  * [*delete*](api::UserDeleteCall), [*insert*](api::UserInsertCall), [*list*](api::UserListCall) and [*update*](api::UserUpdateCall)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](SQLAdmin)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](client::MethodsBuilder) which in turn
//!       allow access to individual [*Call Builders*](client::CallBuilder)
//! * **[Resources](client::Resource)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](client::Part)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](client::CallBuilder)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit().await
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.backup_runs().delete(...).doit().await
//! let r = hub.backup_runs().insert(...).doit().await
//! let r = hub.databases().delete(...).doit().await
//! let r = hub.databases().insert(...).doit().await
//! let r = hub.databases().patch(...).doit().await
//! let r = hub.databases().update(...).doit().await
//! let r = hub.instances().add_server_ca(...).doit().await
//! let r = hub.instances().clone(...).doit().await
//! let r = hub.instances().delete(...).doit().await
//! let r = hub.instances().demote_master(...).doit().await
//! let r = hub.instances().export(...).doit().await
//! let r = hub.instances().failover(...).doit().await
//! let r = hub.instances().import(...).doit().await
//! let r = hub.instances().insert(...).doit().await
//! let r = hub.instances().patch(...).doit().await
//! let r = hub.instances().promote_replica(...).doit().await
//! let r = hub.instances().reset_ssl_config(...).doit().await
//! let r = hub.instances().restart(...).doit().await
//! let r = hub.instances().restore_backup(...).doit().await
//! let r = hub.instances().rotate_server_ca(...).doit().await
//! let r = hub.instances().start_replica(...).doit().await
//! let r = hub.instances().stop_replica(...).doit().await
//! let r = hub.instances().truncate_log(...).doit().await
//! let r = hub.instances().update(...).doit().await
//! let r = hub.operations().get(...).doit().await
//! let r = hub.operations().list(...).doit().await
//! let r = hub.projects().instances_reschedule_maintenance(...).doit().await
//! let r = hub.projects().instances_start_external_sync(...).doit().await
//! let r = hub.ssl_certs().delete(...).doit().await
//! let r = hub.users().delete(...).doit().await
//! let r = hub.users().insert(...).doit().await
//! let r = hub.users().update(...).doit().await
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-sqladmin1_beta4 = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_sqladmin1_beta4 as sqladmin1_beta4;
//! use sqladmin1_beta4::api::User;
//! use sqladmin1_beta4::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use sqladmin1_beta4::{SQLAdmin, oauth2, hyper, hyper_rustls};
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: oauth2::ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = oauth2::InstalledFlowAuthenticator::builder(
//!         secret,
//!         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
//!     ).build().await.unwrap();
//! let mut hub = SQLAdmin::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = User::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.users().update(req, "project", "instance")
//!              .name("sanctus")
//!              .host("sed")
//!              .doit().await;
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::Io(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](client::Result) enumeration as return value of
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](client::Result), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](client::ResponseResult), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](client::Delegate) to the 
//! [Method Builder](client::CallBuilder) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](client::Delegate) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [encodable](client::RequestValue) and 
//! [decodable](client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](client::Part) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](client::RequestValue) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

#[macro_use]
extern crate serde_derive;

// Re-export the hyper and hyper_rustls crate, they are required to build the hub
pub extern crate hyper;
pub extern crate hyper_rustls;
extern crate serde;
extern crate serde_json;
// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
pub extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

pub mod api;
pub mod client;

// Re-export the hub type and some basic client structs
pub use api::SQLAdmin;
pub use client::{Result, Error, Delegate};
