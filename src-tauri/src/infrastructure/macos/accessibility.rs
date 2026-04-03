use core_foundation::base::TCFType;
use core_foundation::boolean::CFBoolean;
use core_foundation::dictionary::CFDictionary;
use core_foundation::string::CFString;

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn AXIsProcessTrusted() -> bool;
    fn AXIsProcessTrustedWithOptions(options: *const core_foundation::dictionary::__CFDictionary) -> bool;
}

// kAXTrustedCheckOptionPrompt key
const AX_TRUSTED_CHECK_OPTION_PROMPT: &str = "AXTrustedCheckOptionPrompt";

/// Check if the app has accessibility permission
pub fn check_accessibility_permission() -> bool {
    unsafe { AXIsProcessTrusted() }
}

/// Request accessibility permission with system prompt
/// Returns true if already has permission, false if permission dialog was shown
pub fn request_accessibility_permission() -> bool {
    let key = CFString::new(AX_TRUSTED_CHECK_OPTION_PROMPT);
    let value = CFBoolean::true_value();

    // Create dictionary with key-value pair as tuple
    let pairs: [(CFString, CFBoolean); 1] = [(key, value)];
    let options = CFDictionary::from_CFType_pairs(&pairs);

    unsafe {
        AXIsProcessTrustedWithOptions(options.as_concrete_TypeRef())
    }
}
