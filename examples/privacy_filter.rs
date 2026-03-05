/// GDPR-Compliant Pipeline (Example)
/// 
/// Demonstrates "Compile-Time GDPR enforcement" using the Rust type system.
/// This prevents developers from accidentally passing PII to an LLM module.

use regex::Regex;
use std::sync::LazyLock;

// 1. Core Traits
// -----------------------------------------------------------------------------

/// Data types that are safe to be used in AI/LLM prompts.
/// This trait is ONLY implemented for sanitized types.
pub trait LlmSafeContext {
    fn get_inquiry(&self) -> &str;
}

// 2. Data Structures
// -----------------------------------------------------------------------------

/// Raw intake from the web or API. Contains PII.
/// This type DOES NOT implement LlmSafeContext.
pub struct RawLead {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub inquiry: String,
    pub location: String,
}

/// Sanitized data structure. Structurally lacks PII fields.
/// This type IMPLEMENTS LlmSafeContext.
pub struct SanitizedContext {
    pub inquiry: String,
    pub location_zone: String, // Derived from Kataster, not the raw address
}

impl LlmSafeContext for SanitizedContext {
    fn get_inquiry(&self) -> &str {
        &self.inquiry
    }
}

// 3. The Privacy Shield
// -----------------------------------------------------------------------------

static PII_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b[\w.+-]+@[\w-]+\.[a-z]{2,}\b|\+?[\d\s\-()]{7,15}").unwrap()
});

/// The only gate to the LLM. 
/// It locally redacts PII and returns a type that the compiler accepts for AI calls.
pub fn strip_pii(raw: RawLead) -> SanitizedContext {
    // Local regex stripping of inquiry text
    let clean_inquiry = PII_PATTERN.replace_all(&raw.inquiry, "[REDACTED]");
    
    SanitizedContext {
        inquiry: clean_inquiry.into_owned(),
        location_zone: "Zone B (Sanitized)".to_string(), // In reality: derived via PostGIS
    }
}

// 4. The AI Bridge (Compiler-Guarded)
// -----------------------------------------------------------------------------

/// This function represents our LLM or Python/PyO3 call site.
/// It forces the use of a type that implements `LlmSafeContext`.
pub fn process_with_ai<T: LlmSafeContext>(ctx: T) -> String {
    format!("AI is processing inquiry: {}", ctx.get_inquiry())
}

fn main() {
    let lead = RawLead {
        name: "John Doe".to_string(),
        phone: "+34 600 000 000".to_string(),
        email: "john@example.com".to_string(),
        inquiry: "I want to buy a villa near the volcano.".to_string(),
        location: "Calle 123, La Palma".to_string(),
    };

    // UNCOMMENTING THE LINE BELOW WILL CAUSE A COMPILE ERROR:
    // process_with_ai(lead); 
    // ^ Error: the trait `LlmSafeContext` is not implemented for `RawLead`

    // SUCCESSFUL PATH:
    let safe_data = strip_pii(lead);
    let response = process_with_ai(safe_data);
    println!("{}", response);
}
