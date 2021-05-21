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

enum class NegotiationStrategy {
  Filtering,
  Matching,
  Lookup,
};

extern "C" {

void fluent_langneg_negotiate_languages(const nsTArray<nsCString> *requested,
                                        const nsTArray<nsCString> *available,
                                        const nsACString *default_,
                                        NegotiationStrategy strategy,
                                        nsTArray<nsCString> *result);

} // extern "C"

} // namespace ffi
} // namespace intl
} // namespace mozilla
