# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT MODIFY BY HAND.
DEPTH := ../..
topobjdir := /worker/build/obj-x86_64-pc-linux-gnu
topsrcdir := /worker/build
srcdir := /worker/build/mozglue/build
srcdir_rel := ../../../mozglue/build
relativesrcdir := mozglue/build
include $(DEPTH)/config/autoconf.mk
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# For FORCE_SHARED_LIB
include $(topsrcdir)/config/config.mk

ifeq (WINNT,$(OS_TARGET))
# Rebuild mozglue.dll if the manifest changes - it's included by mozglue.rc.
# (this dependency should really be just for mozglue.dll, not other targets)
# Note the manifest file exists in the tree, so we use the explicit filename
# here.
EXTRA_DEPS += $(srcdir)/mozglue.dll.manifest
endif

include $(topsrcdir)/config/recurse.mk
