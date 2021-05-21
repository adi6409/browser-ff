# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1 -DHAVE_PTHREADS -DPACKAGE=mozpixman -D_USE_MATH_DEFINES -DUSE_MMX -DUSE_SSE -DUSE_SSE2
LOCAL_INCLUDES += -I$(topsrcdir)/gfx/cairo/cairo/src
CSRCS += $(srcdir)/pixman-access-accessors.c
CSRCS += $(srcdir)/pixman-access.c
CSRCS += $(srcdir)/pixman-arm.c
CSRCS += $(srcdir)/pixman-bits-image.c
CSRCS += $(srcdir)/pixman-combine-float.c
CSRCS += $(srcdir)/pixman-combine32.c
CSRCS += $(srcdir)/pixman-conical-gradient.c
CSRCS += $(srcdir)/pixman-edge-accessors.c
CSRCS += $(srcdir)/pixman-edge.c
CSRCS += $(srcdir)/pixman-fast-path.c
CSRCS += $(srcdir)/pixman-filter.c
CSRCS += $(srcdir)/pixman-general.c
CSRCS += $(srcdir)/pixman-glyph.c
CSRCS += $(srcdir)/pixman-gradient-walker.c
CSRCS += $(srcdir)/pixman-image.c
CSRCS += $(srcdir)/pixman-implementation.c
CSRCS += $(srcdir)/pixman-linear-gradient.c
CSRCS += $(srcdir)/pixman-matrix.c
CSRCS += $(srcdir)/pixman-mips.c
CSRCS += $(srcdir)/pixman-mmx.c
CSRCS += $(srcdir)/pixman-noop.c
CSRCS += $(srcdir)/pixman-ppc.c
CSRCS += $(srcdir)/pixman-radial-gradient.c
CSRCS += $(srcdir)/pixman-region16.c
CSRCS += $(srcdir)/pixman-region32.c
CSRCS += $(srcdir)/pixman-solid-fill.c
CSRCS += $(srcdir)/pixman-sse2.c
CSRCS += $(srcdir)/pixman-trap.c
CSRCS += $(srcdir)/pixman-utils.c
CSRCS += $(srcdir)/pixman-x86.c
CSRCS += $(srcdir)/pixman.c
pixman-mmx.c_FLAGS += -mmmx
pixman-mmx.c_FLAGS += -Winline
pixman-mmx.c_FLAGS += --param
pixman-mmx.c_FLAGS += inline-unit-growth=10000
pixman-mmx.c_FLAGS += --param
pixman-mmx.c_FLAGS += large-function-growth=10000
pixman-sse2.c_FLAGS += -msse
pixman-sse2.c_FLAGS += -msse2
pixman-sse2.c_FLAGS += -Winline
COMPUTED_LDFLAGS += -lpthread -Wl,-z,noexecstack -Wl,-z,text -Wl,-z,relro -Wl,-z,nocopyreloc -Wl,-Bsymbolic-functions -Wl,--build-id=sha1 -fstack-protector-strong -fstack-clash-protection -Wl,-rpath-link,/worker/build/obj-x86_64-pc-linux-gnu/dist/bin -Wl,-rpath-link,/usr/local/lib
LIBRARY_NAME := gfx_cairo_libpixman_src
FORCE_STATIC_LIB := 1
REAL_LIBRARY := libgfx_cairo_libpixman_src.a
DEFINES += -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API
COMPUTED_CFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DHAVE_PTHREADS -DPACKAGE=mozpixman -D_USE_MATH_DEFINES -DUSE_MMX -DUSE_SSE -DUSE_SSE2 -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API -I/worker/build/gfx/cairo/libpixman/src -I/worker/build/obj-x86_64-pc-linux-gnu/gfx/cairo/libpixman/src -I/worker/build/gfx/cairo/cairo/src -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -DMOZILLA_CLIENT -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi -Wno-address -Wno-missing-field-initializers -Wno-sign-compare -Wno-incompatible-pointer-types -Wno-unused -Wno-incompatible-pointer-types -Wno-tautological-compare -Wno-tautological-constant-out-of-range-compare -fexperimental-new-pass-manager
COMPUTED_CXXFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/stl_wrappers -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DHAVE_PTHREADS -DPACKAGE=mozpixman -D_USE_MATH_DEFINES -DUSE_MMX -DUSE_SSE -DUSE_SSE2 -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API -I/worker/build/gfx/cairo/libpixman/src -I/worker/build/obj-x86_64-pc-linux-gnu/gfx/cairo/libpixman/src -I/worker/build/gfx/cairo/cairo/src -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -DMOZILLA_CLIENT -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -fexperimental-new-pass-manager
COMPUTED_CXX_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables
COMPUTED_C_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi
