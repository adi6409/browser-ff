# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1
DIST_SUBDIR = browser
FINAL_TARGET = $(if $(XPI_NAME),$(DIST)/xpi-stage/$(XPI_NAME),$(DIST)/bin)$(DIST_SUBDIR:%=/%)
include $(topsrcdir)/config/AB_rCD.mk
MISC_TARGETS += $(MDDEPDIR)/updater.ini.stub
updater.ini: $(MDDEPDIR)/updater.ini.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/updater.ini.pp
$(MDDEPDIR)/updater.ini.stub: /worker/build/browser/locales/generate_ini.py $(call MERGE_FILE,updater/updater.ini) $(topsrcdir)/browser/installer/windows/nsis/updater_append.ini $(if $(IS_LANGUAGE_REPACK),FORCE)
	$(REPORT_BUILD)
	$(call py_action,file_generate,--locale=$(AB_CD) /worker/build/browser/locales/generate_ini.py main updater.ini $(MDDEPDIR)/updater.ini.pp $(MDDEPDIR)/updater.ini.stub $(call MERGE_FILE,updater/updater.ini) $(topsrcdir)/browser/installer/windows/nsis/updater_append.ini)
	@$(TOUCH) $@

MISC_TARGETS += $(MDDEPDIR)/bookmarks.html.stub
bookmarks.html: $(MDDEPDIR)/bookmarks.html.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/bookmarks.html.pp
$(MDDEPDIR)/bookmarks.html.stub: /worker/build/browser/locales/generate_bookmarks.py $(srcdir)/generic/profile/bookmarks.html.in $(call MERGE_FILE,profile/bookmarks.inc) $(if $(IS_LANGUAGE_REPACK),FORCE)
	$(REPORT_BUILD)
	$(call py_action,file_generate,--locale=$(AB_CD) /worker/build/browser/locales/generate_bookmarks.py main bookmarks.html $(MDDEPDIR)/bookmarks.html.pp $(MDDEPDIR)/bookmarks.html.stub $(srcdir)/generic/profile/bookmarks.html.in $(call MERGE_FILE,profile/bookmarks.inc))
	@$(TOUCH) $@

LOCALIZED_FILES_0_FILES += $(call MERGE_FILE,crashreporter/crashreporter-override.ini)
LOCALIZED_FILES_0_DEST = $(FINAL_TARGET)/
LOCALIZED_FILES_0_TARGET := misc
INSTALL_TARGETS += LOCALIZED_FILES_0
LOCALIZED_FILES_1_FILES += updater.ini
LOCALIZED_FILES_1_DEST = $(FINAL_TARGET)/..
LOCALIZED_FILES_1_TARGET := misc
INSTALL_TARGETS += LOCALIZED_FILES_1
LOCALIZED_PP_FILES_0 += $(call MERGE_FILE,firefox-l10n.js)
LOCALIZED_PP_FILES_0_PATH = $(FINAL_TARGET)/defaults/preferences
LOCALIZED_PP_FILES_0_TARGET := misc
LOCALIZED_PP_FILES_0_FLAGS := --silence-missing-directive-warnings
PP_TARGETS += LOCALIZED_PP_FILES_0
JAR_MANIFEST := /worker/build/browser/locales/jar.mn
