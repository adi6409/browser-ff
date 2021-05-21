# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1
DIST_SUBDIR = browser
FINAL_TARGET = $(if $(XPI_NAME),$(DIST)/xpi-stage/$(XPI_NAME),$(DIST)/bin)$(DIST_SUBDIR:%=/%)
DIRS := acorn css compatibility discovery heapsnapshot inspector jsbeautify layout locales node-properties performance performance-new platform protocol qrcode resources security sprintfjs specs storage transport webconsole worker
JAR_MANIFEST := /worker/build/devtools/shared/jar.mn
