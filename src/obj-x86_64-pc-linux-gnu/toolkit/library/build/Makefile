# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT MODIFY BY HAND.
DEPTH := ../../..
topobjdir := /worker/build/obj-x86_64-pc-linux-gnu
topsrcdir := /worker/build
srcdir := /worker/build/toolkit/library/build
srcdir_rel := ../../../../toolkit/library/build
relativesrcdir := toolkit/library/build
include $(DEPTH)/config/autoconf.mk
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this file,
# You can obtain one at http://mozilla.org/MPL/2.0/.

include $(topsrcdir)/config/config.mk

include $(topsrcdir)/config/rules.mk

DUMP_SYMBOLS_FLAGS = --count-ctors

ifdef CLANG_CL
ifdef MOZ_PROFILE_USE
ifeq (x86,$(CPU_ARCH))
PGO_LDFLAGS += -ORDER:@$(topsrcdir)/build/win32/orderfile.txt
endif
ifeq (x86_64,$(CPU_ARCH))
PGO_LDFLAGS += -ORDER:@$(topsrcdir)/build/win64/orderfile.txt
endif
endif
endif

include $(topsrcdir)/config/recurse.mk
