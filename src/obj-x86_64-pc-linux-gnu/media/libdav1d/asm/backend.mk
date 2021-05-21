# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1 -DDAV1D_API= -DSTACK_ALIGNMENT=16
include $(topsrcdir)/config/AB_rCD.mk
PRE_COMPILE_TARGETS += $(MDDEPDIR)/16bd_cdef_init_tmpl.c.stub
16bd_cdef_init_tmpl.c: $(MDDEPDIR)/16bd_cdef_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/16bd_cdef_init_tmpl.c.pp
$(MDDEPDIR)/16bd_cdef_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/x86/cdef_init_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 16bd_cdef_init_tmpl.c $(MDDEPDIR)/16bd_cdef_init_tmpl.c.pp $(MDDEPDIR)/16bd_cdef_init_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/x86/cdef_init_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/8bd_cdef_init_tmpl.c.stub
8bd_cdef_init_tmpl.c: $(MDDEPDIR)/8bd_cdef_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/8bd_cdef_init_tmpl.c.pp
$(MDDEPDIR)/8bd_cdef_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/x86/cdef_init_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 8bd_cdef_init_tmpl.c $(MDDEPDIR)/8bd_cdef_init_tmpl.c.pp $(MDDEPDIR)/8bd_cdef_init_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/x86/cdef_init_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/16bd_film_grain_init_tmpl.c.stub
16bd_film_grain_init_tmpl.c: $(MDDEPDIR)/16bd_film_grain_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/16bd_film_grain_init_tmpl.c.pp
$(MDDEPDIR)/16bd_film_grain_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/x86/film_grain_init_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 16bd_film_grain_init_tmpl.c $(MDDEPDIR)/16bd_film_grain_init_tmpl.c.pp $(MDDEPDIR)/16bd_film_grain_init_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/x86/film_grain_init_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/8bd_film_grain_init_tmpl.c.stub
8bd_film_grain_init_tmpl.c: $(MDDEPDIR)/8bd_film_grain_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/8bd_film_grain_init_tmpl.c.pp
$(MDDEPDIR)/8bd_film_grain_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/x86/film_grain_init_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 8bd_film_grain_init_tmpl.c $(MDDEPDIR)/8bd_film_grain_init_tmpl.c.pp $(MDDEPDIR)/8bd_film_grain_init_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/x86/film_grain_init_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/16bd_ipred_init_tmpl.c.stub
16bd_ipred_init_tmpl.c: $(MDDEPDIR)/16bd_ipred_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/16bd_ipred_init_tmpl.c.pp
$(MDDEPDIR)/16bd_ipred_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/x86/ipred_init_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 16bd_ipred_init_tmpl.c $(MDDEPDIR)/16bd_ipred_init_tmpl.c.pp $(MDDEPDIR)/16bd_ipred_init_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/x86/ipred_init_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/8bd_ipred_init_tmpl.c.stub
8bd_ipred_init_tmpl.c: $(MDDEPDIR)/8bd_ipred_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/8bd_ipred_init_tmpl.c.pp
$(MDDEPDIR)/8bd_ipred_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/x86/ipred_init_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 8bd_ipred_init_tmpl.c $(MDDEPDIR)/8bd_ipred_init_tmpl.c.pp $(MDDEPDIR)/8bd_ipred_init_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/x86/ipred_init_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/16bd_itx_init_tmpl.c.stub
16bd_itx_init_tmpl.c: $(MDDEPDIR)/16bd_itx_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/16bd_itx_init_tmpl.c.pp
$(MDDEPDIR)/16bd_itx_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/x86/itx_init_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 16bd_itx_init_tmpl.c $(MDDEPDIR)/16bd_itx_init_tmpl.c.pp $(MDDEPDIR)/16bd_itx_init_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/x86/itx_init_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/8bd_itx_init_tmpl.c.stub
8bd_itx_init_tmpl.c: $(MDDEPDIR)/8bd_itx_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/8bd_itx_init_tmpl.c.pp
$(MDDEPDIR)/8bd_itx_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/x86/itx_init_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 8bd_itx_init_tmpl.c $(MDDEPDIR)/8bd_itx_init_tmpl.c.pp $(MDDEPDIR)/8bd_itx_init_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/x86/itx_init_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/16bd_loopfilter_init_tmpl.c.stub
16bd_loopfilter_init_tmpl.c: $(MDDEPDIR)/16bd_loopfilter_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/16bd_loopfilter_init_tmpl.c.pp
$(MDDEPDIR)/16bd_loopfilter_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/x86/loopfilter_init_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 16bd_loopfilter_init_tmpl.c $(MDDEPDIR)/16bd_loopfilter_init_tmpl.c.pp $(MDDEPDIR)/16bd_loopfilter_init_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/x86/loopfilter_init_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/8bd_loopfilter_init_tmpl.c.stub
8bd_loopfilter_init_tmpl.c: $(MDDEPDIR)/8bd_loopfilter_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/8bd_loopfilter_init_tmpl.c.pp
$(MDDEPDIR)/8bd_loopfilter_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/x86/loopfilter_init_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 8bd_loopfilter_init_tmpl.c $(MDDEPDIR)/8bd_loopfilter_init_tmpl.c.pp $(MDDEPDIR)/8bd_loopfilter_init_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/x86/loopfilter_init_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/16bd_looprestoration_init_tmpl.c.stub
16bd_looprestoration_init_tmpl.c: $(MDDEPDIR)/16bd_looprestoration_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/16bd_looprestoration_init_tmpl.c.pp
$(MDDEPDIR)/16bd_looprestoration_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/x86/looprestoration_init_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 16bd_looprestoration_init_tmpl.c $(MDDEPDIR)/16bd_looprestoration_init_tmpl.c.pp $(MDDEPDIR)/16bd_looprestoration_init_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/x86/looprestoration_init_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/8bd_looprestoration_init_tmpl.c.stub
8bd_looprestoration_init_tmpl.c: $(MDDEPDIR)/8bd_looprestoration_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/8bd_looprestoration_init_tmpl.c.pp
$(MDDEPDIR)/8bd_looprestoration_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/x86/looprestoration_init_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 8bd_looprestoration_init_tmpl.c $(MDDEPDIR)/8bd_looprestoration_init_tmpl.c.pp $(MDDEPDIR)/8bd_looprestoration_init_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/x86/looprestoration_init_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/16bd_mc_init_tmpl.c.stub
16bd_mc_init_tmpl.c: $(MDDEPDIR)/16bd_mc_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/16bd_mc_init_tmpl.c.pp
$(MDDEPDIR)/16bd_mc_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/x86/mc_init_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 16bd_mc_init_tmpl.c $(MDDEPDIR)/16bd_mc_init_tmpl.c.pp $(MDDEPDIR)/16bd_mc_init_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/x86/mc_init_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/8bd_mc_init_tmpl.c.stub
8bd_mc_init_tmpl.c: $(MDDEPDIR)/8bd_mc_init_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/8bd_mc_init_tmpl.c.pp
$(MDDEPDIR)/8bd_mc_init_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/x86/mc_init_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 8bd_mc_init_tmpl.c $(MDDEPDIR)/8bd_mc_init_tmpl.c.pp $(MDDEPDIR)/8bd_mc_init_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/x86/mc_init_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

LOCAL_INCLUDES += -I$(topsrcdir)/third_party/dav1d
LOCAL_INCLUDES += -I$(topsrcdir)/third_party/dav1d/include
LOCAL_INCLUDES += -I$(topsrcdir)/third_party/dav1d/src
CSRCS += $(topsrcdir)/third_party/dav1d/src/x86/cpu.c
CSRCS += $(topsrcdir)/third_party/dav1d/src/x86/msac_init.c
ASFILES += $(topsrcdir)/third_party/dav1d/src/x86/cdef_avx2.asm
ASFILES += $(topsrcdir)/third_party/dav1d/src/x86/cdef_avx512.asm
ASFILES += $(topsrcdir)/third_party/dav1d/src/x86/cdef_sse.asm
ASFILES += $(topsrcdir)/third_party/dav1d/src/x86/cpuid.asm
ASFILES += $(topsrcdir)/third_party/dav1d/src/x86/film_grain.asm
ASFILES += $(topsrcdir)/third_party/dav1d/src/x86/film_grain_ssse3.asm
ASFILES += $(topsrcdir)/third_party/dav1d/src/x86/ipred.asm
ASFILES += $(topsrcdir)/third_party/dav1d/src/x86/ipred_ssse3.asm
ASFILES += $(topsrcdir)/third_party/dav1d/src/x86/itx.asm
ASFILES += $(topsrcdir)/third_party/dav1d/src/x86/itx_ssse3.asm
ASFILES += $(topsrcdir)/third_party/dav1d/src/x86/loopfilter.asm
ASFILES += $(topsrcdir)/third_party/dav1d/src/x86/loopfilter_ssse3.asm
ASFILES += $(topsrcdir)/third_party/dav1d/src/x86/looprestoration.asm
ASFILES += $(topsrcdir)/third_party/dav1d/src/x86/looprestoration_sse.asm
ASFILES += $(topsrcdir)/third_party/dav1d/src/x86/mc_avx2.asm
ASFILES += $(topsrcdir)/third_party/dav1d/src/x86/mc_avx512.asm
ASFILES += $(topsrcdir)/third_party/dav1d/src/x86/mc_sse.asm
ASFILES += $(topsrcdir)/third_party/dav1d/src/x86/msac.asm
CSRCS += 16bd_cdef_init_tmpl.c
CSRCS += 16bd_film_grain_init_tmpl.c
CSRCS += 16bd_ipred_init_tmpl.c
CSRCS += 16bd_itx_init_tmpl.c
CSRCS += 16bd_loopfilter_init_tmpl.c
CSRCS += 16bd_looprestoration_init_tmpl.c
CSRCS += 16bd_mc_init_tmpl.c
CSRCS += 8bd_cdef_init_tmpl.c
CSRCS += 8bd_film_grain_init_tmpl.c
CSRCS += 8bd_ipred_init_tmpl.c
CSRCS += 8bd_itx_init_tmpl.c
CSRCS += 8bd_loopfilter_init_tmpl.c
CSRCS += 8bd_looprestoration_init_tmpl.c
CSRCS += 8bd_mc_init_tmpl.c
AS := /usr/sbin/nasm
ASOUTOPTION := -o 
AS_DASH_C_FLAG := 
COMPUTED_LDFLAGS += -lpthread -Wl,-z,noexecstack -Wl,-z,text -Wl,-z,relro -Wl,-z,nocopyreloc -Wl,-Bsymbolic-functions -Wl,--build-id=sha1 -fstack-protector-strong -fstack-clash-protection -Wl,-rpath-link,/worker/build/obj-x86_64-pc-linux-gnu/dist/bin -Wl,-rpath-link,/usr/local/lib
LIBRARY_NAME := media_libdav1d_asm
FORCE_STATIC_LIB := 1
REAL_LIBRARY := libmedia_libdav1d_asm.a
DEFINES += -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API
COMPUTED_CFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DDAV1D_API= -DSTACK_ALIGNMENT=16 -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API -I/worker/build/media/libdav1d/asm -I/worker/build/obj-x86_64-pc-linux-gnu/media/libdav1d/asm -I/worker/build/third_party/dav1d -I/worker/build/third_party/dav1d/include -I/worker/build/third_party/dav1d/src -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -DMOZILLA_CLIENT -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/dav1d/ -fexperimental-new-pass-manager
COMPUTED_CXXFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/stl_wrappers -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DDAV1D_API= -DSTACK_ALIGNMENT=16 -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API -I/worker/build/media/libdav1d/asm -I/worker/build/obj-x86_64-pc-linux-gnu/media/libdav1d/asm -I/worker/build/third_party/dav1d -I/worker/build/third_party/dav1d/include -I/worker/build/third_party/dav1d/src -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -DMOZILLA_CLIENT -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -fexperimental-new-pass-manager
COMPUTED_CXX_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables
COMPUTED_C_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi
COMPUTED_ASFLAGS += -f elf64 -F dwarf -I/worker/build/third_party/dav1d/src/ -DHAVE_AVX512ICL=1 -I/worker/build/media/libdav1d/asm/x86_64/linux/ -Dprivate_prefix=dav1d
COMPUTED_SFLAGS += -DNDEBUG=1 -DTRIMMED=1 -DDAV1D_API= -DSTACK_ALIGNMENT=16 -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API -f elf64 -F dwarf -I/worker/build/third_party/dav1d -I/worker/build/third_party/dav1d/include -I/worker/build/third_party/dav1d/src -I/worker/build/third_party/dav1d/src/ -DHAVE_AVX512ICL=1 -I/worker/build/media/libdav1d/asm/x86_64/linux/ -Dprivate_prefix=dav1d
