/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef mozilla_net_rustHelper_h
#define mozilla_net_rustHelper_h

/* Generated with cbindgen:0.16.0 */

/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen. */

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>
#include "nsError.h"
#include "nsString.h"

namespace mozilla {
namespace net {

using ParsingCallback = bool(*)(const nsTArray<nsCString>*);

extern "C" {

/// Allocates an nsACString that contains a ISO 639 language list
/// notated with HTTP "q" values for output with an HTTP Accept-Language
/// header. Previous q values will be stripped because the order of
/// the langs implies the q value. The q values are calculated by dividing
/// 1.0 amongst the number of languages present.
///
/// Ex: passing: "en, ja"
///     returns: "en,ja;q=0.5"
///
///     passing: "en, ja, fr_CA"
///     returns: "en,ja;q=0.7,fr_CA;q=0.3"
nsresult rust_prepare_accept_languages(const nsACString *i_accept_languages,
                                       nsACString *o_accept_languages);

bool rust_net_is_valid_ipv4_addr(const nsACString *addr);

bool rust_net_is_valid_ipv6_addr(const nsACString *addr);

bool rust_net_is_valid_scheme_char(uint8_t a_char);

bool rust_net_is_valid_scheme(const nsACString *scheme);

void rust_parse_etc_hosts(const nsACString *path, ParsingCallback callback);

} // extern "C"

} // namespace net
} // namespace mozilla

#endif // mozilla_net_rustHelper_h
