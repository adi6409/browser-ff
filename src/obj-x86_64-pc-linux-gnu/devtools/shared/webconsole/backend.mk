# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1
DIST_SUBDIR = browser
FINAL_TARGET = $(if $(XPI_NAME),$(DIST)/xpi-stage/$(XPI_NAME),$(DIST)/bin)$(DIST_SUBDIR:%=/%)
dist_bin_browser_chrome_devtools_modules_devtools_shared_webconsole_FILES += reserved-js-words.js
dist_bin_browser_chrome_devtools_modules_devtools_shared_webconsole_DEST := $(DEPTH)/dist/bin/browser/chrome/devtools/modules/devtools/shared/webconsole
dist_bin_browser_chrome_devtools_modules_devtools_shared_webconsole_TARGET := misc
INSTALL_TARGETS += dist_bin_browser_chrome_devtools_modules_devtools_shared_webconsole
