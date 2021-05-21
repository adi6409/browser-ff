/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/* The MozURL type is implemented in Rust code, and uses extern "C" FFI calls to
 * operate on the internal data structure. This file contains C declarations of
 * these files.

 * WARNING: DO NOT CALL ANY OF THESE FUNCTIONS. USE |MozURL| INSTEAD! */
 

#ifndef mozilla_net_MozURL_ffi_h
#define mozilla_net_MozURL_ffi_h

/* Generated with cbindgen:0.16.0 */

/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen. */

namespace mozilla {
namespace net {
class MozURL;
}  // namespace net
}  // namespace mozilla

extern "C" {

// FFI-compatible string slice struct used internally by MozURL.
// Coerces to nsDependentCSubstring.
struct MozURLSpecSlice {
  char* mData;
  uint32_t mLen;

  operator nsDependentCSubstring() {
    return nsDependentCSubstring(mData, mLen);
  }
};

nsresult mozurl_new(mozilla::net::MozURL** aResult, const nsACString* aSpec,
                    /* optional */ const mozilla::net::MozURL* aBase);

void mozurl_clone(const mozilla::net::MozURL* aThis,
                  mozilla::net::MozURL** aResult);

nsresult mozurl_common_base(const mozilla::net::MozURL* aUrl1,
                            const mozilla::net::MozURL* aUrl2,
                            mozilla::net::MozURL** aResult);
}



#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>
#include "nsError.h"
#include "nsString.h"

extern "C" {

nsrefcnt mozurl_addref(const mozilla::net::MozURL *url);

nsrefcnt mozurl_release(const mozilla::net::MozURL *url);

MozURLSpecSlice mozurl_spec(const mozilla::net::MozURL *url);

MozURLSpecSlice mozurl_scheme(const mozilla::net::MozURL *url);

MozURLSpecSlice mozurl_username(const mozilla::net::MozURL *url);

MozURLSpecSlice mozurl_password(const mozilla::net::MozURL *url);

MozURLSpecSlice mozurl_host(const mozilla::net::MozURL *url);

int32_t mozurl_port(const mozilla::net::MozURL *url);

int32_t mozurl_real_port(const mozilla::net::MozURL *url);

MozURLSpecSlice mozurl_host_port(const mozilla::net::MozURL *url);

MozURLSpecSlice mozurl_filepath(const mozilla::net::MozURL *url);

MozURLSpecSlice mozurl_path(const mozilla::net::MozURL *url);

MozURLSpecSlice mozurl_query(const mozilla::net::MozURL *url);

MozURLSpecSlice mozurl_fragment(const mozilla::net::MozURL *url);

MozURLSpecSlice mozurl_spec_no_ref(const mozilla::net::MozURL *url);

bool mozurl_has_fragment(const mozilla::net::MozURL *url);

MozURLSpecSlice mozurl_directory(const mozilla::net::MozURL *url);

MozURLSpecSlice mozurl_prepath(const mozilla::net::MozURL *url);

void mozurl_origin(const mozilla::net::MozURL *url, nsACString *origin);

nsresult mozurl_base_domain(const mozilla::net::MozURL *url, nsACString *base_domain);

nsresult mozurl_set_scheme(mozilla::net::MozURL *url, const nsACString *scheme);

nsresult mozurl_set_username(mozilla::net::MozURL *url, const nsACString *username);

nsresult mozurl_set_password(mozilla::net::MozURL *url, const nsACString *password);

nsresult mozurl_set_host_port(mozilla::net::MozURL *url, const nsACString *hostport);

nsresult mozurl_set_hostname(mozilla::net::MozURL *url, const nsACString *host);

nsresult mozurl_set_port_no(mozilla::net::MozURL *url, int32_t new_port);

nsresult mozurl_set_pathname(mozilla::net::MozURL *url, const nsACString *path);

nsresult mozurl_set_query(mozilla::net::MozURL *url, const nsACString *query);

nsresult mozurl_set_fragment(mozilla::net::MozURL *url, const nsACString *fragment);

uintptr_t mozurl_sizeof(const mozilla::net::MozURL *url);

nsresult mozurl_relative(const mozilla::net::MozURL *url1,
                         const mozilla::net::MozURL *url2,
                         nsACString *result);

/// This type is used by nsStandardURL
nsresult rusturl_parse_ipv6addr(const nsACString *input, nsACString *addr);

} // extern "C"

#endif // mozilla_net_MozURL_ffi_h
