# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG -DTRIMMED=1 -DNSPR20 -DYY_NO_UNPUT -DYY_NO_INPUT -DNSS_FIPS_DISABLED -DNSS_NO_INIT_SUPPORT -DNSS_X86_OR_X64 -DNSS_X64 -DNSS_USE_64 -DUSE_UTIL_DIRECTLY -DNO_NSPR_10_SUPPORT -DSSL_DISABLE_DEPRECATED_CIPHER_SUITE_NAMES -DLINUX2_1 -DLINUX -Dlinux -D_DEFAULT_SOURCE -D_BSD_SOURCE -D_POSIX_SOURCE -DSQL_MEASURE_USE_TEMP_DIR -DHAVE_STRERROR -DXP_UNIX -D_REENTRANT -DNSS_DISABLE_DBM -DNSS_ENABLE_DRAFT_HPKE -DNSS_DISABLE_LIBPKIX
LOCAL_INCLUDES += -I$(topobjdir)/dist/include/private/nss
LOCAL_INCLUDES += -I$(topobjdir)/dist/include/private/dbm
LOCAL_INCLUDES += -I$(topobjdir)/dist/include/nspr
LOCAL_INCLUDES += -I$(topobjdir)/dist/include/private/sectools
LOCAL_INCLUDES += -I$(topobjdir)/dist/include/nss
CSRCS += $(srcdir)/install-ds.c
CSRCS += $(srcdir)/install.c
CSRCS += $(srcdir)/installparse.c
CSRCS += $(srcdir)/instsec.c
CSRCS += $(srcdir)/lex.Pk11Install_yy.c
CSRCS += $(srcdir)/modutil.c
CSRCS += $(srcdir)/pk11.c
NO_PROFILE_GUIDED_OPTIMIZE := 1
COMPUTED_LDFLAGS += -lpthread -Wl,-z,noexecstack -Wl,-z,text -Wl,-z,relro -Wl,-z,nocopyreloc -Wl,-Bsymbolic-functions -Wl,--build-id=sha1 -fstack-protector-strong -fstack-clash-protection -Wl,-rpath-link,/worker/build/obj-x86_64-pc-linux-gnu/dist/bin -Wl,-rpath-link,/usr/local/lib
COMPUTED_CFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG -DTRIMMED=1 -DNSPR20 -DYY_NO_UNPUT -DYY_NO_INPUT -DNSS_FIPS_DISABLED -DNSS_NO_INIT_SUPPORT -DNSS_X86_OR_X64 -DNSS_X64 -DNSS_USE_64 -DUSE_UTIL_DIRECTLY -DNO_NSPR_10_SUPPORT -DSSL_DISABLE_DEPRECATED_CIPHER_SUITE_NAMES -DLINUX2_1 -DLINUX -Dlinux -D_DEFAULT_SOURCE -D_BSD_SOURCE -D_POSIX_SOURCE -DSQL_MEASURE_USE_TEMP_DIR -DHAVE_STRERROR -DXP_UNIX -D_REENTRANT -DNSS_DISABLE_DBM -DNSS_ENABLE_DRAFT_HPKE -DNSS_DISABLE_LIBPKIX -I/worker/build/security/nss/cmd/modutil -I/worker/build/obj-x86_64-pc-linux-gnu/security/nss/cmd/modutil/modutil_modutil -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/private/nss -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/private/dbm -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/private/sectools -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -fPIC -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -DMOZILLA_CLIENT -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi -fexperimental-new-pass-manager
COMPUTED_CXXFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/stl_wrappers -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG -DTRIMMED=1 -DNSPR20 -DYY_NO_UNPUT -DYY_NO_INPUT -DNSS_FIPS_DISABLED -DNSS_NO_INIT_SUPPORT -DNSS_X86_OR_X64 -DNSS_X64 -DNSS_USE_64 -DUSE_UTIL_DIRECTLY -DNO_NSPR_10_SUPPORT -DSSL_DISABLE_DEPRECATED_CIPHER_SUITE_NAMES -DLINUX2_1 -DLINUX -Dlinux -D_DEFAULT_SOURCE -D_BSD_SOURCE -D_POSIX_SOURCE -DSQL_MEASURE_USE_TEMP_DIR -DHAVE_STRERROR -DXP_UNIX -D_REENTRANT -DNSS_DISABLE_DBM -DNSS_ENABLE_DRAFT_HPKE -DNSS_DISABLE_LIBPKIX -I/worker/build/security/nss/cmd/modutil -I/worker/build/obj-x86_64-pc-linux-gnu/security/nss/cmd/modutil/modutil_modutil -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/private/nss -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/private/dbm -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/private/sectools -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -fPIC -DMOZILLA_CLIENT -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -fexperimental-new-pass-manager
COMPUTED_CXX_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables
COMPUTED_C_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi
PROGRAM = $(DEPTH)/dist/bin/modutil
PROG_IS_C_ONLY_modutil := 1
modutil_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/security/nss/cmd/modutil/modutil_modutil/modutil.list
$(DEPTH)/dist/bin/modutil: modutil.list
$(DEPTH)/dist/bin/modutil: install-ds.o \
    install.o \
    installparse.o \
    instsec.o \
    lex.Pk11Install_yy.o \
    modutil.o \
    pk11.o \
    ../../../lib/jar/jar_jar/jar-ds.o \
    ../../../lib/jar/jar_jar/jar.o \
    ../../../lib/jar/jar_jar/jarfile.o \
    ../../../lib/jar/jar_jar/jarint.o \
    ../../../lib/jar/jar_jar/jarsign.o \
    ../../../lib/jar/jar_jar/jarver.o \
    ../../lib/lib_sectool/basicutil.o \
    ../../lib/lib_sectool/derprint.o \
    ../../lib/lib_sectool/ffs.o \
    ../../lib/lib_sectool/moreoids.o \
    ../../lib/lib_sectool/pk11table.o \
    ../../lib/lib_sectool/pppolicy.o \
    ../../lib/lib_sectool/secpwd.o \
    ../../lib/lib_sectool/secutil.o
SHARED_LIBS += ../../../../../config/external/nspr/pr/libnspr4.so
SHARED_LIBS += ../../../../../config/external/nspr/libc/libplc4.so
SHARED_LIBS += ../../../../../config/external/nspr/ds/libplds4.so
SHARED_LIBS += ../../../lib/nss/nss_nss3/libnss3.so
SHARED_LIBS += ../../../lib/util/util_nssutil3/libnssutil3.so
SHARED_LIBS += ../../../lib/smime/smime_smime3/libsmime3.so
SHARED_LIBS += ../../../lib/ssl/ssl_ssl3/libssl3.so
OS_LIBS += -lpthread
OS_LIBS += -ldl
OS_LIBS += -lc
OS_LIBS += -lz
