# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1 -DOS_POSIX=1 -DOS_LINUX=1 -DNSS_ENABLE_ECC=True '-DDLL_PREFIX="lib"' '-DDLL_SUFFIX=".so"'
include $(topsrcdir)/config/AB_rCD.mk
PRE_COMPILE_TARGETS += $(MDDEPDIR)/xpcshell.inc.stub
xpcshell.inc: $(MDDEPDIR)/xpcshell.inc.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/xpcshell.inc.pp
$(MDDEPDIR)/xpcshell.inc.stub: /worker/build/security/apps/gen_cert_header.py $(topsrcdir)/security/manager/ssl/tests/unit/test_signed_apps/xpcshellTestRoot.der
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/apps/gen_cert_header.py xpcshellRoot xpcshell.inc $(MDDEPDIR)/xpcshell.inc.pp $(MDDEPDIR)/xpcshell.inc.stub $(topsrcdir)/security/manager/ssl/tests/unit/test_signed_apps/xpcshellTestRoot.der)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/addons-public.inc.stub
addons-public.inc: $(MDDEPDIR)/addons-public.inc.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/addons-public.inc.pp
$(MDDEPDIR)/addons-public.inc.stub: /worker/build/security/apps/gen_cert_header.py $(srcdir)/addons-public.crt
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/apps/gen_cert_header.py addonsPublicRoot addons-public.inc $(MDDEPDIR)/addons-public.inc.pp $(MDDEPDIR)/addons-public.inc.stub $(srcdir)/addons-public.crt)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/addons-public-intermediate.inc.stub
addons-public-intermediate.inc: $(MDDEPDIR)/addons-public-intermediate.inc.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/addons-public-intermediate.inc.pp
$(MDDEPDIR)/addons-public-intermediate.inc.stub: /worker/build/security/apps/gen_cert_header.py $(srcdir)/addons-public-intermediate.crt
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/apps/gen_cert_header.py addonsPublicIntermediate addons-public-intermediate.inc $(MDDEPDIR)/addons-public-intermediate.inc.pp $(MDDEPDIR)/addons-public-intermediate.inc.stub $(srcdir)/addons-public-intermediate.crt)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/addons-stage.inc.stub
addons-stage.inc: $(MDDEPDIR)/addons-stage.inc.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/addons-stage.inc.pp
$(MDDEPDIR)/addons-stage.inc.stub: /worker/build/security/apps/gen_cert_header.py $(srcdir)/addons-stage.crt
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/apps/gen_cert_header.py addonsStageRoot addons-stage.inc $(MDDEPDIR)/addons-stage.inc.pp $(MDDEPDIR)/addons-stage.inc.stub $(srcdir)/addons-stage.crt)
	@$(TOUCH) $@

LOCAL_INCLUDES += -I$(topobjdir)/ipc/ipdl/_ipdlheaders
LOCAL_INCLUDES += -I$(topsrcdir)/ipc/chromium/src
LOCAL_INCLUDES += -I$(topsrcdir)/ipc/glue
LOCAL_INCLUDES += -I$(topsrcdir)/security/certverifier
LOCAL_INCLUDES += -I$(topsrcdir)/security/manager/ssl
LOCAL_INCLUDES += -I$(topsrcdir)/third_party/rust/cose-c/include

# We build files in 'unified' mode by including several files
# together into a single source file.  This cuts down on
# compilation times and debug information size.
UNIFIED_CPPSRCS := Unified_cpp_security_apps0.cpp
CPPSRCS += $(UNIFIED_CPPSRCS)
COMPUTED_LDFLAGS += -lpthread -Wl,-z,noexecstack -Wl,-z,text -Wl,-z,relro -Wl,-z,nocopyreloc -Wl,-Bsymbolic-functions -Wl,--build-id=sha1 -fstack-protector-strong -fstack-clash-protection -Wl,-rpath-link,/worker/build/obj-x86_64-pc-linux-gnu/dist/bin -Wl,-rpath-link,/usr/local/lib
LIBRARY_NAME := security_apps
FORCE_STATIC_LIB := 1
REAL_LIBRARY := libsecurity_apps.a
DEFINES += -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API
COMPUTED_CFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DOS_POSIX=1 -DOS_LINUX=1 -DNSS_ENABLE_ECC=True '-DDLL_PREFIX="lib"' '-DDLL_SUFFIX=".so"' -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API -I/worker/build/security/apps -I/worker/build/obj-x86_64-pc-linux-gnu/security/apps -I/worker/build/obj-x86_64-pc-linux-gnu/ipc/ipdl/_ipdlheaders -I/worker/build/ipc/chromium/src -I/worker/build/ipc/glue -I/worker/build/security/certverifier -I/worker/build/security/manager/ssl -I/worker/build/third_party/rust/cose-c/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -DMOZILLA_CLIENT -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi -fexperimental-new-pass-manager
COMPUTED_CXXFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/stl_wrappers -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DOS_POSIX=1 -DOS_LINUX=1 -DNSS_ENABLE_ECC=True '-DDLL_PREFIX="lib"' '-DDLL_SUFFIX=".so"' -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API -I/worker/build/security/apps -I/worker/build/obj-x86_64-pc-linux-gnu/security/apps -I/worker/build/obj-x86_64-pc-linux-gnu/ipc/ipdl/_ipdlheaders -I/worker/build/ipc/chromium/src -I/worker/build/ipc/glue -I/worker/build/security/certverifier -I/worker/build/security/manager/ssl -I/worker/build/third_party/rust/cose-c/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -DMOZILLA_CLIENT -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Wextra -Wno-unused-parameter -fexperimental-new-pass-manager
COMPUTED_CXX_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables
COMPUTED_C_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi
