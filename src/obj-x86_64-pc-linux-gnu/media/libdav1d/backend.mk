# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1 -DDAV1D_API= -DSTACK_ALIGNMENT=16
DIRS := asm
include $(topsrcdir)/config/AB_rCD.mk
PRE_COMPILE_TARGETS += $(MDDEPDIR)/16bd_cdef_apply_tmpl.c.stub
16bd_cdef_apply_tmpl.c: $(MDDEPDIR)/16bd_cdef_apply_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/16bd_cdef_apply_tmpl.c.pp
$(MDDEPDIR)/16bd_cdef_apply_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/cdef_apply_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 16bd_cdef_apply_tmpl.c $(MDDEPDIR)/16bd_cdef_apply_tmpl.c.pp $(MDDEPDIR)/16bd_cdef_apply_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/cdef_apply_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/8bd_cdef_apply_tmpl.c.stub
8bd_cdef_apply_tmpl.c: $(MDDEPDIR)/8bd_cdef_apply_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/8bd_cdef_apply_tmpl.c.pp
$(MDDEPDIR)/8bd_cdef_apply_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/cdef_apply_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 8bd_cdef_apply_tmpl.c $(MDDEPDIR)/8bd_cdef_apply_tmpl.c.pp $(MDDEPDIR)/8bd_cdef_apply_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/cdef_apply_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/16bd_cdef_tmpl.c.stub
16bd_cdef_tmpl.c: $(MDDEPDIR)/16bd_cdef_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/16bd_cdef_tmpl.c.pp
$(MDDEPDIR)/16bd_cdef_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/cdef_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 16bd_cdef_tmpl.c $(MDDEPDIR)/16bd_cdef_tmpl.c.pp $(MDDEPDIR)/16bd_cdef_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/cdef_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/8bd_cdef_tmpl.c.stub
8bd_cdef_tmpl.c: $(MDDEPDIR)/8bd_cdef_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/8bd_cdef_tmpl.c.pp
$(MDDEPDIR)/8bd_cdef_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/cdef_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 8bd_cdef_tmpl.c $(MDDEPDIR)/8bd_cdef_tmpl.c.pp $(MDDEPDIR)/8bd_cdef_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/cdef_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/16bd_fg_apply_tmpl.c.stub
16bd_fg_apply_tmpl.c: $(MDDEPDIR)/16bd_fg_apply_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/16bd_fg_apply_tmpl.c.pp
$(MDDEPDIR)/16bd_fg_apply_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/fg_apply_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 16bd_fg_apply_tmpl.c $(MDDEPDIR)/16bd_fg_apply_tmpl.c.pp $(MDDEPDIR)/16bd_fg_apply_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/fg_apply_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/8bd_fg_apply_tmpl.c.stub
8bd_fg_apply_tmpl.c: $(MDDEPDIR)/8bd_fg_apply_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/8bd_fg_apply_tmpl.c.pp
$(MDDEPDIR)/8bd_fg_apply_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/fg_apply_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 8bd_fg_apply_tmpl.c $(MDDEPDIR)/8bd_fg_apply_tmpl.c.pp $(MDDEPDIR)/8bd_fg_apply_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/fg_apply_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/16bd_film_grain_tmpl.c.stub
16bd_film_grain_tmpl.c: $(MDDEPDIR)/16bd_film_grain_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/16bd_film_grain_tmpl.c.pp
$(MDDEPDIR)/16bd_film_grain_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/film_grain_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 16bd_film_grain_tmpl.c $(MDDEPDIR)/16bd_film_grain_tmpl.c.pp $(MDDEPDIR)/16bd_film_grain_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/film_grain_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/8bd_film_grain_tmpl.c.stub
8bd_film_grain_tmpl.c: $(MDDEPDIR)/8bd_film_grain_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/8bd_film_grain_tmpl.c.pp
$(MDDEPDIR)/8bd_film_grain_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/film_grain_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 8bd_film_grain_tmpl.c $(MDDEPDIR)/8bd_film_grain_tmpl.c.pp $(MDDEPDIR)/8bd_film_grain_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/film_grain_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/16bd_ipred_prepare_tmpl.c.stub
16bd_ipred_prepare_tmpl.c: $(MDDEPDIR)/16bd_ipred_prepare_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/16bd_ipred_prepare_tmpl.c.pp
$(MDDEPDIR)/16bd_ipred_prepare_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/ipred_prepare_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 16bd_ipred_prepare_tmpl.c $(MDDEPDIR)/16bd_ipred_prepare_tmpl.c.pp $(MDDEPDIR)/16bd_ipred_prepare_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/ipred_prepare_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/8bd_ipred_prepare_tmpl.c.stub
8bd_ipred_prepare_tmpl.c: $(MDDEPDIR)/8bd_ipred_prepare_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/8bd_ipred_prepare_tmpl.c.pp
$(MDDEPDIR)/8bd_ipred_prepare_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/ipred_prepare_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 8bd_ipred_prepare_tmpl.c $(MDDEPDIR)/8bd_ipred_prepare_tmpl.c.pp $(MDDEPDIR)/8bd_ipred_prepare_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/ipred_prepare_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/16bd_ipred_tmpl.c.stub
16bd_ipred_tmpl.c: $(MDDEPDIR)/16bd_ipred_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/16bd_ipred_tmpl.c.pp
$(MDDEPDIR)/16bd_ipred_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/ipred_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 16bd_ipred_tmpl.c $(MDDEPDIR)/16bd_ipred_tmpl.c.pp $(MDDEPDIR)/16bd_ipred_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/ipred_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/8bd_ipred_tmpl.c.stub
8bd_ipred_tmpl.c: $(MDDEPDIR)/8bd_ipred_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/8bd_ipred_tmpl.c.pp
$(MDDEPDIR)/8bd_ipred_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/ipred_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 8bd_ipred_tmpl.c $(MDDEPDIR)/8bd_ipred_tmpl.c.pp $(MDDEPDIR)/8bd_ipred_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/ipred_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/16bd_itx_tmpl.c.stub
16bd_itx_tmpl.c: $(MDDEPDIR)/16bd_itx_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/16bd_itx_tmpl.c.pp
$(MDDEPDIR)/16bd_itx_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/itx_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 16bd_itx_tmpl.c $(MDDEPDIR)/16bd_itx_tmpl.c.pp $(MDDEPDIR)/16bd_itx_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/itx_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/8bd_itx_tmpl.c.stub
8bd_itx_tmpl.c: $(MDDEPDIR)/8bd_itx_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/8bd_itx_tmpl.c.pp
$(MDDEPDIR)/8bd_itx_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/itx_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 8bd_itx_tmpl.c $(MDDEPDIR)/8bd_itx_tmpl.c.pp $(MDDEPDIR)/8bd_itx_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/itx_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/16bd_lf_apply_tmpl.c.stub
16bd_lf_apply_tmpl.c: $(MDDEPDIR)/16bd_lf_apply_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/16bd_lf_apply_tmpl.c.pp
$(MDDEPDIR)/16bd_lf_apply_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/lf_apply_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 16bd_lf_apply_tmpl.c $(MDDEPDIR)/16bd_lf_apply_tmpl.c.pp $(MDDEPDIR)/16bd_lf_apply_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/lf_apply_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/8bd_lf_apply_tmpl.c.stub
8bd_lf_apply_tmpl.c: $(MDDEPDIR)/8bd_lf_apply_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/8bd_lf_apply_tmpl.c.pp
$(MDDEPDIR)/8bd_lf_apply_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/lf_apply_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 8bd_lf_apply_tmpl.c $(MDDEPDIR)/8bd_lf_apply_tmpl.c.pp $(MDDEPDIR)/8bd_lf_apply_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/lf_apply_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/16bd_loopfilter_tmpl.c.stub
16bd_loopfilter_tmpl.c: $(MDDEPDIR)/16bd_loopfilter_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/16bd_loopfilter_tmpl.c.pp
$(MDDEPDIR)/16bd_loopfilter_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/loopfilter_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 16bd_loopfilter_tmpl.c $(MDDEPDIR)/16bd_loopfilter_tmpl.c.pp $(MDDEPDIR)/16bd_loopfilter_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/loopfilter_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/8bd_loopfilter_tmpl.c.stub
8bd_loopfilter_tmpl.c: $(MDDEPDIR)/8bd_loopfilter_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/8bd_loopfilter_tmpl.c.pp
$(MDDEPDIR)/8bd_loopfilter_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/loopfilter_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 8bd_loopfilter_tmpl.c $(MDDEPDIR)/8bd_loopfilter_tmpl.c.pp $(MDDEPDIR)/8bd_loopfilter_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/loopfilter_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/16bd_looprestoration_tmpl.c.stub
16bd_looprestoration_tmpl.c: $(MDDEPDIR)/16bd_looprestoration_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/16bd_looprestoration_tmpl.c.pp
$(MDDEPDIR)/16bd_looprestoration_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/looprestoration_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 16bd_looprestoration_tmpl.c $(MDDEPDIR)/16bd_looprestoration_tmpl.c.pp $(MDDEPDIR)/16bd_looprestoration_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/looprestoration_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/8bd_looprestoration_tmpl.c.stub
8bd_looprestoration_tmpl.c: $(MDDEPDIR)/8bd_looprestoration_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/8bd_looprestoration_tmpl.c.pp
$(MDDEPDIR)/8bd_looprestoration_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/looprestoration_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 8bd_looprestoration_tmpl.c $(MDDEPDIR)/8bd_looprestoration_tmpl.c.pp $(MDDEPDIR)/8bd_looprestoration_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/looprestoration_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/16bd_lr_apply_tmpl.c.stub
16bd_lr_apply_tmpl.c: $(MDDEPDIR)/16bd_lr_apply_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/16bd_lr_apply_tmpl.c.pp
$(MDDEPDIR)/16bd_lr_apply_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/lr_apply_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 16bd_lr_apply_tmpl.c $(MDDEPDIR)/16bd_lr_apply_tmpl.c.pp $(MDDEPDIR)/16bd_lr_apply_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/lr_apply_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/8bd_lr_apply_tmpl.c.stub
8bd_lr_apply_tmpl.c: $(MDDEPDIR)/8bd_lr_apply_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/8bd_lr_apply_tmpl.c.pp
$(MDDEPDIR)/8bd_lr_apply_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/lr_apply_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 8bd_lr_apply_tmpl.c $(MDDEPDIR)/8bd_lr_apply_tmpl.c.pp $(MDDEPDIR)/8bd_lr_apply_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/lr_apply_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/16bd_mc_tmpl.c.stub
16bd_mc_tmpl.c: $(MDDEPDIR)/16bd_mc_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/16bd_mc_tmpl.c.pp
$(MDDEPDIR)/16bd_mc_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/mc_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 16bd_mc_tmpl.c $(MDDEPDIR)/16bd_mc_tmpl.c.pp $(MDDEPDIR)/16bd_mc_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/mc_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/8bd_mc_tmpl.c.stub
8bd_mc_tmpl.c: $(MDDEPDIR)/8bd_mc_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/8bd_mc_tmpl.c.pp
$(MDDEPDIR)/8bd_mc_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/mc_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 8bd_mc_tmpl.c $(MDDEPDIR)/8bd_mc_tmpl.c.pp $(MDDEPDIR)/8bd_mc_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/mc_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/16bd_recon_tmpl.c.stub
16bd_recon_tmpl.c: $(MDDEPDIR)/16bd_recon_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/16bd_recon_tmpl.c.pp
$(MDDEPDIR)/16bd_recon_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/recon_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 16bd_recon_tmpl.c $(MDDEPDIR)/16bd_recon_tmpl.c.pp $(MDDEPDIR)/16bd_recon_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/recon_tmpl.c BITDEPTH 16)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/8bd_recon_tmpl.c.stub
8bd_recon_tmpl.c: $(MDDEPDIR)/8bd_recon_tmpl.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/8bd_recon_tmpl.c.pp
$(MDDEPDIR)/8bd_recon_tmpl.c.stub: /worker/build/media/libdav1d/generate_source.py $(topsrcdir)/third_party/dav1d/src/recon_tmpl.c backend.mk
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/media/libdav1d/generate_source.py add_define 8bd_recon_tmpl.c $(MDDEPDIR)/8bd_recon_tmpl.c.pp $(MDDEPDIR)/8bd_recon_tmpl.c.stub $(topsrcdir)/third_party/dav1d/src/recon_tmpl.c BITDEPTH 8)
	@$(TOUCH) $@

LOCAL_INCLUDES += -I$(topsrcdir)/third_party/dav1d
LOCAL_INCLUDES += -I$(topsrcdir)/third_party/dav1d/include
LOCAL_INCLUDES += -I$(topsrcdir)/third_party/dav1d/include/dav1d
LOCAL_INCLUDES += -I$(topsrcdir)/third_party/dav1d/src
CSRCS += $(topsrcdir)/third_party/dav1d/src/cdf.c
CSRCS += $(topsrcdir)/third_party/dav1d/src/cpu.c
CSRCS += $(topsrcdir)/third_party/dav1d/src/data.c
CSRCS += $(topsrcdir)/third_party/dav1d/src/decode.c
CSRCS += $(topsrcdir)/third_party/dav1d/src/dequant_tables.c
CSRCS += $(topsrcdir)/third_party/dav1d/src/getbits.c
CSRCS += $(topsrcdir)/third_party/dav1d/src/intra_edge.c
CSRCS += $(topsrcdir)/third_party/dav1d/src/itx_1d.c
CSRCS += $(topsrcdir)/third_party/dav1d/src/lf_mask.c
CSRCS += $(topsrcdir)/third_party/dav1d/src/lib.c
CSRCS += $(topsrcdir)/third_party/dav1d/src/log.c
CSRCS += $(topsrcdir)/third_party/dav1d/src/mem.c
CSRCS += $(topsrcdir)/third_party/dav1d/src/msac.c
CSRCS += $(topsrcdir)/third_party/dav1d/src/obu.c
CSRCS += $(topsrcdir)/third_party/dav1d/src/picture.c
CSRCS += $(topsrcdir)/third_party/dav1d/src/qm.c
CSRCS += $(topsrcdir)/third_party/dav1d/src/ref.c
CSRCS += $(topsrcdir)/third_party/dav1d/src/refmvs.c
CSRCS += $(topsrcdir)/third_party/dav1d/src/scan.c
CSRCS += $(topsrcdir)/third_party/dav1d/src/tables.c
CSRCS += $(topsrcdir)/third_party/dav1d/src/thread_task.c
CSRCS += $(topsrcdir)/third_party/dav1d/src/warpmv.c
CSRCS += $(topsrcdir)/third_party/dav1d/src/wedge.c
CSRCS += 16bd_cdef_apply_tmpl.c
CSRCS += 16bd_cdef_tmpl.c
CSRCS += 16bd_fg_apply_tmpl.c
CSRCS += 16bd_film_grain_tmpl.c
CSRCS += 16bd_ipred_prepare_tmpl.c
CSRCS += 16bd_ipred_tmpl.c
CSRCS += 16bd_itx_tmpl.c
CSRCS += 16bd_lf_apply_tmpl.c
CSRCS += 16bd_loopfilter_tmpl.c
CSRCS += 16bd_looprestoration_tmpl.c
CSRCS += 16bd_lr_apply_tmpl.c
CSRCS += 16bd_mc_tmpl.c
CSRCS += 16bd_recon_tmpl.c
CSRCS += 8bd_cdef_apply_tmpl.c
CSRCS += 8bd_cdef_tmpl.c
CSRCS += 8bd_fg_apply_tmpl.c
CSRCS += 8bd_film_grain_tmpl.c
CSRCS += 8bd_ipred_prepare_tmpl.c
CSRCS += 8bd_ipred_tmpl.c
CSRCS += 8bd_itx_tmpl.c
CSRCS += 8bd_lf_apply_tmpl.c
CSRCS += 8bd_loopfilter_tmpl.c
CSRCS += 8bd_looprestoration_tmpl.c
CSRCS += 8bd_lr_apply_tmpl.c
CSRCS += 8bd_mc_tmpl.c
CSRCS += 8bd_recon_tmpl.c
COMPUTED_LDFLAGS += -lpthread -Wl,-z,noexecstack -Wl,-z,text -Wl,-z,relro -Wl,-z,nocopyreloc -Wl,-Bsymbolic-functions -Wl,--build-id=sha1 -fstack-protector-strong -fstack-clash-protection -Wl,-rpath-link,/worker/build/obj-x86_64-pc-linux-gnu/dist/bin -Wl,-rpath-link,/usr/local/lib
LIBRARY_NAME := dav1d
FORCE_STATIC_LIB := 1
REAL_LIBRARY := libdav1d.a
DEFINES += -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API
COMPUTED_CFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DDAV1D_API= -DSTACK_ALIGNMENT=16 -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API -I/worker/build/media/libdav1d -I/worker/build/obj-x86_64-pc-linux-gnu/media/libdav1d -I/worker/build/third_party/dav1d -I/worker/build/third_party/dav1d/include -I/worker/build/third_party/dav1d/include/dav1d -I/worker/build/third_party/dav1d/src -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -DMOZILLA_CLIENT -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi -fexperimental-new-pass-manager
COMPUTED_CXXFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/stl_wrappers -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DDAV1D_API= -DSTACK_ALIGNMENT=16 -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API -I/worker/build/media/libdav1d -I/worker/build/obj-x86_64-pc-linux-gnu/media/libdav1d -I/worker/build/third_party/dav1d -I/worker/build/third_party/dav1d/include -I/worker/build/third_party/dav1d/include/dav1d -I/worker/build/third_party/dav1d/src -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -DMOZILLA_CLIENT -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -fexperimental-new-pass-manager
COMPUTED_CXX_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables
COMPUTED_C_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi
