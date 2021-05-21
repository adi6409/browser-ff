# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1 -DMOZ_APP_NAME=konke '-DMOZ_MACBUNDLE_NAME=Konke Browser.app' -DHAVE_SHELL_SERVICE=1
DIST_SUBDIR = browser
FINAL_TARGET = $(if $(XPI_NAME),$(DIST)/xpi-stage/$(XPI_NAME),$(DIST)/bin)$(DIST_SUBDIR:%=/%)
JAR_MANIFEST := /worker/build/browser/components/preferences/dialogs/jar.mn
