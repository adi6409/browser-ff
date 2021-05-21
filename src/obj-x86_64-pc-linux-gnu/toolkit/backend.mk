# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1
DIRS := actors components content crashreporter locales modules mozapps/downloads mozapps/extensions mozapps/preferences profile themes mozapps/update mozapps/update/common xre mozapps/handling system/unixproxy
misc::
	$(call py_action,buildlist,$(DEPTH)/dist/bin/chrome.manifest 'manifest components/l10n-registry.manifest')
