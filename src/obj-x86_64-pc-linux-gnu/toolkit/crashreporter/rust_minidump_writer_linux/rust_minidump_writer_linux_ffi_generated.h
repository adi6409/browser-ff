/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/* Generated with cbindgen:0.16.0 */

/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen. See RunCbindgen.py */


#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

bool write_minidump_linux(const char *dump_path,
                          pid_t child,
                          pid_t child_blamed_thread,
                          nsCString *error_msg);

bool write_minidump_linux_with_context(const char *dump_path,
                                       pid_t child,
                                       const void *context,
                                       nsCString *error_msg);

} // extern "C"
