use super::describe;
use crate::app::AppContext;
#[derive(Clone, Default)]
pub struct Routes {
    pub prefix: Option<String>,
    pub handlers: Vec<Handler>,
    // pub version: Option<String>,
}

#[derive(Clone, Default)]
pub struct Handler {
    pub uri: String,
    pub method: axum::routing::MethodRouter<AppContext>,
    pub actions: Vec<axum::http::Method>,
}

impl Routes {
    /// Creates a new [`Routes`] instance with default settings.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a prefix for the routes. this prefix will be a prefix for all the
    /// routes.
    ///
    /// # Example
    ///
    /// In the following example the we are adding `status`  as a prefix to the
    /// _ping endpoint HOST/status/_ping.
    ///
    /// ```rust
    /// use loco_rs::prelude::*;
    /// use serde::Serialize;;
    ///
    /// #[derive(Serialize)]
    /// struct Health {
    ///    pub ok: bool,
    /// }
    ///
    /// async fn ping() -> Result<Response> {
    ///     format::json(Health { ok: true })
    /// }
    /// Routes::at("status").add("/_ping", get(ping));
    ///    
    /// ````
    #[must_use]
    pub fn at(prefix: &str) -> Self {
        Self {
            prefix: Some(prefix.to_string()),
            ..Self::default()
        }
    }

    /// Adding new router
    ///
    /// # Example
    ///
    /// This example preset how to add a get endpoint int the Router.
    ///
    /// ```rust
    /// use loco_rs::prelude::*;
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct Health {
    ///    pub ok: bool,
    /// }
    ///
    /// async fn ping() -> Result<Response> {
    ///     format::json(Health { ok: true })
    /// }
    /// Routes::new().add("/_ping", get(ping));
    /// ````
    #[must_use]
    pub fn add(mut self, uri: &str, method: axum::routing::MethodRouter<AppContext>) -> Self {
        describe::method_action(&method);
        self.handlers.push(Handler {
            uri: uri.to_owned(),
            actions: describe::method_action(&method),
            method,
        });
        self
    }

    /// Merge two routes together. This will add the prefix of the route to the
    /// routes.
    ///
    /// # Example
    ///
    /// _ping endpoint HOST/status/internal/_ping.
    /// _ping endpoint HOST/status/external/_ping.
    ///
    /// ```rust
    /// let routesA = Routes::at("internal").add("/_ping", get(ping));
    /// let routesB = Routes::at("external").add("/_ping", get(ping));
    /// Routes::new().prefix("status").merge(routesA).merge(routesB);
    /// ````
    #[must_use]
    pub fn merge(mut self, route: Routes) -> Self {
        let prefix = route.prefix.clone().unwrap_or("".to_string());
        for handler in route.handlers {
            self.handlers.push(Handler {
                uri: format!("/{}/{}", prefix, handler.uri),
                actions: handler.actions,
                method: handler.method,
            });
        }
        self
    }


    /// Set a prefix for the routes. this prefix will be a prefix for all the
    /// routes.
    ///
    /// # Example
    ///
    /// In the following example the we are adding `status`  as a prefix to the
    /// _ping endpoint HOST/status/_ping.
    ///
    /// ```rust
    /// use loco_rs::prelude::*;
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct Health {
    ///    pub ok: bool,
    /// }
    ///
    /// async fn ping() -> Result<Response> {
    ///     format::json(Health { ok: true })
    /// }
    /// Routes::new().prefix("status").add("/_ping", get(ping));
    /// ````
    #[must_use]
    pub fn prefix(mut self, uri: &str) -> Self {
        self.prefix = Some(uri.to_owned());
        self
    }
}
