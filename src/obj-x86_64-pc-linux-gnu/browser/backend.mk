# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1 -DAPP_VERSION=87.0 -DMOZ_WIDEVINE_EME
DIST_SUBDIR = browser
FINAL_TARGET = $(if $(XPI_NAME),$(DIST)/xpi-stage/$(XPI_NAME),$(DIST)/bin)$(DIST_SUBDIR:%=/%)
DIRS := actors base components fonts fxr locales modules themes extensions app tools/mozscreenshots
DIST_FILES_0 += $(srcdir)/app/profile/firefox.js
DIST_FILES_0_PATH := $(DEPTH)/dist/bin/browser/defaults/preferences
DIST_FILES_0_TARGET := misc
PP_TARGETS += DIST_FILES_0
misc::
	$(call py_action,buildlist,$(DEPTH)/dist/bin/browser/chrome.manifest 'manifest components/l10n-registry.manifest')
