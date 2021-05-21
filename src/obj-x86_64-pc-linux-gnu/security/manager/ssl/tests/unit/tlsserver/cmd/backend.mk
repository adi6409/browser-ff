# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1 -DMOZ_HAS_MOZGLUE
LOCAL_INCLUDES += -I$(topsrcdir)/security/manager/ssl/tests/unit/tlsserver/lib
CPPSRCS += $(srcdir)/BadCertAndPinningServer.cpp
CPPSRCS += $(srcdir)/DelegatedCredentialsServer.cpp
CPPSRCS += $(srcdir)/EncryptedClientHelloServer.cpp
CPPSRCS += $(srcdir)/GenerateOCSPResponse.cpp
CPPSRCS += $(srcdir)/OCSPStaplingServer.cpp
CPPSRCS += $(srcdir)/SanctionsTestServer.cpp
COMPUTED_LDFLAGS += -lpthread -Wl,-z,noexecstack -Wl,-z,text -Wl,-z,relro -Wl,-z,nocopyreloc -Wl,-Bsymbolic-functions -Wl,--build-id=sha1 -fstack-protector-strong -fstack-clash-protection -rdynamic -Wl,-rpath-link,/worker/build/obj-x86_64-pc-linux-gnu/dist/bin -Wl,-rpath-link,/usr/local/lib
COMPUTED_CFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DMOZ_HAS_MOZGLUE -I/worker/build/security/manager/ssl/tests/unit/tlsserver/cmd -I/worker/build/obj-x86_64-pc-linux-gnu/security/manager/ssl/tests/unit/tlsserver/cmd -I/worker/build/security/manager/ssl/tests/unit/tlsserver/lib -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -DMOZILLA_CLIENT -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi -fexperimental-new-pass-manager
COMPUTED_CXXFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/stl_wrappers -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DMOZ_HAS_MOZGLUE -I/worker/build/security/manager/ssl/tests/unit/tlsserver/cmd -I/worker/build/obj-x86_64-pc-linux-gnu/security/manager/ssl/tests/unit/tlsserver/cmd -I/worker/build/security/manager/ssl/tests/unit/tlsserver/lib -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -DMOZILLA_CLIENT -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -I/worker/build/widget/gtk/compat-gtk3 -I/usr/include/gtk-3.0 -I/usr/include/pango-1.0 -I/usr/include/glib-2.0 -I/usr/lib/glib-2.0/include -I/usr/include/harfbuzz -I/usr/include/freetype2 -I/usr/include/libpng16 -I/usr/include/libmount -I/usr/include/blkid -I/usr/include/fribidi -I/usr/include/cairo -I/usr/include/lzo -I/usr/include/pixman-1 -I/usr/include/gdk-pixbuf-2.0 -I/usr/include/gio-unix-2.0 -I/usr/include/cloudproviders -I/usr/include/atk-1.0 -I/usr/include/at-spi2-atk/2.0 -I/usr/include/dbus-1.0 -I/usr/lib/dbus-1.0/include -I/usr/include/at-spi-2.0 -I/usr/include/gtk-3.0/unix-print -pthread -fexperimental-new-pass-manager
COMPUTED_CXX_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables
COMPUTED_C_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi
SIMPLE_PROGRAMS += BadCertAndPinningServer
BadCertAndPinningServer_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/security/manager/ssl/tests/unit/tlsserver/cmd/BadCertAndPinningServer.list
BadCertAndPinningServer: BadCertAndPinningServer.list
BadCertAndPinningServer: BadCertAndPinningServer.o \
    ../../../../../../../mozglue/build/SSE.o \
    ../../../../../../../mozglue/build/dummy.o \
    ../../../../../../../memory/build/Unified_cpp_memory_build0.o \
    ../../../../../../../memory/mozalloc/cxxalloc.o \
    ../../../../../../../memory/mozalloc/mozalloc_abort.o \
    ../../../../../../../memory/mozalloc/Unified_cpp_memory_mozalloc0.o \
    ../../../../../../../mozglue/baseprofiler/shared-libraries-linux.o \
    ../../../../../../../mozglue/baseprofiler/Unified_cpp_mozglue_baseprofiler0.o \
    ../../../../../../../mozglue/baseprofiler/Unified_cpp_mozglue_baseprofiler1.o \
    ../../../../../../../mozglue/misc/AutoProfilerLabel.o \
    ../../../../../../../mozglue/misc/ConditionVariable_posix.o \
    ../../../../../../../mozglue/misc/MmapFaultHandler.o \
    ../../../../../../../mozglue/misc/Mutex_posix.o \
    ../../../../../../../mozglue/misc/Printf.o \
    ../../../../../../../mozglue/misc/StackWalk.o \
    ../../../../../../../mozglue/misc/TimeStamp.o \
    ../../../../../../../mozglue/misc/TimeStamp_posix.o \
    ../../../../../../../mozglue/misc/Uptime.o \
    ../../../../../../../mozglue/misc/Decimal.o \
    ../../../../../../../mfbt/lz4.o \
    ../../../../../../../mfbt/lz4frame.o \
    ../../../../../../../mfbt/lz4hc.o \
    ../../../../../../../mfbt/xxhash.o \
    ../../../../../../../mfbt/Unified_cpp_mfbt0.o \
    ../../../../../../../mfbt/Unified_cpp_mfbt1.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixbuild.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixcert.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixcheck.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixder.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixnames.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixnss.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixocsp.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixresult.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixtime.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixverify.o \
    ../lib/Unified_cpp_unit_tlsserver_lib0.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix-testlib/pkixtestalg.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix-testlib/pkixtestnss.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix-testlib/pkixtestutil.o
SHARED_LIBS += ../../../../../../../config/external/nspr/pr/libnspr4.so
SHARED_LIBS += ../../../../../../../config/external/nspr/libc/libplc4.so
SHARED_LIBS += ../../../../../../../config/external/nspr/ds/libplds4.so
SHARED_LIBS += ../../../../../../nss/lib/nss/nss_nss3/libnss3.so
SHARED_LIBS += ../../../../../../nss/lib/util/util_nssutil3/libnssutil3.so
SHARED_LIBS += ../../../../../../nss/lib/smime/smime_smime3/libsmime3.so
SHARED_LIBS += ../../../../../../../config/external/sqlite/libmozsqlite3.so
SHARED_LIBS += ../../../../../../nss/lib/ssl/ssl_ssl3/libssl3.so
OS_LIBS += -lpthread
OS_LIBS += -ldl
OS_LIBS += -lc
SIMPLE_PROGRAMS += DelegatedCredentialsServer
DelegatedCredentialsServer_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/security/manager/ssl/tests/unit/tlsserver/cmd/DelegatedCredentialsServer.list
DelegatedCredentialsServer: DelegatedCredentialsServer.list
DelegatedCredentialsServer: DelegatedCredentialsServer.o \
    ../../../../../../../mozglue/build/SSE.o \
    ../../../../../../../mozglue/build/dummy.o \
    ../../../../../../../memory/build/Unified_cpp_memory_build0.o \
    ../../../../../../../memory/mozalloc/cxxalloc.o \
    ../../../../../../../memory/mozalloc/mozalloc_abort.o \
    ../../../../../../../memory/mozalloc/Unified_cpp_memory_mozalloc0.o \
    ../../../../../../../mozglue/baseprofiler/shared-libraries-linux.o \
    ../../../../../../../mozglue/baseprofiler/Unified_cpp_mozglue_baseprofiler0.o \
    ../../../../../../../mozglue/baseprofiler/Unified_cpp_mozglue_baseprofiler1.o \
    ../../../../../../../mozglue/misc/AutoProfilerLabel.o \
    ../../../../../../../mozglue/misc/ConditionVariable_posix.o \
    ../../../../../../../mozglue/misc/MmapFaultHandler.o \
    ../../../../../../../mozglue/misc/Mutex_posix.o \
    ../../../../../../../mozglue/misc/Printf.o \
    ../../../../../../../mozglue/misc/StackWalk.o \
    ../../../../../../../mozglue/misc/TimeStamp.o \
    ../../../../../../../mozglue/misc/TimeStamp_posix.o \
    ../../../../../../../mozglue/misc/Uptime.o \
    ../../../../../../../mozglue/misc/Decimal.o \
    ../../../../../../../mfbt/lz4.o \
    ../../../../../../../mfbt/lz4frame.o \
    ../../../../../../../mfbt/lz4hc.o \
    ../../../../../../../mfbt/xxhash.o \
    ../../../../../../../mfbt/Unified_cpp_mfbt0.o \
    ../../../../../../../mfbt/Unified_cpp_mfbt1.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixbuild.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixcert.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixcheck.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixder.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixnames.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixnss.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixocsp.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixresult.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixtime.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixverify.o \
    ../lib/Unified_cpp_unit_tlsserver_lib0.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix-testlib/pkixtestalg.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix-testlib/pkixtestnss.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix-testlib/pkixtestutil.o
SIMPLE_PROGRAMS += EncryptedClientHelloServer
EncryptedClientHelloServer_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/security/manager/ssl/tests/unit/tlsserver/cmd/EncryptedClientHelloServer.list
EncryptedClientHelloServer: EncryptedClientHelloServer.list
EncryptedClientHelloServer: EncryptedClientHelloServer.o \
    ../../../../../../../mozglue/build/SSE.o \
    ../../../../../../../mozglue/build/dummy.o \
    ../../../../../../../memory/build/Unified_cpp_memory_build0.o \
    ../../../../../../../memory/mozalloc/cxxalloc.o \
    ../../../../../../../memory/mozalloc/mozalloc_abort.o \
    ../../../../../../../memory/mozalloc/Unified_cpp_memory_mozalloc0.o \
    ../../../../../../../mozglue/baseprofiler/shared-libraries-linux.o \
    ../../../../../../../mozglue/baseprofiler/Unified_cpp_mozglue_baseprofiler0.o \
    ../../../../../../../mozglue/baseprofiler/Unified_cpp_mozglue_baseprofiler1.o \
    ../../../../../../../mozglue/misc/AutoProfilerLabel.o \
    ../../../../../../../mozglue/misc/ConditionVariable_posix.o \
    ../../../../../../../mozglue/misc/MmapFaultHandler.o \
    ../../../../../../../mozglue/misc/Mutex_posix.o \
    ../../../../../../../mozglue/misc/Printf.o \
    ../../../../../../../mozglue/misc/StackWalk.o \
    ../../../../../../../mozglue/misc/TimeStamp.o \
    ../../../../../../../mozglue/misc/TimeStamp_posix.o \
    ../../../../../../../mozglue/misc/Uptime.o \
    ../../../../../../../mozglue/misc/Decimal.o \
    ../../../../../../../mfbt/lz4.o \
    ../../../../../../../mfbt/lz4frame.o \
    ../../../../../../../mfbt/lz4hc.o \
    ../../../../../../../mfbt/xxhash.o \
    ../../../../../../../mfbt/Unified_cpp_mfbt0.o \
    ../../../../../../../mfbt/Unified_cpp_mfbt1.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixbuild.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixcert.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixcheck.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixder.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixnames.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixnss.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixocsp.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixresult.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixtime.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixverify.o \
    ../lib/Unified_cpp_unit_tlsserver_lib0.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix-testlib/pkixtestalg.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix-testlib/pkixtestnss.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix-testlib/pkixtestutil.o
SIMPLE_PROGRAMS += GenerateOCSPResponse
GenerateOCSPResponse_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/security/manager/ssl/tests/unit/tlsserver/cmd/GenerateOCSPResponse.list
GenerateOCSPResponse: GenerateOCSPResponse.list
GenerateOCSPResponse: GenerateOCSPResponse.o \
    ../../../../../../../mozglue/build/SSE.o \
    ../../../../../../../mozglue/build/dummy.o \
    ../../../../../../../memory/build/Unified_cpp_memory_build0.o \
    ../../../../../../../memory/mozalloc/cxxalloc.o \
    ../../../../../../../memory/mozalloc/mozalloc_abort.o \
    ../../../../../../../memory/mozalloc/Unified_cpp_memory_mozalloc0.o \
    ../../../../../../../mozglue/baseprofiler/shared-libraries-linux.o \
    ../../../../../../../mozglue/baseprofiler/Unified_cpp_mozglue_baseprofiler0.o \
    ../../../../../../../mozglue/baseprofiler/Unified_cpp_mozglue_baseprofiler1.o \
    ../../../../../../../mozglue/misc/AutoProfilerLabel.o \
    ../../../../../../../mozglue/misc/ConditionVariable_posix.o \
    ../../../../../../../mozglue/misc/MmapFaultHandler.o \
    ../../../../../../../mozglue/misc/Mutex_posix.o \
    ../../../../../../../mozglue/misc/Printf.o \
    ../../../../../../../mozglue/misc/StackWalk.o \
    ../../../../../../../mozglue/misc/TimeStamp.o \
    ../../../../../../../mozglue/misc/TimeStamp_posix.o \
    ../../../../../../../mozglue/misc/Uptime.o \
    ../../../../../../../mozglue/misc/Decimal.o \
    ../../../../../../../mfbt/lz4.o \
    ../../../../../../../mfbt/lz4frame.o \
    ../../../../../../../mfbt/lz4hc.o \
    ../../../../../../../mfbt/xxhash.o \
    ../../../../../../../mfbt/Unified_cpp_mfbt0.o \
    ../../../../../../../mfbt/Unified_cpp_mfbt1.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixbuild.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixcert.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixcheck.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixder.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixnames.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixnss.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixocsp.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixresult.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixtime.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixverify.o \
    ../lib/Unified_cpp_unit_tlsserver_lib0.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix-testlib/pkixtestalg.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix-testlib/pkixtestnss.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix-testlib/pkixtestutil.o
SIMPLE_PROGRAMS += OCSPStaplingServer
OCSPStaplingServer_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/security/manager/ssl/tests/unit/tlsserver/cmd/OCSPStaplingServer.list
OCSPStaplingServer: OCSPStaplingServer.list
OCSPStaplingServer: OCSPStaplingServer.o \
    ../../../../../../../mozglue/build/SSE.o \
    ../../../../../../../mozglue/build/dummy.o \
    ../../../../../../../memory/build/Unified_cpp_memory_build0.o \
    ../../../../../../../memory/mozalloc/cxxalloc.o \
    ../../../../../../../memory/mozalloc/mozalloc_abort.o \
    ../../../../../../../memory/mozalloc/Unified_cpp_memory_mozalloc0.o \
    ../../../../../../../mozglue/baseprofiler/shared-libraries-linux.o \
    ../../../../../../../mozglue/baseprofiler/Unified_cpp_mozglue_baseprofiler0.o \
    ../../../../../../../mozglue/baseprofiler/Unified_cpp_mozglue_baseprofiler1.o \
    ../../../../../../../mozglue/misc/AutoProfilerLabel.o \
    ../../../../../../../mozglue/misc/ConditionVariable_posix.o \
    ../../../../../../../mozglue/misc/MmapFaultHandler.o \
    ../../../../../../../mozglue/misc/Mutex_posix.o \
    ../../../../../../../mozglue/misc/Printf.o \
    ../../../../../../../mozglue/misc/StackWalk.o \
    ../../../../../../../mozglue/misc/TimeStamp.o \
    ../../../../../../../mozglue/misc/TimeStamp_posix.o \
    ../../../../../../../mozglue/misc/Uptime.o \
    ../../../../../../../mozglue/misc/Decimal.o \
    ../../../../../../../mfbt/lz4.o \
    ../../../../../../../mfbt/lz4frame.o \
    ../../../../../../../mfbt/lz4hc.o \
    ../../../../../../../mfbt/xxhash.o \
    ../../../../../../../mfbt/Unified_cpp_mfbt0.o \
    ../../../../../../../mfbt/Unified_cpp_mfbt1.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixbuild.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixcert.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixcheck.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixder.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixnames.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixnss.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixocsp.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixresult.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixtime.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixverify.o \
    ../lib/Unified_cpp_unit_tlsserver_lib0.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix-testlib/pkixtestalg.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix-testlib/pkixtestnss.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix-testlib/pkixtestutil.o
SIMPLE_PROGRAMS += SanctionsTestServer
SanctionsTestServer_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/security/manager/ssl/tests/unit/tlsserver/cmd/SanctionsTestServer.list
SanctionsTestServer: SanctionsTestServer.list
SanctionsTestServer: SanctionsTestServer.o \
    ../../../../../../../mozglue/build/SSE.o \
    ../../../../../../../mozglue/build/dummy.o \
    ../../../../../../../memory/build/Unified_cpp_memory_build0.o \
    ../../../../../../../memory/mozalloc/cxxalloc.o \
    ../../../../../../../memory/mozalloc/mozalloc_abort.o \
    ../../../../../../../memory/mozalloc/Unified_cpp_memory_mozalloc0.o \
    ../../../../../../../mozglue/baseprofiler/shared-libraries-linux.o \
    ../../../../../../../mozglue/baseprofiler/Unified_cpp_mozglue_baseprofiler0.o \
    ../../../../../../../mozglue/baseprofiler/Unified_cpp_mozglue_baseprofiler1.o \
    ../../../../../../../mozglue/misc/AutoProfilerLabel.o \
    ../../../../../../../mozglue/misc/ConditionVariable_posix.o \
    ../../../../../../../mozglue/misc/MmapFaultHandler.o \
    ../../../../../../../mozglue/misc/Mutex_posix.o \
    ../../../../../../../mozglue/misc/Printf.o \
    ../../../../../../../mozglue/misc/StackWalk.o \
    ../../../../../../../mozglue/misc/TimeStamp.o \
    ../../../../../../../mozglue/misc/TimeStamp_posix.o \
    ../../../../../../../mozglue/misc/Uptime.o \
    ../../../../../../../mozglue/misc/Decimal.o \
    ../../../../../../../mfbt/lz4.o \
    ../../../../../../../mfbt/lz4frame.o \
    ../../../../../../../mfbt/lz4hc.o \
    ../../../../../../../mfbt/xxhash.o \
    ../../../../../../../mfbt/Unified_cpp_mfbt0.o \
    ../../../../../../../mfbt/Unified_cpp_mfbt1.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixbuild.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixcert.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixcheck.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixder.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixnames.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixnss.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixocsp.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixresult.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixtime.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix/pkixverify.o \
    ../lib/Unified_cpp_unit_tlsserver_lib0.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix-testlib/pkixtestalg.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix-testlib/pkixtestnss.o \
    ../../../../../../nss/lib/mozpkix/mozpkix_mozpkix-testlib/pkixtestutil.o
