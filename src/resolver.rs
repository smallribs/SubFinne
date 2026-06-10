use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::error::ResolveError;
use trust_dns_resolver::proto::rr::RecordType;
use std::time::Duration;
use std::sync::LazyLock;

static RESOLVER: LazyLock<TokioAsyncResolver> = LazyLock::new(|| {
    let mut opts = ResolverOpts::default();
    opts.timeout = Duration::from_secs(3);
    opts.attempts = 1;
    TokioAsyncResolver::tokio(ResolverConfig::default(), opts)
});

pub async fn resolve_all_record_types(domain: &str) -> Result<bool, ResolveError> {
    for record_type in [
        RecordType::A,
        RecordType::AAAA,
        RecordType::CNAME,
    ] {
        match RESOLVER.lookup(domain, record_type).await {
            Ok(_) => {
                return Ok(true);
            }
            Err(_) => continue,
        }
    }
    Ok(false)
}