# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1 -DACCEPTED_MAR_CHANNEL_IDS= -Dtopsrcdir=/worker/build -Dtopobjdir=/worker/build/obj-x86_64-pc-linux-gnu
DIRS := unix
dist_bin_FILES += application.ini
dist_bin_DEST := $(DEPTH)/dist/bin/
dist_bin_TARGET := misc
INSTALL_TARGETS += dist_bin
DIST_FILES_0 += $(srcdir)/update-settings.ini
DIST_FILES_0 += $(srcdir)/.lldbinit.in
DIST_FILES_0_PATH := $(DEPTH)/dist/bin/
DIST_FILES_0_TARGET := misc
PP_TARGETS += DIST_FILES_0
OBJDIR_0_FILES += $(DEPTH)/dist/bin/.lldbinit
OBJDIR_0_FILES += $(topsrcdir)/.ycm_extra_conf.py
OBJDIR_0_DEST := $(topobjdir)/
OBJDIR_0_TARGET := misc
INSTALL_TARGETS += OBJDIR_0
OBJDIR_1_FILES += $(srcdir)/.gdbinit.loader
OBJDIR_1_FILES += $(srcdir)/.gdbinit
OBJDIR_1_DEST := $(topobjdir)/build
OBJDIR_1_TARGET := misc
INSTALL_TARGETS += OBJDIR_1
OBJDIR_PP_FILES_0 += $(srcdir)/.gdbinit.py.in
OBJDIR_PP_FILES_0_PATH := $(DEPTH)/build
OBJDIR_PP_FILES_0_TARGET := misc
PP_TARGETS += OBJDIR_PP_FILES_0
