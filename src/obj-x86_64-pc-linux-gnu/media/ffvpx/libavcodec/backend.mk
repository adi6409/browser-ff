# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1 -DHAVE_AV_CONFIG_H
DIRS := x86
LOCAL_INCLUDES += -I$(topsrcdir)/media/ffvpx
include $(topsrcdir)/config/AB_rCD.mk
PRE_COMPILE_TARGETS += $(MDDEPDIR)/libmozavcodec.so.symbols.stub
libmozavcodec.so.symbols: $(MDDEPDIR)/libmozavcodec.so.symbols.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/libmozavcodec.so.symbols.pp
$(MDDEPDIR)/libmozavcodec.so.symbols.stub: /worker/build/python/mozbuild/mozbuild/action/generate_symbols_file.py $(srcdir)/avcodec.symbols backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/python/mozbuild/mozbuild/action/generate_symbols_file.py generate_symbols_file libmozavcodec.so.symbols $(MDDEPDIR)/libmozavcodec.so.symbols.pp $(MDDEPDIR)/libmozavcodec.so.symbols.stub $(srcdir)/avcodec.symbols -DNDEBUG=1 -DTRIMMED=1 -DHAVE_AV_CONFIG_H)
	@$(TOUCH) $@

CSRCS += $(srcdir)/allcodecs.c
CSRCS += $(srcdir)/avdct.c
CSRCS += $(srcdir)/avfft.c
CSRCS += $(srcdir)/avpacket.c
CSRCS += $(srcdir)/avpicture.c
CSRCS += $(srcdir)/bitstream.c
CSRCS += $(srcdir)/bitstream_filter.c
CSRCS += $(srcdir)/bitstream_filters.c
CSRCS += $(srcdir)/bsf.c
CSRCS += $(srcdir)/codec_desc.c
CSRCS += $(srcdir)/dct.c
CSRCS += $(srcdir)/dct32_fixed.c
CSRCS += $(srcdir)/dct32_float.c
CSRCS += $(srcdir)/decode.c
CSRCS += $(srcdir)/faandct.c
CSRCS += $(srcdir)/faanidct.c
CSRCS += $(srcdir)/fdctdsp.c
CSRCS += $(srcdir)/fft_fixed.c
CSRCS += $(srcdir)/fft_fixed_32.c
CSRCS += $(srcdir)/fft_float.c
CSRCS += $(srcdir)/fft_init_table.c
CSRCS += $(srcdir)/flac.c
CSRCS += $(srcdir)/flacdata.c
CSRCS += $(srcdir)/flacdec.c
CSRCS += $(srcdir)/flacdsp.c
CSRCS += $(srcdir)/golomb.c
CSRCS += $(srcdir)/h264pred.c
CSRCS += $(srcdir)/idctdsp.c
CSRCS += $(srcdir)/imgconvert.c
CSRCS += $(srcdir)/jfdctfst.c
CSRCS += $(srcdir)/jfdctint.c
CSRCS += $(srcdir)/jrevdct.c
CSRCS += $(srcdir)/log2_tab.c
CSRCS += $(srcdir)/mathtables.c
CSRCS += $(srcdir)/mpegaudio.c
CSRCS += $(srcdir)/mpegaudiodata.c
CSRCS += $(srcdir)/mpegaudiodec_fixed.c
CSRCS += $(srcdir)/mpegaudiodecheader.c
CSRCS += $(srcdir)/mpegaudiodsp.c
CSRCS += $(srcdir)/mpegaudiodsp_data.c
CSRCS += $(srcdir)/mpegaudiodsp_fixed.c
CSRCS += $(srcdir)/mpegaudiodsp_float.c
CSRCS += $(srcdir)/null_bsf.c
CSRCS += $(srcdir)/options.c
CSRCS += $(srcdir)/parser.c
CSRCS += $(srcdir)/parsers.c
CSRCS += $(srcdir)/profiles.c
CSRCS += $(srcdir)/pthread.c
CSRCS += $(srcdir)/pthread_frame.c
CSRCS += $(srcdir)/pthread_slice.c
CSRCS += $(srcdir)/qsv_api.c
CSRCS += $(srcdir)/raw.c
CSRCS += $(srcdir)/rdft.c
CSRCS += $(srcdir)/reverse.c
CSRCS += $(srcdir)/simple_idct.c
CSRCS += $(srcdir)/utils.c
CSRCS += $(srcdir)/vaapi_decode.c
CSRCS += $(srcdir)/vaapi_vp8.c
CSRCS += $(srcdir)/vaapi_vp9.c
CSRCS += $(srcdir)/videodsp.c
CSRCS += $(srcdir)/vorbis_parser.c
CSRCS += $(srcdir)/vp56rac.c
CSRCS += $(srcdir)/vp8.c
CSRCS += $(srcdir)/vp8_parser.c
CSRCS += $(srcdir)/vp8dsp.c
CSRCS += $(srcdir)/vp9.c
CSRCS += $(srcdir)/vp9_parser.c
CSRCS += $(srcdir)/vp9_superframe_split_bsf.c
CSRCS += $(srcdir)/vp9block.c
CSRCS += $(srcdir)/vp9data.c
CSRCS += $(srcdir)/vp9dsp.c
CSRCS += $(srcdir)/vp9dsp_10bpp.c
CSRCS += $(srcdir)/vp9dsp_12bpp.c
CSRCS += $(srcdir)/vp9dsp_8bpp.c
CSRCS += $(srcdir)/vp9lpf.c
CSRCS += $(srcdir)/vp9mvs.c
CSRCS += $(srcdir)/vp9prob.c
CSRCS += $(srcdir)/vp9recon.c
CSRCS += $(srcdir)/xiph.c
AS := /usr/sbin/yasm
ASOUTOPTION := -o 
AS_DASH_C_FLAG := 
COMPUTED_LDFLAGS += -lpthread -Wl,-z,noexecstack -Wl,-z,text -Wl,-z,relro -Wl,-z,nocopyreloc -Wl,-Bsymbolic-functions -Wl,--build-id=sha1 -fstack-protector-strong -fstack-clash-protection -Wl,-rpath-link,/worker/build/obj-x86_64-pc-linux-gnu/dist/bin -Wl,-rpath-link,/usr/local/lib
LIBRARY_NAME := mozavcodec
FORCE_SHARED_LIB := 1
IMPORT_LIBRARY := libmozavcodec.so
SHARED_LIBRARY := libmozavcodec.so
DSO_SONAME := libmozavcodec.so
EXTRA_DSO_LDOPTS += -Wl,--version-script,libmozavcodec.so.symbols
LIB_IS_C_ONLY := 1
libmozavcodec.so_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/media/ffvpx/libavcodec/libmozavcodec_so.list
libmozavcodec.so: libmozavcodec_so.list
libmozavcodec.so: allcodecs.o \
    avdct.o \
    avfft.o \
    avpacket.o \
    avpicture.o \
    bitstream.o \
    bitstream_filter.o \
    bitstream_filters.o \
    bsf.o \
    codec_desc.o \
    dct.o \
    dct32_fixed.o \
    dct32_float.o \
    decode.o \
    faandct.o \
    faanidct.o \
    fdctdsp.o \
    fft_fixed.o \
    fft_fixed_32.o \
    fft_float.o \
    fft_init_table.o \
    flac.o \
    flacdata.o \
    flacdec.o \
    flacdsp.o \
    golomb.o \
    h264pred.o \
    idctdsp.o \
    imgconvert.o \
    jfdctfst.o \
    jfdctint.o \
    jrevdct.o \
    log2_tab.o \
    mathtables.o \
    mpegaudio.o \
    mpegaudiodata.o \
    mpegaudiodec_fixed.o \
    mpegaudiodecheader.o \
    mpegaudiodsp.o \
    mpegaudiodsp_data.o \
    mpegaudiodsp_fixed.o \
    mpegaudiodsp_float.o \
    null_bsf.o \
    options.o \
    parser.o \
    parsers.o \
    profiles.o \
    pthread.o \
    pthread_frame.o \
    pthread_slice.o \
    qsv_api.o \
    raw.o \
    rdft.o \
    reverse.o \
    simple_idct.o \
    utils.o \
    vaapi_decode.o \
    vaapi_vp8.o \
    vaapi_vp9.o \
    videodsp.o \
    vorbis_parser.o \
    vp56rac.o \
    vp8.o \
    vp8_parser.o \
    vp8dsp.o \
    vp9.o \
    vp9_parser.o \
    vp9_superframe_split_bsf.o \
    vp9block.o \
    vp9data.o \
    vp9dsp.o \
    vp9dsp_10bpp.o \
    vp9dsp_12bpp.o \
    vp9dsp_8bpp.o \
    vp9lpf.o \
    vp9mvs.o \
    vp9prob.o \
    vp9recon.o \
    xiph.o \
    x86/constants.o \
    x86/dct_init.o \
    x86/fdct.o \
    x86/fdctdsp_init.o \
    x86/fft_init.o \
    x86/flacdsp_init.o \
    x86/h264_intrapred_init.o \
    x86/idctdsp_init.o \
    x86/mpegaudiodsp.o \
    x86/videodsp_init.o \
    x86/vp8dsp_init.o \
    x86/vp9dsp_init.o \
    x86/vp9dsp_init_10bpp.o \
    x86/vp9dsp_init_12bpp.o \
    x86/vp9dsp_init_16bpp.o \
    x86/dct32.o \
    x86/fft.o \
    x86/flacdsp.o \
    x86/h264_intrapred.o \
    x86/h264_intrapred_10bit.o \
    x86/idctdsp.o \
    x86/imdct36.o \
    x86/simple_idct.o \
    x86/simple_idct10.o \
    x86/videodsp.o \
    x86/vp8dsp.o \
    x86/vp8dsp_loopfilter.o \
    x86/vp9intrapred.o \
    x86/vp9intrapred_16bpp.o \
    x86/vp9itxfm.o \
    x86/vp9itxfm_16bpp.o \
    x86/vp9lpf.o \
    x86/vp9lpf_16bpp.o \
    x86/vp9mc.o \
    x86/vp9mc_16bpp.o \
    ../mozva/mozva.o
SHARED_LIBS += ../libavutil/libmozavutil.so
OS_LIBS += -lm
COMPUTED_CFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DHAVE_AV_CONFIG_H -I/worker/build/media/ffvpx/libavcodec -I/worker/build/obj-x86_64-pc-linux-gnu/media/ffvpx/libavcodec -I/worker/build/media/ffvpx -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -DMOZILLA_CLIENT -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi -Wno-parentheses -Wno-pointer-sign -Wno-sign-compare -Wno-switch -Wno-type-limits -Wno-unused-function -Wno-deprecated-declarations -Wno-absolute-value -Wno-incompatible-pointer-types -Wno-string-conversion -Wno-visibility -include libavutil_visibility.h -fexperimental-new-pass-manager
COMPUTED_CXXFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/stl_wrappers -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DHAVE_AV_CONFIG_H -I/worker/build/media/ffvpx/libavcodec -I/worker/build/obj-x86_64-pc-linux-gnu/media/ffvpx/libavcodec -I/worker/build/media/ffvpx -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -DMOZILLA_CLIENT -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -fexperimental-new-pass-manager
COMPUTED_CXX_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables
COMPUTED_C_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi
