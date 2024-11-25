use std::sync::Once;
use v8;

// Error handling
#[derive(Debug)]
pub enum Error {
    JsInitError(String),
    JsExecError(String),
    JsValueError(String),
}

#[derive(Default)]
pub struct Opts {
    display_mode: Option<bool>,
}

impl Opts {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn display_mode(mut self, value: bool) -> Self {
        self.display_mode = Some(value);
        self
    }
}

static INIT: Once = Once::new();

fn initialize_v8() {
    INIT.call_once(|| {
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();
    });
}

// Helper trait to create v8::String
trait IntoV8String {
    fn to_v8_string<'s>(&self, scope: &mut v8::HandleScope<'s>) -> v8::Local<'s, v8::String>;
}

impl<T: AsRef<str>> IntoV8String for T {
    fn to_v8_string<'s>(&self, scope: &mut v8::HandleScope<'s>) -> v8::Local<'s, v8::String> {
        // We use unwrap here because this conversion should never fail
        // If it does fail (e.g., due to OOM), it's a catastrophic error anyway
        v8::String::new(scope, self.as_ref()).unwrap()
    }
}

pub fn inject_katex<'a>(
    context: v8::Local<'a, v8::Context>,
    scope: &mut v8::ContextScope<'a, v8::HandleScope<'_>>,
) -> Result<v8::Local<'a, v8::Object>, Error> {
    let katex_src = include_str!("../vendor/katex.min.js").to_v8_string(scope);

    v8::Script::compile(scope, katex_src, None)
        .and_then(|script| script.run(scope))
        .ok_or_else(|| Error::JsInitError("Failed to compile KaTeX script".to_string()))?;

    let global = context.global(scope);
    let katex_string = "katex".to_v8_string(scope);

    let katex = global
        .get(scope, katex_string.into())
        .ok_or_else(|| Error::JsValueError("Failed to get KaTeX object".to_string()))?;
    let katex_obj = v8::Local::<v8::Object>::try_from(katex);

    match katex_obj {
        Ok(obj) => Ok(obj),
        Err(e) => Err(Error::JsValueError(format!(
            "Failed to acquire KaTeX object: {}",
            e
        ))),
    }
}

pub fn render(input: &str, opts: &Opts) -> Result<String, Error> {
    initialize_v8();

    let isolate = &mut v8::Isolate::new(Default::default());
    let handle_scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(handle_scope, Default::default());
    let scope = &mut v8::ContextScope::new(handle_scope, context);

    let katex = inject_katex(context, scope)?;

    let input = v8::String::new(scope, input).unwrap();
    let opts_obj = v8::Object::new(scope);
    if let Some(display_mode) = opts.display_mode {
        let key = "displayMode".to_v8_string(scope);
        let value = v8::Boolean::new(scope, display_mode);
        opts_obj.set(scope, key.into(), value.into()).unwrap();
    }

    let render_to_string = "renderToString".to_v8_string(scope);

    let render_func =
        v8::Local::<v8::Function>::try_from(katex.get(scope, render_to_string.into()).unwrap())
            .unwrap();

    let args = &[input.into(), opts_obj.into()];
    let result = render_func.call(scope, katex.into(), args).unwrap();

    let result = result.to_string(scope).unwrap();
    Ok(result.to_rust_string_lossy(scope))
}

fn main() -> Result<(), Error> {
    let opts = Opts::new().display_mode(true);
    let html = render("E = mc^2", &opts)?;
    println!("{}", html);
    Ok(())
}
