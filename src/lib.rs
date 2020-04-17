use log::trace;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use primes::is_prime;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_http_context(|context_id, _| -> Box<dyn HttpContext> {
        Box::new(PrimeAuthorizer { context_id })
    });
}

struct PrimeAuthorizer {
    context_id: u32,
}

impl Context for PrimeAuthorizer {}

impl HttpContext for PrimeAuthorizer {
    fn on_http_request_headers(&mut self, _: usize) -> Action {
        for (name, value) in &self.get_http_request_headers() {
            trace!("In WASM : #{} -> {}: {}", self.context_id, name, value);
        }

        match self.get_http_request_header("token") {
            Some(token) if token.parse::<u64>().is_ok() && is_prime(token.parse().unwrap()) => {
                self.resume_http_request();
                Action::Continue
            }
            _ => {
                self.send_http_response(
                    403,
                    vec![("Powered-By", "proxy-wasm")],
                    Some(b"Access forbidden.\n"),
                );
                Action::Pause
            }
        }
    }
}
