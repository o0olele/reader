/// Credentials and request defaults exposed to a source's JavaScript rules.
/// Network access is deliberately only available through these injected
/// functions; the QuickJS sandbox has no filesystem, process, or environment
/// access.
#[derive(Clone, Debug, Default)]
pub struct JsHttpContext {
    pub base_url: String,
    pub headers: Option<String>,
    pub access_token: Option<String>,
    pub session_cookie: Option<String>,
    pub session_expired: bool,
    pub sign_script: Option<String>,
}

struct JsHttpSession {
    client: reqwest::blocking::Client,
    context: JsHttpContext,
    response: Arc<Mutex<Option<JsHttpResponse>>>,
}

#[derive(Clone, Debug, Default)]
struct JsHttpResponse {
    status: u16,
    headers: HashMap<String, String>,
}

#[derive(Clone, Debug, Default)]
pub struct JsContext {
    pub result: String,
    pub url: Option<String>,
    pub key: Option<String>,
    pub base_url: Option<String>,
    pub variables: HashMap<String, String>,
    pub http: Option<JsHttpContext>,
    /// Legado aliases exposed by AnalyzeRule.evalJS.
    pub title: Option<String>,
    pub src: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum JsValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Json(JsonValue),
}

#[async_trait]
pub trait JsRuntime: Send + Sync {
    async fn execute(&self, script: &str, context: JsContext) -> Result<JsValue, AppError>;
}

#[derive(Clone, Debug)]
pub struct QuickJsRuntime {
    timeout: Duration,
    memory_limit: usize,
}

impl Default for QuickJsRuntime {
    fn default() -> Self {
        Self::new(Duration::from_secs(5), 16 * 1024 * 1024)
    }
}

impl QuickJsRuntime {
    pub fn new(timeout: Duration, memory_limit: usize) -> Self {
        Self {
            timeout,
            memory_limit,
        }
    }

    pub fn execute_blocking(&self, script: &str, context: JsContext) -> Result<JsValue, AppError> {
        self.execute_blocking_with_context(script, context)
            .map(|(value, _)| value)
    }

    pub fn execute_blocking_with_context(
        &self,
        script: &str,
        context: JsContext,
    ) -> Result<(JsValue, HashMap<String, String>), AppError> {
        // `reqwest::blocking::Client` owns a private Tokio runtime. Dropping
        // it on a Tokio worker panics (`Cannot drop a runtime in a context
        // where blocking is not allowed`). URL parsing and the rule pipeline
        // are synchronous APIs, so isolate the whole QuickJS session on a
        // plain thread whenever this entry point is reached from async code.
        if tokio::runtime::Handle::try_current().is_ok() {
            let runtime = self.clone();
            let script = script.to_owned();
            return std::thread::spawn(move || runtime.execute_blocking_inner(&script, context))
                .join()
                .map_err(|_| AppError::Source("JavaScript worker panicked".into()))?;
        }
        self.execute_blocking_inner(script, context)
    }

    fn execute_blocking_inner(
        &self,
        script: &str,
        context: JsContext,
    ) -> Result<(JsValue, HashMap<String, String>), AppError> {
        let runtime =
            Runtime::new().map_err(|error| AppError::Source(format!("JS runtime: {error}")))?;
        runtime.set_memory_limit(self.memory_limit);
        let deadline = Instant::now() + self.timeout;
        runtime.set_interrupt_handler(Some(Box::new(move || Instant::now() >= deadline)));
        let quick_context = Context::full(&runtime)
            .map_err(|error| AppError::Source(format!("JS context: {error}")))?;
        let values = Arc::new(Mutex::new(context.variables));
        let output = quick_context
            .with(|ctx| {
                install_globals(
                    ctx,
                    &values,
                    context.result,
                    context.url,
                    context.key,
                    context.base_url,
                    context.http,
                    context.title,
                    context.src,
                )
            })
            .and_then(|()| quick_context.with(|ctx| evaluate_script(ctx, script)))?;
        let variables = values
            .lock()
            .map(|values| values.clone())
            .unwrap_or_default();
        Ok((output, variables))
    }
}

#[async_trait]
impl JsRuntime for QuickJsRuntime {
    async fn execute(&self, script: &str, context: JsContext) -> Result<JsValue, AppError> {
        if script.trim().is_empty() {
            return Err(AppError::InvalidArgument("JavaScript 规则不能为空".into()));
        }
        let runtime = self.clone();
        let script = script.to_owned();
        tokio::time::timeout(
            self.timeout,
            tokio::task::spawn_blocking(move || runtime.execute_blocking(&script, context)),
        )
        .await
        .map_err(|_| {
            AppError::Source(format!(
                "JavaScript 执行超时（{}ms）",
                self.timeout.as_millis()
            ))
        })?
        .map_err(|error| AppError::Source(format!("JavaScript worker failed: {error}")))?
    }
}

