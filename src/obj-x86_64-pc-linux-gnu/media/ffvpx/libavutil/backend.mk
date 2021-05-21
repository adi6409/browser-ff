# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1 -DHAVE_AV_CONFIG_H
DIRS := x86
LOCAL_INCLUDES += -I$(topsrcdir)/media/ffvpx
include $(topsrcdir)/config/AB_rCD.mk
PRE_COMPILE_TARGETS += $(MDDEPDIR)/libmozavutil.so.symbols.stub
libmozavutil.so.symbols: $(MDDEPDIR)/libmozavutil.so.symbols.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/libmozavutil.so.symbols.pp
$(MDDEPDIR)/libmozavutil.so.symbols.stub: /worker/build/python/mozbuild/mozbuild/action/generate_symbols_file.py $(srcdir)/avutil.symbols backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/generate_symbols_file.py generate_symbols_file libmozavutil.so.symbols $(MDDEPDIR)/libmozavutil.so.symbols.pp $(MDDEPDIR)/libmozavutil.so.symbols.stub $(srcdir)/avutil.symbols -DNDEBUG=1 -DTRIMMED=1 -DHAVE_AV_CONFIG_H)
	@$(TOUCH) $@

CSRCS += $(srcdir)/adler32.c
CSRCS += $(srcdir)/avstring.c
CSRCS += $(srcdir)/base64.c
CSRCS += $(srcdir)/bprint.c
CSRCS += $(srcdir)/buffer.c
CSRCS += $(srcdir)/channel_layout.c
CSRCS += $(srcdir)/color_utils.c
CSRCS += $(srcdir)/cpu.c
CSRCS += $(srcdir)/crc.c
CSRCS += $(srcdir)/dict.c
CSRCS += $(srcdir)/error.c
CSRCS += $(srcdir)/eval.c
CSRCS += $(srcdir)/fifo.c
CSRCS += $(srcdir)/fixed_dsp.c
CSRCS += $(srcdir)/float_dsp.c
CSRCS += $(srcdir)/frame.c
CSRCS += $(srcdir)/hwcontext.c
CSRCS += $(srcdir)/hwcontext_vaapi.c
CSRCS += $(srcdir)/imgutils.c
CSRCS += $(srcdir)/integer.c
CSRCS += $(srcdir)/intmath.c
CSRCS += $(srcdir)/lls.c
CSRCS += $(srcdir)/log.c
CSRCS += $(srcdir)/log2_tab.c
CSRCS += $(srcdir)/mathematics.c
CSRCS += $(srcdir)/mem.c
CSRCS += $(srcdir)/opt.c
CSRCS += $(srcdir)/parseutils.c
CSRCS += $(srcdir)/pixdesc.c
CSRCS += $(srcdir)/pixelutils.c
CSRCS += $(srcdir)/rational.c
CSRCS += $(srcdir)/reverse.c
CSRCS += $(srcdir)/samplefmt.c
CSRCS += $(srcdir)/slicethread.c
CSRCS += $(srcdir)/threadmessage.c
CSRCS += $(srcdir)/time.c
CSRCS += $(srcdir)/timecode.c
CSRCS += $(srcdir)/utils.c
AS := /usr/sbin/yasm
ASOUTOPTION := -o 
AS_DASH_C_FLAG := 
COMPUTED_LDFLAGS += -lpthread -Wl,-z,noexecstack -Wl,-z,text -Wl,-z,relro -Wl,-z,nocopyreloc -Wl,-Bsymbolic-functions -Wl,--build-id=sha1 -fstack-protector-strong -fstack-clash-protection -Wl,-rpath-link,/worker/build/obj-x86_64-pc-linux-gnu/dist/bin -Wl,-rpath-link,/usr/local/lib
LIBRARY_NAME := mozavutil
FORCE_SHARED_LIB := 1
IMPORT_LIBRARY := libmozavutil.so
SHARED_LIBRARY := libmozavutil.so
DSO_SONAME := libmozavutil.so
EXTRA_DSO_LDOPTS += -Wl,--version-script,libmozavutil.so.symbols
LIB_IS_C_ONLY := 1
libmozavutil.so_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/media/ffvpx/libavutil/libmozavutil_so.list
libmozavutil.so: libmozavutil_so.list
libmozavutil.so: adler32.o \
    avstring.o \
    base64.o \
    bprint.o \
    buffer.o \
    channel_layout.o \
    color_utils.o \
    cpu.o \
    crc.o \
    dict.o \
    error.o \
    eval.o \
    fifo.o \
    fixed_dsp.o \
    float_dsp.o \
    frame.o \
    hwcontext.o \
    hwcontext_vaapi.o \
    imgutils.o \
    integer.o \
    intmath.o \
    lls.o \
    log.o \
    log2_tab.o \
    mathematics.o \
    mem.o \
    opt.o \
    parseutils.o \
    pixdesc.o \
    pixelutils.o \
    rational.o \
    reverse.o \
    samplefmt.o \
    slicethread.o \
    threadmessage.o \
    time.o \
    timecode.o \
    utils.o \
    x86/cpu.o \
    x86/fixed_dsp_init.o \
    x86/float_dsp_init.o \
    x86/imgutils_init.o \
    x86/lls_init.o \
    x86/cpuid.o \
    x86/emms.o \
    x86/fixed_dsp.o \
    x86/float_dsp.o \
    x86/imgutils.o \
    x86/lls.o \
    ../mozva/mozva.o
OS_LIBS += -lm
COMPUTED_CFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DHAVE_AV_CONFIG_H -I/worker/build/media/ffvpx/libavutil -I/worker/build/obj-x86_64-pc-linux-gnu/media/ffvpx/libavutil -I/worker/build/media/ffvpx -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -DMOZILLA_CLIENT -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi -Wno-parentheses -Wno-pointer-sign -Wno-sign-compare -Wno-switch -Wno-type-limits -Wno-unused-function -Wno-deprecated-declarations -Wno-absolute-value -Wno-incompatible-pointer-types -Wno-string-conversion -Wno-visibility -include libavutil_visibility.h -fexperimental-new-pass-manager
COMPUTED_CXXFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/stl_wrappers -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DHAVE_AV_CONFIG_H -I/worker/build/media/ffvpx/libavutil -I/worker/build/obj-x86_64-pc-linux-gnu/media/ffvpx/libavutil -I/worker/build/media/ffvpx -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -DMOZILLA_CLIENT -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -fexperimental-new-pass-manager
COMPUTED_CXX_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables
COMPUTED_C_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi
