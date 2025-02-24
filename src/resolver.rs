use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::error::ResolveError;
use trust_dns_resolver::proto::rr::RecordType;


pub async fn resolve_all_record_types(domain: &str) -> Result<bool, ResolveError> {
    let resolver = TokioAsyncResolver::tokio_from_system_conf()?;
    let mut is_success = false;

    for record_type in [
        RecordType::A,
        RecordType::AAAA,
        RecordType::CNAME,
        RecordType::MX,
        RecordType::NS,
        RecordType::PTR,
        RecordType::SOA,
        RecordType::SRV,
        RecordType::TXT,
    ] {
        match resolver.lookup(domain, record_type).await{
            Ok(_) => {
                is_success = true;
                break;
            }
            Err(_) => break,
        }
    }
    Ok(is_success)


}