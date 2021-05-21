# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1 -DSKIA_IMPLEMENTATION=1
LOCAL_INCLUDES += -I$(srcdir)/skia
LOCAL_INCLUDES += -I$(srcdir)/skia/include/third_party/skcms
CPPSRCS += $(srcdir)/skia/src/core/SkBitmapProcState.cpp
CPPSRCS += $(srcdir)/skia/src/core/SkBitmapProcState_matrixProcs.cpp
CPPSRCS += $(srcdir)/skia/src/core/SkBitmapScaler.cpp
CPPSRCS += $(srcdir)/skia/src/core/SkBlitRow_D32.cpp
CPPSRCS += $(srcdir)/skia/src/core/SkBlitter.cpp
CPPSRCS += $(srcdir)/skia/src/core/SkBlitter_A8.cpp
CPPSRCS += $(srcdir)/skia/src/core/SkBlitter_ARGB32.cpp
CPPSRCS += $(srcdir)/skia/src/core/SkBlitter_RGB565.cpp
CPPSRCS += $(srcdir)/skia/src/core/SkBlitter_Sprite.cpp
CPPSRCS += $(srcdir)/skia/src/core/SkColorSpace.cpp
CPPSRCS += $(srcdir)/skia/src/core/SkColorSpaceXformSteps.cpp
CPPSRCS += $(srcdir)/skia/src/core/SkMatrix.cpp
CPPSRCS += $(srcdir)/skia/src/core/SkMatrix44.cpp
CPPSRCS += $(srcdir)/skia/src/core/SkMiniRecorder.cpp
CPPSRCS += $(srcdir)/skia/src/core/SkOpts.cpp
CPPSRCS += $(srcdir)/skia/src/core/SkPictureData.cpp
CPPSRCS += $(srcdir)/skia/src/core/SkRTree.cpp
CPPSRCS += $(srcdir)/skia/src/core/SkRecorder.cpp
CPPSRCS += $(srcdir)/skia/src/core/SkScan_AntiPath.cpp
CPPSRCS += $(srcdir)/skia/src/core/SkScan_Antihair.cpp
CPPSRCS += $(srcdir)/skia/src/core/SkSpriteBlitter_ARGB32.cpp
CPPSRCS += $(srcdir)/skia/src/core/SkSpriteBlitter_RGB565.cpp
CPPSRCS += $(srcdir)/skia/src/core/SkVertices.cpp
CPPSRCS += $(srcdir)/skia/src/core/SkXfermode.cpp
CPPSRCS += $(srcdir)/skia/src/core/SkXfermodeInterpretation.cpp
CPPSRCS += $(srcdir)/skia/src/opts/SkOpts_avx.cpp
CPPSRCS += $(srcdir)/skia/src/opts/SkOpts_hsw.cpp
CPPSRCS += $(srcdir)/skia/src/opts/SkOpts_sse41.cpp
CPPSRCS += $(srcdir)/skia/src/opts/SkOpts_sse42.cpp
CPPSRCS += $(srcdir)/skia/src/opts/SkOpts_ssse3.cpp
CPPSRCS += $(srcdir)/skia/src/pathops/SkPathOpsDebug.cpp
CPPSRCS += $(srcdir)/skia/src/ports/SkFontHost_FreeType_common.cpp
CPPSRCS += $(srcdir)/skia/src/ports/SkFontHost_cairo.cpp
CPPSRCS += $(srcdir)/skia/src/sksl/SkSLLexer.cpp
CPPSRCS += $(srcdir)/skia/src/utils/SkParse.cpp
CPPSRCS += $(srcdir)/skia/src/utils/SkParsePath.cpp
CPPSRCS += $(srcdir)/skia/third_party/skcms/skcms.cc

# We build files in 'unified' mode by including several files
# together into a single source file.  This cuts down on
# compilation times and debug information size.
UNIFIED_CPPSRCS := Unified_cpp_gfx_skia0.cpp Unified_cpp_gfx_skia1.cpp Unified_cpp_gfx_skia10.cpp Unified_cpp_gfx_skia11.cpp Unified_cpp_gfx_skia12.cpp Unified_cpp_gfx_skia13.cpp Unified_cpp_gfx_skia14.cpp Unified_cpp_gfx_skia15.cpp Unified_cpp_gfx_skia16.cpp Unified_cpp_gfx_skia17.cpp Unified_cpp_gfx_skia2.cpp Unified_cpp_gfx_skia3.cpp Unified_cpp_gfx_skia4.cpp Unified_cpp_gfx_skia5.cpp Unified_cpp_gfx_skia6.cpp Unified_cpp_gfx_skia7.cpp Unified_cpp_gfx_skia8.cpp Unified_cpp_gfx_skia9.cpp
CPPSRCS += $(UNIFIED_CPPSRCS)
SkBitmapProcState.cpp_FLAGS += -O3
SkBitmapProcState_matrixProcs.cpp_FLAGS += -O3
SkBitmapScaler.cpp_FLAGS += -O3
SkBlitRow_D32.cpp_FLAGS += -O3
SkBlitter.cpp_FLAGS += -O3
SkBlitter_A8.cpp_FLAGS += -O3
SkBlitter_ARGB32.cpp_FLAGS += -O3
SkBlitter_RGB565.cpp_FLAGS += -O3
SkBlitter_Sprite.cpp_FLAGS += -O3
SkMatrix.cpp_FLAGS += -O3
SkOpts.cpp_FLAGS += -O3
SkSpriteBlitter_ARGB32.cpp_FLAGS += -O3
SkSpriteBlitter_RGB565.cpp_FLAGS += -O3
SkOpts_avx.cpp_FLAGS += -O3
SkOpts_avx.cpp_FLAGS += -mavx
SkOpts_hsw.cpp_FLAGS += -O3
SkOpts_hsw.cpp_FLAGS += -mavx2
SkOpts_hsw.cpp_FLAGS += -mf16c
SkOpts_hsw.cpp_FLAGS += -mfma
SkOpts_sse41.cpp_FLAGS += -O3
SkOpts_sse41.cpp_FLAGS += -msse4.1
SkOpts_sse42.cpp_FLAGS += -O3
SkOpts_sse42.cpp_FLAGS += -msse4.2
SkOpts_ssse3.cpp_FLAGS += -O3
SkOpts_ssse3.cpp_FLAGS += -mssse3
skcms.cc_FLAGS += -O3
COMPUTED_LDFLAGS += -lpthread -Wl,-z,noexecstack -Wl,-z,text -Wl,-z,relro -Wl,-z,nocopyreloc -Wl,-Bsymbolic-functions -Wl,--build-id=sha1 -fstack-protector-strong -fstack-clash-protection -Wl,-rpath-link,/worker/build/obj-x86_64-pc-linux-gnu/dist/bin -Wl,-rpath-link,/usr/local/lib
LIBRARY_NAME := gfx_skia
FORCE_STATIC_LIB := 1
REAL_LIBRARY := libgfx_skia.a
DEFINES += -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API
COMPUTED_CFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DSKIA_IMPLEMENTATION=1 -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API -I/worker/build/gfx/skia -I/worker/build/obj-x86_64-pc-linux-gnu/gfx/skia -I/worker/build/gfx/skia/skia -I/worker/build/gfx/skia/skia/include/third_party/skcms -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -DMOZILLA_CLIENT -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi -fexperimental-new-pass-manager
COMPUTED_CXXFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/stl_wrappers -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DSKIA_IMPLEMENTATION=1 -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API -I/worker/build/gfx/skia -I/worker/build/obj-x86_64-pc-linux-gnu/gfx/skia -I/worker/build/gfx/skia/skia -I/worker/build/gfx/skia/skia/include/third_party/skcms -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -DMOZILLA_CLIENT -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Wno-deprecated-declarations -Wno-overloaded-virtual -Wno-shadow -Wno-sign-compare -Wno-unreachable-code -Wno-unused-function -Wno-implicit-fallthrough -Wno-inconsistent-missing-override -Wno-macro-redefined -Wno-unused-private-field -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/cairo -I/usr/include/freetype2 -I/usr/include/libpng16 -I/usr/include/harfbuzz -I/usr/include/glib-2.0 -I/usr/lib/glib-2.0/include -I/usr/include/freetype2 -I/usr/include/libpng16 -I/usr/include/harfbuzz -I/usr/include/glib-2.0 -I/usr/lib/glib-2.0/include -I/usr/include/pango-1.0 -I/usr/include/glib-2.0 -I/usr/lib/glib-2.0/include -I/usr/include/harfbuzz -I/usr/include/freetype2 -I/usr/include/libpng16 -I/usr/include/libmount -I/usr/include/blkid -I/usr/include/fribidi -I/usr/include/cairo -I/usr/include/lzo -I/usr/include/pixman-1 -pthread -fexperimental-new-pass-manager
COMPUTED_CXX_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables
COMPUTED_C_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi
