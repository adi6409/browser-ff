# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1
CPPSRCS += $(srcdir)/e_acos.cpp
CPPSRCS += $(srcdir)/e_acosh.cpp
CPPSRCS += $(srcdir)/e_asin.cpp
CPPSRCS += $(srcdir)/e_atan2.cpp
CPPSRCS += $(srcdir)/e_atanh.cpp
CPPSRCS += $(srcdir)/e_cosh.cpp
CPPSRCS += $(srcdir)/e_exp.cpp
CPPSRCS += $(srcdir)/e_hypot.cpp
CPPSRCS += $(srcdir)/e_log.cpp
CPPSRCS += $(srcdir)/e_log10.cpp
CPPSRCS += $(srcdir)/e_log2.cpp
CPPSRCS += $(srcdir)/e_pow.cpp
CPPSRCS += $(srcdir)/e_sinh.cpp
CPPSRCS += $(srcdir)/k_exp.cpp
CPPSRCS += $(srcdir)/s_asinh.cpp
CPPSRCS += $(srcdir)/s_atan.cpp
CPPSRCS += $(srcdir)/s_cbrt.cpp
CPPSRCS += $(srcdir)/s_ceil.cpp
CPPSRCS += $(srcdir)/s_ceilf.cpp
CPPSRCS += $(srcdir)/s_copysign.cpp
CPPSRCS += $(srcdir)/s_expm1.cpp
CPPSRCS += $(srcdir)/s_fabs.cpp
CPPSRCS += $(srcdir)/s_floor.cpp
CPPSRCS += $(srcdir)/s_floorf.cpp
CPPSRCS += $(srcdir)/s_log1p.cpp
CPPSRCS += $(srcdir)/s_nearbyint.cpp
CPPSRCS += $(srcdir)/s_rint.cpp
CPPSRCS += $(srcdir)/s_rintf.cpp
CPPSRCS += $(srcdir)/s_scalbn.cpp
CPPSRCS += $(srcdir)/s_tanh.cpp
CPPSRCS += $(srcdir)/s_trunc.cpp
CPPSRCS += $(srcdir)/s_truncf.cpp
COMPUTED_LDFLAGS += -lpthread -Wl,-z,noexecstack -Wl,-z,text -Wl,-z,relro -Wl,-z,nocopyreloc -Wl,-Bsymbolic-functions -Wl,--build-id=sha1 -fstack-protector-strong -fstack-clash-protection -Wl,-rpath-link,/worker/build/obj-x86_64-pc-linux-gnu/dist/bin -Wl,-rpath-link,/usr/local/lib
LIBRARY_NAME := modules_fdlibm_src
FORCE_STATIC_LIB := 1
REAL_LIBRARY := libmodules_fdlibm_src.a
DEFINES += -DEXPORT_JS_API -DMOZ_HAS_MOZGLUE
COMPUTED_CFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DEXPORT_JS_API -DMOZ_HAS_MOZGLUE -I/worker/build/modules/fdlibm/src -I/worker/build/obj-x86_64-pc-linux-gnu/modules/fdlibm/src -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -DMOZILLA_CLIENT -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi -fexperimental-new-pass-manager
COMPUTED_CXXFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/stl_wrappers -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DEXPORT_JS_API -DMOZ_HAS_MOZGLUE -I/worker/build/modules/fdlibm/src -I/worker/build/obj-x86_64-pc-linux-gnu/modules/fdlibm/src -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -DMOZILLA_CLIENT -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Wno-parentheses -Wno-sign-compare -Wno-dangling-else -fexperimental-new-pass-manager
COMPUTED_CXX_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables
COMPUTED_C_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi
