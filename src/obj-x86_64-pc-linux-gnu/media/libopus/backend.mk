# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1 -DOPUS_BUILD '-DOPUS_VERSION="v1.3-rc-19-g5cbd7d5f-mozilla"' -DUSE_ALLOCA -DENABLE_HARDENING -DOPUS_EXPORT= -DHAVE_LRINTF -DOPUS_HAVE_RTCD -DOPUS_X86_MAY_HAVE_SSE -DOPUS_X86_MAY_HAVE_SSE2 -DOPUS_X86_MAY_HAVE_SSE4_1 -DOPUS_X86_MAY_HAVE_AVX
LOCAL_INCLUDES += -I$(srcdir)/celt
LOCAL_INCLUDES += -I$(srcdir)/include
LOCAL_INCLUDES += -I$(srcdir)/silk
LOCAL_INCLUDES += -I$(srcdir)/silk/fixed
LOCAL_INCLUDES += -I$(srcdir)/silk/float
LOCAL_INCLUDES += -I$(srcdir)/src
CSRCS += $(srcdir)/celt/celt.c
CSRCS += $(srcdir)/celt/celt_decoder.c
CSRCS += $(srcdir)/celt/celt_encoder.c
CSRCS += $(srcdir)/celt/x86/celt_lpc_sse4_1.c
CSRCS += $(srcdir)/celt/x86/pitch_sse.c
CSRCS += $(srcdir)/celt/x86/pitch_sse2.c
CSRCS += $(srcdir)/celt/x86/pitch_sse4_1.c
CSRCS += $(srcdir)/celt/x86/vq_sse2.c
CSRCS += $(srcdir)/celt/x86/x86_celt_map.c
CSRCS += $(srcdir)/celt/x86/x86cpu.c
CSRCS += $(srcdir)/silk/LPC_inv_pred_gain.c
CSRCS += $(srcdir)/silk/NLSF2A.c
CSRCS += $(srcdir)/silk/x86/NSQ_del_dec_sse4_1.c
CSRCS += $(srcdir)/silk/x86/NSQ_sse4_1.c
CSRCS += $(srcdir)/silk/x86/VAD_sse4_1.c
CSRCS += $(srcdir)/silk/x86/VQ_WMat_EC_sse4_1.c
CSRCS += $(srcdir)/silk/x86/x86_silk_map.c

# We build files in 'unified' mode by including several files
# together into a single source file.  This cuts down on
# compilation times and debug information size.
UNIFIED_CSRCS := Unified_c_media_libopus0.c Unified_c_media_libopus1.c Unified_c_media_libopus2.c Unified_c_media_libopus3.c Unified_c_media_libopus4.c Unified_c_media_libopus5.c Unified_c_media_libopus6.c Unified_c_media_libopus7.c Unified_c_media_libopus8.c
CSRCS += $(UNIFIED_CSRCS)
celt_lpc_sse4_1.c_FLAGS += -msse4.1
pitch_sse.c_FLAGS += -msse
pitch_sse2.c_FLAGS += -msse2
pitch_sse4_1.c_FLAGS += -msse4.1
vq_sse2.c_FLAGS += -msse2
x86_celt_map.c_FLAGS += -msse
x86cpu.c_FLAGS += -msse
NSQ_del_dec_sse4_1.c_FLAGS += -msse4.1
NSQ_sse4_1.c_FLAGS += -msse4.1
VAD_sse4_1.c_FLAGS += -msse4.1
VQ_WMat_EC_sse4_1.c_FLAGS += -msse4.1
x86_silk_map.c_FLAGS += -msse4.1
COMPUTED_LDFLAGS += -lpthread -Wl,-z,noexecstack -Wl,-z,text -Wl,-z,relro -Wl,-z,nocopyreloc -Wl,-Bsymbolic-functions -Wl,--build-id=sha1 -fstack-protector-strong -fstack-clash-protection -Wl,-rpath-link,/worker/build/obj-x86_64-pc-linux-gnu/dist/bin -Wl,-rpath-link,/usr/local/lib
LIBRARY_NAME := media_libopus
FORCE_STATIC_LIB := 1
REAL_LIBRARY := libmedia_libopus.a
DEFINES += -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API
COMPUTED_CFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DOPUS_BUILD '-DOPUS_VERSION="v1.3-rc-19-g5cbd7d5f-mozilla"' -DUSE_ALLOCA -DENABLE_HARDENING -DOPUS_EXPORT= -DHAVE_LRINTF -DOPUS_HAVE_RTCD -DOPUS_X86_MAY_HAVE_SSE -DOPUS_X86_MAY_HAVE_SSE2 -DOPUS_X86_MAY_HAVE_SSE4_1 -DOPUS_X86_MAY_HAVE_AVX -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API -I/worker/build/media/libopus -I/worker/build/obj-x86_64-pc-linux-gnu/media/libopus -I/worker/build/media/libopus/celt -I/worker/build/media/libopus/include -I/worker/build/media/libopus/silk -I/worker/build/media/libopus/silk/fixed -I/worker/build/media/libopus/silk/float -I/worker/build/media/libopus/src -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -DMOZILLA_CLIENT -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi '-Wno-\#pragma-messages' -fexperimental-new-pass-manager
COMPUTED_CXXFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/stl_wrappers -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DOPUS_BUILD '-DOPUS_VERSION="v1.3-rc-19-g5cbd7d5f-mozilla"' -DUSE_ALLOCA -DENABLE_HARDENING -DOPUS_EXPORT= -DHAVE_LRINTF -DOPUS_HAVE_RTCD -DOPUS_X86_MAY_HAVE_SSE -DOPUS_X86_MAY_HAVE_SSE2 -DOPUS_X86_MAY_HAVE_SSE4_1 -DOPUS_X86_MAY_HAVE_AVX -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API -I/worker/build/media/libopus -I/worker/build/obj-x86_64-pc-linux-gnu/media/libopus -I/worker/build/media/libopus/celt -I/worker/build/media/libopus/include -I/worker/build/media/libopus/silk -I/worker/build/media/libopus/silk/fixed -I/worker/build/media/libopus/silk/float -I/worker/build/media/libopus/src -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -DMOZILLA_CLIENT -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -fexperimental-new-pass-manager
COMPUTED_CXX_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables
COMPUTED_C_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi
