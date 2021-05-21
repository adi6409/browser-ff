# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1
CSRCS += $(srcdir)/jcapimin.c
CSRCS += $(srcdir)/jcapistd.c
CSRCS += $(srcdir)/jccoefct.c
CSRCS += $(srcdir)/jccolor.c
CSRCS += $(srcdir)/jcdctmgr.c
CSRCS += $(srcdir)/jchuff.c
CSRCS += $(srcdir)/jcicc.c
CSRCS += $(srcdir)/jcinit.c
CSRCS += $(srcdir)/jcmainct.c
CSRCS += $(srcdir)/jcmarker.c
CSRCS += $(srcdir)/jcmaster.c
CSRCS += $(srcdir)/jcomapi.c
CSRCS += $(srcdir)/jcparam.c
CSRCS += $(srcdir)/jcphuff.c
CSRCS += $(srcdir)/jcprepct.c
CSRCS += $(srcdir)/jcsample.c
CSRCS += $(srcdir)/jctrans.c
CSRCS += $(srcdir)/jdapimin.c
CSRCS += $(srcdir)/jdapistd.c
CSRCS += $(srcdir)/jdatadst.c
CSRCS += $(srcdir)/jdatasrc.c
CSRCS += $(srcdir)/jdcoefct.c
CSRCS += $(srcdir)/jdcolor.c
CSRCS += $(srcdir)/jddctmgr.c
CSRCS += $(srcdir)/jdhuff.c
CSRCS += $(srcdir)/jdicc.c
CSRCS += $(srcdir)/jdinput.c
CSRCS += $(srcdir)/jdmainct.c
CSRCS += $(srcdir)/jdmarker.c
CSRCS += $(srcdir)/jdmaster.c
CSRCS += $(srcdir)/jdmerge.c
CSRCS += $(srcdir)/jdphuff.c
CSRCS += $(srcdir)/jdpostct.c
CSRCS += $(srcdir)/jdsample.c
CSRCS += $(srcdir)/jdtrans.c
CSRCS += $(srcdir)/jerror.c
CSRCS += $(srcdir)/jfdctflt.c
CSRCS += $(srcdir)/jfdctfst.c
CSRCS += $(srcdir)/jfdctint.c
CSRCS += $(srcdir)/jidctflt.c
CSRCS += $(srcdir)/jidctfst.c
CSRCS += $(srcdir)/jidctint.c
CSRCS += $(srcdir)/jidctred.c
CSRCS += $(srcdir)/jmemmgr.c
CSRCS += $(srcdir)/jmemnobs.c
CSRCS += $(srcdir)/jpeg_nbits_table.c
CSRCS += $(srcdir)/jquant1.c
CSRCS += $(srcdir)/jquant2.c
CSRCS += $(srcdir)/jutils.c
CSRCS += $(srcdir)/simd/x86_64/jsimd.c
ASFILES += $(srcdir)/simd/x86_64/jccolor-avx2.asm
ASFILES += $(srcdir)/simd/x86_64/jccolor-sse2.asm
ASFILES += $(srcdir)/simd/x86_64/jcgray-avx2.asm
ASFILES += $(srcdir)/simd/x86_64/jcgray-sse2.asm
ASFILES += $(srcdir)/simd/x86_64/jchuff-sse2.asm
ASFILES += $(srcdir)/simd/x86_64/jcphuff-sse2.asm
ASFILES += $(srcdir)/simd/x86_64/jcsample-avx2.asm
ASFILES += $(srcdir)/simd/x86_64/jcsample-sse2.asm
ASFILES += $(srcdir)/simd/x86_64/jdcolor-avx2.asm
ASFILES += $(srcdir)/simd/x86_64/jdcolor-sse2.asm
ASFILES += $(srcdir)/simd/x86_64/jdmerge-avx2.asm
ASFILES += $(srcdir)/simd/x86_64/jdmerge-sse2.asm
ASFILES += $(srcdir)/simd/x86_64/jdsample-avx2.asm
ASFILES += $(srcdir)/simd/x86_64/jdsample-sse2.asm
ASFILES += $(srcdir)/simd/x86_64/jfdctflt-sse.asm
ASFILES += $(srcdir)/simd/x86_64/jfdctfst-sse2.asm
ASFILES += $(srcdir)/simd/x86_64/jfdctint-avx2.asm
ASFILES += $(srcdir)/simd/x86_64/jfdctint-sse2.asm
ASFILES += $(srcdir)/simd/x86_64/jidctflt-sse2.asm
ASFILES += $(srcdir)/simd/x86_64/jidctfst-sse2.asm
ASFILES += $(srcdir)/simd/x86_64/jidctint-avx2.asm
ASFILES += $(srcdir)/simd/x86_64/jidctint-sse2.asm
ASFILES += $(srcdir)/simd/x86_64/jidctred-sse2.asm
ASFILES += $(srcdir)/simd/x86_64/jquantf-sse2.asm
ASFILES += $(srcdir)/simd/x86_64/jquanti-avx2.asm
ASFILES += $(srcdir)/simd/x86_64/jquanti-sse2.asm
ASFILES += $(srcdir)/simd/x86_64/jsimdcpu.asm
AS := /usr/sbin/yasm
ASOUTOPTION := -o 
AS_DASH_C_FLAG := 
COMPUTED_LDFLAGS += -lpthread -Wl,-z,noexecstack -Wl,-z,text -Wl,-z,relro -Wl,-z,nocopyreloc -Wl,-Bsymbolic-functions -Wl,--build-id=sha1 -fstack-protector-strong -fstack-clash-protection -Wl,-rpath-link,/worker/build/obj-x86_64-pc-linux-gnu/dist/bin -Wl,-rpath-link,/usr/local/lib
LIBRARY_NAME := media_libjpeg
FORCE_STATIC_LIB := 1
REAL_LIBRARY := libmedia_libjpeg.a
DEFINES += -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API
COMPUTED_CFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API -I/worker/build/media/libjpeg -I/worker/build/obj-x86_64-pc-linux-gnu/media/libjpeg -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -DMOZILLA_CLIENT -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi -fexperimental-new-pass-manager
COMPUTED_CXXFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/stl_wrappers -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API -I/worker/build/media/libjpeg -I/worker/build/obj-x86_64-pc-linux-gnu/media/libjpeg -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -DMOZILLA_CLIENT -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -fexperimental-new-pass-manager
COMPUTED_CXX_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables
COMPUTED_C_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi
COMPUTED_ASFLAGS += -f elf64 -rnasm -pnasm -g dwarf2 -D__x86_64__ -DPIC -DELF -I/worker/build/media/libjpeg/simd/nasm/ -I/worker/build/media/libjpeg/simd/x86_64/
COMPUTED_SFLAGS += -DNDEBUG=1 -DTRIMMED=1 -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API -f elf64 -rnasm -pnasm -g dwarf2 -D__x86_64__ -DPIC -DELF -I/worker/build/media/libjpeg/simd/nasm/ -I/worker/build/media/libjpeg/simd/x86_64/
