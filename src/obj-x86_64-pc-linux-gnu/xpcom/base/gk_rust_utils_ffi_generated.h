/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/* Generated with cbindgen:0.16.0 */

/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen.
 * To generate this file:
 *   1. Get the latest cbindgen using `cargo install --force cbindgen`
 *      a. Alternatively, you can clone `https://github.com/eqrion/cbindgen` and use a tagged release
 *   2. Run `rustup run nightly cbindgen xpcom/rust/gkrust_utils --lockfile Cargo.lock --crate gkrust_utils -o xpcom/base/gk_rust_utils_ffi_generated.h`
 */
#include "nsError.h"
#include "nsString.h"


#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace mozilla {

extern "C" {

void GkRustUtils_GenerateUUID(nsACString *res);

bool GkRustUtils_ParseSemVer(const nsACString *ver,
                             uint64_t *out_major,
                             uint64_t *out_minor,
                             uint64_t *out_patch);

} // extern "C"

} // namespace mozilla
