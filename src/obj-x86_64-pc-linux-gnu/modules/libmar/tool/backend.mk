# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1 '-DMAR_CHANNEL_ID=""' '-DMOZ_APP_VERSION="87.0"'
CSRCS += $(srcdir)/mar.c
CPPSRCS += $(topsrcdir)/other-licenses/nsis/Contrib/CityHash/cityhash/city.cpp
HOST_DEFINES += -DNDEBUG=1 -DTRIMMED=1 -DNO_SIGN_VERIFY '-DMAR_CHANNEL_ID=""' '-DMOZ_APP_VERSION="87.0"'
HOST_CSRCS += $(srcdir)/mar.c
HOST_CPPSRCS += $(topsrcdir)/other-licenses/nsis/Contrib/CityHash/cityhash/city.cpp
COMPUTED_LDFLAGS += -lpthread -Wl,-z,noexecstack -Wl,-z,text -Wl,-z,relro -Wl,-z,nocopyreloc -Wl,-Bsymbolic-functions -Wl,--build-id=sha1 -fstack-protector-strong -fstack-clash-protection -Wl,-rpath-link,/worker/build/obj-x86_64-pc-linux-gnu/dist/bin -Wl,-rpath-link,/usr/local/lib
COMPUTED_HOST_CFLAGS += -DXP_UNIX -O2 -DNDEBUG=1 -DTRIMMED=1 -DNO_SIGN_VERIFY '-DMAR_CHANNEL_ID=""' '-DMOZ_APP_VERSION="87.0"' -std=gnu11 -I/worker/build/modules/libmar/tool -I/worker/build/obj-x86_64-pc-linux-gnu/modules/libmar/tool -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include
COMPUTED_HOST_CXXFLAGS += -O2 -DNDEBUG=1 -DTRIMMED=1 -DNO_SIGN_VERIFY '-DMAR_CHANNEL_ID=""' '-DMOZ_APP_VERSION="87.0"' -I/worker/build/modules/libmar/tool -I/worker/build/obj-x86_64-pc-linux-gnu/modules/libmar/tool -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include
COMPUTED_HOST_CXX_LDFLAGS += -O2
COMPUTED_HOST_C_LDFLAGS += -DXP_UNIX -O2 -std=gnu11
COMPUTED_CFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 '-DMAR_CHANNEL_ID=""' '-DMOZ_APP_VERSION="87.0"' -I/worker/build/modules/libmar/tool -I/worker/build/obj-x86_64-pc-linux-gnu/modules/libmar/tool -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -DMOZILLA_CLIENT -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi -std=gnu11 -fexperimental-new-pass-manager
COMPUTED_CXXFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 '-DMAR_CHANNEL_ID=""' '-DMOZ_APP_VERSION="87.0"' -I/worker/build/modules/libmar/tool -I/worker/build/obj-x86_64-pc-linux-gnu/modules/libmar/tool -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -DMOZILLA_CLIENT -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -fexperimental-new-pass-manager
COMPUTED_CXX_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables
COMPUTED_C_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi
PROGRAM = $(DEPTH)/dist/bin/signmar
signmar_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/modules/libmar/tool/signmar.list
$(DEPTH)/dist/bin/signmar: signmar.list
$(DEPTH)/dist/bin/signmar: mar.o \
    city.o \
    ../src/Unified_c_modules_libmar_src0.o \
    ../sign/Unified_c_modules_libmar_sign0.o \
    ../verify/Unified_c_libmar_verify0.o
SHARED_LIBS += ../../../config/external/nspr/pr/libnspr4.so
SHARED_LIBS += ../../../config/external/nspr/libc/libplc4.so
SHARED_LIBS += ../../../config/external/nspr/ds/libplds4.so
SHARED_LIBS += ../../../security/nss/lib/nss/nss_nss3/libnss3.so
SHARED_LIBS += ../../../security/nss/lib/util/util_nssutil3/libnssutil3.so
SHARED_LIBS += ../../../security/nss/lib/smime/smime_smime3/libsmime3.so
SHARED_LIBS += ../../../config/external/sqlite/libmozsqlite3.so
SHARED_LIBS += ../../../security/nss/lib/ssl/ssl_ssl3/libssl3.so
OS_LIBS += -Wl,-rpath=\$$ORIGIN
HOST_PROGRAM = $(DEPTH)/dist/host/bin/mar
mar_OBJS := @/worker/build/obj-x86_64-pc-linux-gnu/modules/libmar/tool/mar.list
mar: mar.list
mar: host_mar.o \
    host_city.o \
    ../src/host_mar_create.o \
    ../src/host_mar_extract.o \
    ../src/host_mar_read.o
