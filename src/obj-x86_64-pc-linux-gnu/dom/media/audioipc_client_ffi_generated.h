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

struct AudioIpcInitParams {
  PlatformHandleType mServerConnection;
  uintptr_t mPoolSize;
  uintptr_t mStackSize;
  void (*mThreadCreateCallback)(const char*);
  void (*mThreadDestroyCallback)();
};

extern "C" {

/// Entry point from C code.
int audioipc_client_init(cubeb **aC,
                         const char *aContextName,
                         const AudioIpcInitParams *aInitParams);

} // extern "C"

} // namespace audioipc
} // namespace mozilla
