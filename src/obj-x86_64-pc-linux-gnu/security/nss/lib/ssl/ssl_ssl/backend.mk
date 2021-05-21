# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG -DTRIMMED=1 -DNSS_FIPS_DISABLED -DNSS_NO_INIT_SUPPORT -DNSS_X86_OR_X64 -DNSS_X64 -DNSS_USE_64 -DNSS_ALLOW_SSLKEYLOGFILE -DUSE_UTIL_DIRECTLY -DNO_NSPR_10_SUPPORT -DSSL_DISABLE_DEPRECATED_CIPHER_SUITE_NAMES -DLINUX2_1 -DLINUX -Dlinux -D_DEFAULT_SOURCE -D_BSD_SOURCE -D_POSIX_SOURCE -DSQL_MEASURE_USE_TEMP_DIR -DHAVE_STRERROR -DXP_UNIX -D_REENTRANT -DNSS_DISABLE_DBM -DNSS_ENABLE_DRAFT_HPKE -DNSS_DISABLE_LIBPKIX
LOCAL_INCLUDES += -I$(topobjdir)/dist/include/nspr
LOCAL_INCLUDES += -I$(topobjdir)/dist/include/private/nss
LOCAL_INCLUDES += -I$(topobjdir)/dist/include/nss
CSRCS += $(srcdir)/authcert.c
CSRCS += $(srcdir)/cmpcert.c
CSRCS += $(srcdir)/dtls13con.c
CSRCS += $(srcdir)/dtlscon.c
CSRCS += $(srcdir)/prelib.c
CSRCS += $(srcdir)/selfencrypt.c
CSRCS += $(srcdir)/ssl3con.c
CSRCS += $(srcdir)/ssl3ecc.c
CSRCS += $(srcdir)/ssl3ext.c
CSRCS += $(srcdir)/ssl3exthandle.c
CSRCS += $(srcdir)/ssl3gthr.c
CSRCS += $(srcdir)/sslauth.c
CSRCS += $(srcdir)/sslbloom.c
CSRCS += $(srcdir)/sslcert.c
CSRCS += $(srcdir)/sslcon.c
CSRCS += $(srcdir)/ssldef.c
CSRCS += $(srcdir)/sslencode.c
CSRCS += $(srcdir)/sslenum.c
CSRCS += $(srcdir)/sslerr.c
CSRCS += $(srcdir)/sslerrstrs.c
CSRCS += $(srcdir)/sslgrp.c
CSRCS += $(srcdir)/sslinfo.c
CSRCS += $(srcdir)/sslinit.c
CSRCS += $(srcdir)/sslmutex.c
CSRCS += $(srcdir)/sslnonce.c
CSRCS += $(srcdir)/sslprimitive.c
CSRCS += $(srcdir)/sslreveal.c
CSRCS += $(srcdir)/sslsecur.c
CSRCS += $(srcdir)/sslsnce.c
CSRCS += $(srcdir)/sslsock.c
CSRCS += $(srcdir)/sslspec.c
CSRCS += $(srcdir)/ssltrace.c
CSRCS += $(srcdir)/sslver.c
CSRCS += $(srcdir)/tls13con.c
CSRCS += $(srcdir)/tls13ech.c
CSRCS += $(srcdir)/tls13exthandle.c
CSRCS += $(srcdir)/tls13hashstate.c
CSRCS += $(srcdir)/tls13hkdf.c
CSRCS += $(srcdir)/tls13psk.c
CSRCS += $(srcdir)/tls13replay.c
CSRCS += $(srcdir)/tls13subcerts.c
CSRCS += $(srcdir)/unix_err.c
NO_PROFILE_GUIDED_OPTIMIZE := 1
COMPUTED_LDFLAGS += -lpthread -Wl,-z,noexecstack -Wl,-z,text -Wl,-z,relro -Wl,-z,nocopyreloc -Wl,-Bsymbolic-functions -Wl,--build-id=sha1 -fstack-protector-strong -fstack-clash-protection -Wl,-rpath-link,/worker/build/obj-x86_64-pc-linux-gnu/dist/bin -Wl,-rpath-link,/usr/local/lib
LIBRARY_NAME := ssl
FORCE_STATIC_LIB := 1
REAL_LIBRARY := libssl.a
OS_LIBS += -lpthread
OS_LIBS += -ldl
OS_LIBS += -lc
COMPUTED_CFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG -DTRIMMED=1 -DNSS_FIPS_DISABLED -DNSS_NO_INIT_SUPPORT -DNSS_X86_OR_X64 -DNSS_X64 -DNSS_USE_64 -DNSS_ALLOW_SSLKEYLOGFILE -DUSE_UTIL_DIRECTLY -DNO_NSPR_10_SUPPORT -DSSL_DISABLE_DEPRECATED_CIPHER_SUITE_NAMES -DLINUX2_1 -DLINUX -Dlinux -D_DEFAULT_SOURCE -D_BSD_SOURCE -D_POSIX_SOURCE -DSQL_MEASURE_USE_TEMP_DIR -DHAVE_STRERROR -DXP_UNIX -D_REENTRANT -DNSS_DISABLE_DBM -DNSS_ENABLE_DRAFT_HPKE -DNSS_DISABLE_LIBPKIX -I/worker/build/security/nss/lib/ssl -I/worker/build/obj-x86_64-pc-linux-gnu/security/nss/lib/ssl/ssl_ssl -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/private/nss -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -fPIC -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -DMOZILLA_CLIENT -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi -fexperimental-new-pass-manager
COMPUTED_CXXFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/stl_wrappers -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG -DTRIMMED=1 -DNSS_FIPS_DISABLED -DNSS_NO_INIT_SUPPORT -DNSS_X86_OR_X64 -DNSS_X64 -DNSS_USE_64 -DNSS_ALLOW_SSLKEYLOGFILE -DUSE_UTIL_DIRECTLY -DNO_NSPR_10_SUPPORT -DSSL_DISABLE_DEPRECATED_CIPHER_SUITE_NAMES -DLINUX2_1 -DLINUX -Dlinux -D_DEFAULT_SOURCE -D_BSD_SOURCE -D_POSIX_SOURCE -DSQL_MEASURE_USE_TEMP_DIR -DHAVE_STRERROR -DXP_UNIX -D_REENTRANT -DNSS_DISABLE_DBM -DNSS_ENABLE_DRAFT_HPKE -DNSS_DISABLE_LIBPKIX -I/worker/build/security/nss/lib/ssl -I/worker/build/obj-x86_64-pc-linux-gnu/security/nss/lib/ssl/ssl_ssl -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/private/nss -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -fPIC -DMOZILLA_CLIENT -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -fexperimental-new-pass-manager
COMPUTED_CXX_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables
COMPUTED_C_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi
