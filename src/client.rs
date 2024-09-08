use crate::error::Result;
use crate::toucan_extern;
use worker::js_sys::{self, Reflect};

/// Sentry configuration

/// Only the DSN is required to build a SentryConfig instance, the other fields are optional.
#[derive(Debug)]
pub struct SentryConfig {
    dsn: String,
    environment: Option<String>,
    cf_access_client_id: Option<String>,
    cf_access_client_secret: Option<String>,
}

impl SentryConfig {
    /// Build the Sentry configuration from a DSN.
    ///
    /// This does not require an environment, cf_access_client_id, or cf_access
    /// The SentryConfig struct can be built manually if these are needed.
    pub fn from_dsn(dsn: String) -> Result<Self> {
        Ok(Self {
            dsn,
            environment: None,
            cf_access_client_id: None,
            cf_access_client_secret: None,
        })
    }
}

/// Sentry client
#[derive(Debug)]
pub struct SentryClient {
    inner: toucan_extern::Toucan,
}

impl SentryClient {
    /// Create a Sentry client. This will initialize the Sentry client with the provided configuration.
    /// The worker request and context are optional.
    pub fn new(
        config: SentryConfig,
        request: Option<&worker::Request>,
        context: Option<&worker::Context>,
    ) -> Result<SentryClient> {
        let transport_options_headers = js_sys::Object::new();

        if let Some(ref cf_access_client_id) = config.cf_access_client_id {
            Reflect::set(
                &transport_options_headers,
                &"CF-Access-Client-ID".into(),
                &cf_access_client_id.into(),
            )?;
        }

        if let Some(ref cf_access_client_secret) = config.cf_access_client_secret {
            Reflect::set(
                &transport_options_headers,
                &"CF-Access-Client-Secret".into(),
                &cf_access_client_secret.into(),
            )?;
        }

        let transport_options = js_sys::Object::new();
        Reflect::set(
            &transport_options,
            &"headers".into(),
            &transport_options_headers,
        )?;

        let args = js_sys::Object::new();
        Reflect::set(&args, &"dsn".into(), &config.dsn.into())?;
        Reflect::set(&args, &"transportOptions".into(), &transport_options)?;

        let allowed_headers = js_sys::Array::new();

        [
            "user-agent",
            "accept-encoding",
            "accept-language",
            "cf-ray",
            "content-length",
            "content-type",
            "x-real-ip",
            "host",
        ]
        .iter()
        .for_each(|header| {
            allowed_headers.push(&(*header).into());
        });

        Reflect::set(&args, &"allowedHeaders".into(), &allowed_headers)?;

        if let Some(request) = request {
            Reflect::set(&args, &"request".into(), request.inner())?;
        }

        if let Some(ref context) = context {
            Reflect::set(&args, &"context".into(), context.as_ref())?;
        }

        Reflect::set(&args, &"debug".into(), &false.into())?;

        if let Some(ref environment) = config.environment {
            Reflect::set(&args, &"environment".into(), &environment.into())?;
        }

        worker::console_debug!("Toucan args: {:?}", args);

        let toucan = toucan_extern::Toucan::new(args);

        Ok(SentryClient { inner: toucan })
    }

    /// See Sentry's setTag documentation
    pub fn set_tag(&self, key: &str, value: &str) {
        self.inner.set_tag(key, value);
    }

    /// See Sentry's setContext documentation
    pub fn set_context(&self, name: &str, value: js_sys::Object) {
        self.inner.set_context(name, value);
    }

    /// See Sentry's captureMessage documentation
    pub fn capture_message(&self, message: &str) {
        self.inner.capture_message(message);
    }

    /// See Sentry's captureException documentation
    pub fn capture_exception<T: std::error::Error + ?Sized>(&self, error: &T) {
        let js_error = js_sys::Error::new(&error.to_string());
        self.inner.capture_exception(js_error);
    }

    /// Same as `capture_exception` but with a `js_sys::Error`.
    pub fn capture_js_error(&self, js_error: js_sys::Error) {
        self.inner.capture_exception(js_error);
    }

    // See Sentry's setPropagationContext documentation
    pub fn set_propagation_context(&self, pctx: js_sys::Object) {
        self.inner.set_propagation_context(pctx);
    }

    // See Sentry's setRequestBody documentation
    pub fn set_request_body(&self, body: &str) {
        self.inner.set_request_body(body);
    }

    // See Sentry's setEnabled documentation
    pub fn set_enabled(&self, enabled: bool) {
        self.inner.set_enabled(enabled);
    }

    // See Sentry's addBreadcrumb documentation
    pub fn add_breadcrumb(&self, breadcrumb: js_sys::Object, max_breadcrumbs: u32) {
        self.inner.add_breadcrumb(breadcrumb, max_breadcrumbs);
    }
}
