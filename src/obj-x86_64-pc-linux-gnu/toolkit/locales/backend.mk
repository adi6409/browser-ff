# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1 -DMOZ_GTK
include $(topsrcdir)/config/AB_rCD.mk
MISC_TARGETS += $(MDDEPDIR)/update.locale.stub
update.locale: $(MDDEPDIR)/update.locale.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/update.locale.pp
$(MDDEPDIR)/update.locale.stub: /worker/build/toolkit/locales/generate_update_locale.py $(if $(IS_LANGUAGE_REPACK),FORCE)
	$(REPORT_BUILD)
	$(call py_action,file_generate,--locale=$(AB_CD) /worker/build/toolkit/locales/generate_update_locale.py main update.locale $(MDDEPDIR)/update.locale.pp $(MDDEPDIR)/update.locale.stub)
	@$(TOUCH) $@

MISC_TARGETS += $(MDDEPDIR)/locale.ini.stub
locale.ini: $(MDDEPDIR)/locale.ini.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/locale.ini.pp
$(MDDEPDIR)/locale.ini.stub: /worker/build/toolkit/locales/generate_locale_ini.py $(if $(IS_LANGUAGE_REPACK),FORCE)
	$(REPORT_BUILD)
	$(call py_action,file_generate,--locale=$(AB_CD) /worker/build/toolkit/locales/generate_locale_ini.py main locale.ini $(MDDEPDIR)/locale.ini.pp $(MDDEPDIR)/locale.ini.stub)
	@$(TOUCH) $@

dist_bin_res_FILES += multilocale.txt
dist_bin_res_DEST := $(DEPTH)/dist/bin/res
dist_bin_res_TARGET := libs
INSTALL_TARGETS += dist_bin_res
LOCALIZED_FILES_0_FILES += $(call MERGE_FILE,crashreporter/crashreporter.ini)
LOCALIZED_FILES_0_FILES += update.locale
LOCALIZED_FILES_0_FILES += locale.ini
LOCALIZED_FILES_0_DEST = $(FINAL_TARGET)/
LOCALIZED_FILES_0_TARGET := misc
INSTALL_TARGETS += LOCALIZED_FILES_0
JAR_MANIFEST := /worker/build/toolkit/locales/jar.mn
