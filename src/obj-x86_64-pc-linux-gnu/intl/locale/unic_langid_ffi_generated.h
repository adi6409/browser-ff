/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/* Generated with cbindgen:0.16.0 */

/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen. See RunCbindgen.py */
#ifndef mozilla_intl_locale_MozLocaleBindings_h
#error "Don't include this file directly, instead include MozLocaleBindings.h"
#endif


#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace mozilla {
namespace intl {
namespace ffi {

/// `LanguageIdentifier` is a core struct representing a Unicode Language Identifier.
///
/// # Examples
///
/// ```
/// use unic_langid_impl::LanguageIdentifier;
///
/// let li: LanguageIdentifier = "en-US".parse()
///     .expect("Failed to parse.");
///
/// assert_eq!(li.language, "en");
/// assert_eq!(li.script, None);
/// assert_eq!(li.region.as_ref().map(Into::into), Some("US"));
/// assert_eq!(li.variants().len(), 0);
/// ```
///
/// # Parsing
///
/// Unicode recognizes three levels of standard conformance for any language identifier:
///
///  * *well-formed* - syntactically correct
///  * *valid* - well-formed and only uses registered language subtags, extensions, keywords, types...
///  * *canonical* - valid and no deprecated codes or structure.
///
/// At the moment parsing normalizes a well-formed language identifier converting
/// `_` separators to `-` and adjusting casing to conform to the Unicode standard.
///
/// Any bogus subtags will cause the parsing to fail with an error.
/// No subtag validation is performed.
///
/// # Examples:
///
/// ```
/// use unic_langid_impl::LanguageIdentifier;
///
/// let li: LanguageIdentifier = "eN_latn_Us-Valencia".parse()
///     .expect("Failed to parse.");
///
/// assert_eq!(li.language, "en");
/// assert_eq!(li.script.as_ref().map(Into::into), Some("Latn"));
/// assert_eq!(li.region.as_ref().map(Into::into), Some("US"));
/// assert_eq!(li.variants().map(|v| v.as_str()).collect::<Vec<_>>(), &["valencia"]);
/// ```
struct LanguageIdentifier;

extern "C" {

bool unic_langid_canonicalize(nsACString *name);

LanguageIdentifier *unic_langid_new(const nsACString *name, bool *ret_val);

void unic_langid_destroy(LanguageIdentifier *langid);

void unic_langid_as_string(LanguageIdentifier *langid, nsACString *ret_val);

void unic_langid_get_language(const LanguageIdentifier *langid, nsDependentCSubstring *out);

bool unic_langid_set_language(LanguageIdentifier *langid, const nsACString *string);

void unic_langid_clear_language(LanguageIdentifier *langid);

void unic_langid_get_script(const LanguageIdentifier *langid, nsDependentCSubstring *out);

bool unic_langid_set_script(LanguageIdentifier *langid, const nsACString *string);

void unic_langid_clear_script(LanguageIdentifier *langid);

void unic_langid_get_region(const LanguageIdentifier *langid, nsDependentCSubstring *out);

bool unic_langid_set_region(LanguageIdentifier *langid, const nsACString *string);

void unic_langid_clear_region(LanguageIdentifier *langid);

void unic_langid_get_variants(const LanguageIdentifier *langid, nsTArray<nsCString> *variants);

bool unic_langid_set_variants(LanguageIdentifier *langid, const nsTArray<nsCString> *variants);

void unic_langid_clear_variants(LanguageIdentifier *langid);

bool unic_langid_matches(const LanguageIdentifier *langid,
                         const LanguageIdentifier *other,
                         bool self_as_range,
                         bool other_as_range);

bool unic_langid_maximize(LanguageIdentifier *langid);

bool unic_langid_is_rtl(const nsACString *name);

} // extern "C"

} // namespace ffi
} // namespace intl
} // namespace mozilla
