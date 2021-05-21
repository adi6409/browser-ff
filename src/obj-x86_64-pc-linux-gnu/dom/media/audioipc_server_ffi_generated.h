/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/* Generated with cbindgen:0.16.0 */

/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen. */

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace mozilla {
namespace audioipc {

#if defined(XP_WIN)
using PlatformHandleType = HANDLE;
#endif

#if defined(XP_UNIX)
using PlatformHandleType = int;
#endif

extern "C" {

void *audioipc_server_start(const char *aContextName,
                            const char *aBackendName);

PlatformHandleType audioipc_server_new_client(void *aP);

void audioipc_server_stop(void *aP);

} // extern "C"

} // namespace audioipc
} // namespace mozilla
