# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1 -DHAVE_AV_CONFIG_H
LOCAL_INCLUDES += -I$(topsrcdir)/media/ffvpx
CSRCS += $(srcdir)/constants.c
CSRCS += $(srcdir)/dct_init.c
CSRCS += $(srcdir)/fdct.c
CSRCS += $(srcdir)/fdctdsp_init.c
CSRCS += $(srcdir)/fft_init.c
CSRCS += $(srcdir)/flacdsp_init.c
CSRCS += $(srcdir)/h264_intrapred_init.c
CSRCS += $(srcdir)/idctdsp_init.c
CSRCS += $(srcdir)/mpegaudiodsp.c
CSRCS += $(srcdir)/videodsp_init.c
CSRCS += $(srcdir)/vp8dsp_init.c
CSRCS += $(srcdir)/vp9dsp_init.c
CSRCS += $(srcdir)/vp9dsp_init_10bpp.c
CSRCS += $(srcdir)/vp9dsp_init_12bpp.c
CSRCS += $(srcdir)/vp9dsp_init_16bpp.c
ASFILES += $(srcdir)/dct32.asm
ASFILES += $(srcdir)/fft.asm
ASFILES += $(srcdir)/flacdsp.asm
ASFILES += $(srcdir)/h264_intrapred.asm
ASFILES += $(srcdir)/h264_intrapred_10bit.asm
ASFILES += $(srcdir)/idctdsp.asm
ASFILES += $(srcdir)/imdct36.asm
ASFILES += $(srcdir)/simple_idct.asm
ASFILES += $(srcdir)/simple_idct10.asm
ASFILES += $(srcdir)/videodsp.asm
ASFILES += $(srcdir)/vp8dsp.asm
ASFILES += $(srcdir)/vp8dsp_loopfilter.asm
ASFILES += $(srcdir)/vp9intrapred.asm
ASFILES += $(srcdir)/vp9intrapred_16bpp.asm
ASFILES += $(srcdir)/vp9itxfm.asm
ASFILES += $(srcdir)/vp9itxfm_16bpp.asm
ASFILES += $(srcdir)/vp9lpf.asm
ASFILES += $(srcdir)/vp9lpf_16bpp.asm
ASFILES += $(srcdir)/vp9mc.asm
ASFILES += $(srcdir)/vp9mc_16bpp.asm
AS := /usr/sbin/yasm
ASOUTOPTION := -o 
AS_DASH_C_FLAG := 
COMPUTED_LDFLAGS += -lpthread -Wl,-z,noexecstack -Wl,-z,text -Wl,-z,relro -Wl,-z,nocopyreloc -Wl,-Bsymbolic-functions -Wl,--build-id=sha1 -fstack-protector-strong -fstack-clash-protection -Wl,-rpath-link,/worker/build/obj-x86_64-pc-linux-gnu/dist/bin -Wl,-rpath-link,/usr/local/lib
LIBRARY_NAME := media_ffvpx_libavcodec_x86
FORCE_STATIC_LIB := 1
REAL_LIBRARY := libmedia_ffvpx_libavcodec_x86.a
COMPUTED_CFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DHAVE_AV_CONFIG_H -I/worker/build/media/ffvpx/libavcodec/x86 -I/worker/build/obj-x86_64-pc-linux-gnu/media/ffvpx/libavcodec/x86 -I/worker/build/media/ffvpx -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -DMOZILLA_CLIENT -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi -Wno-parentheses -Wno-pointer-sign -Wno-sign-compare -Wno-switch -Wno-type-limits -Wno-unused-function -Wno-deprecated-declarations -Wno-absolute-value -Wno-incompatible-pointer-types -Wno-string-conversion -Wno-visibility -include libavutil_visibility.h -fexperimental-new-pass-manager
COMPUTED_CXXFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/stl_wrappers -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DHAVE_AV_CONFIG_H -I/worker/build/media/ffvpx/libavcodec/x86 -I/worker/build/obj-x86_64-pc-linux-gnu/media/ffvpx/libavcodec/x86 -I/worker/build/media/ffvpx -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -DMOZILLA_CLIENT -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -fexperimental-new-pass-manager
COMPUTED_CXX_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables
COMPUTED_C_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi
COMPUTED_ASFLAGS += -f elf64 -rnasm -pnasm -g dwarf2 -D__x86_64__ -DPIC -DELF -Pconfig_unix64.asm -Pdefaults_disabled.asm -I/worker/build/media/ffvpx/ -I/worker/build/media/ffvpx/libavcodec/x86/ -I/worker/build/media/ffvpx/libavutil/x86/
COMPUTED_SFLAGS += -DNDEBUG=1 -DTRIMMED=1 -DHAVE_AV_CONFIG_H -f elf64 -rnasm -pnasm -g dwarf2 -I/worker/build/media/ffvpx -D__x86_64__ -DPIC -DELF -Pconfig_unix64.asm -Pdefaults_disabled.asm -I/worker/build/media/ffvpx/ -I/worker/build/media/ffvpx/libavcodec/x86/ -I/worker/build/media/ffvpx/libavutil/x86/
