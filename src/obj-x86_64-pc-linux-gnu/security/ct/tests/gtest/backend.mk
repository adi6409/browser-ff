# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG -DTRIMMED=1
include $(topsrcdir)/config/AB_rCD.mk
PRE_COMPILE_TARGETS += $(MDDEPDIR)/valid-sth.inc.stub
valid-sth.inc: $(MDDEPDIR)/valid-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/valid-sth.inc.pp
$(MDDEPDIR)/valid-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(srcdir)/valid-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main valid-sth.inc $(MDDEPDIR)/valid-sth.inc.pp $(MDDEPDIR)/valid-sth.inc.stub $(srcdir)/valid-sth.inc.tbs)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/valid-with-extension-sth.inc.stub
valid-with-extension-sth.inc: $(MDDEPDIR)/valid-with-extension-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/valid-with-extension-sth.inc.pp
$(MDDEPDIR)/valid-with-extension-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(srcdir)/valid-with-extension-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main valid-with-extension-sth.inc $(MDDEPDIR)/valid-with-extension-sth.inc.pp $(MDDEPDIR)/valid-with-extension-sth.inc.stub $(srcdir)/valid-with-extension-sth.inc.tbs)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/valid-secp521r1-sha512-sth.inc.stub
valid-secp521r1-sha512-sth.inc: $(MDDEPDIR)/valid-secp521r1-sha512-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/valid-secp521r1-sha512-sth.inc.pp
$(MDDEPDIR)/valid-secp521r1-sha512-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(srcdir)/valid-secp521r1-sha512-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main valid-secp521r1-sha512-sth.inc $(MDDEPDIR)/valid-secp521r1-sha512-sth.inc.pp $(MDDEPDIR)/valid-secp521r1-sha512-sth.inc.stub $(srcdir)/valid-secp521r1-sha512-sth.inc.tbs)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/signature-covers-log-id-sth.inc.stub
signature-covers-log-id-sth.inc: $(MDDEPDIR)/signature-covers-log-id-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/signature-covers-log-id-sth.inc.pp
$(MDDEPDIR)/signature-covers-log-id-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(srcdir)/signature-covers-log-id-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main signature-covers-log-id-sth.inc $(MDDEPDIR)/signature-covers-log-id-sth.inc.pp $(MDDEPDIR)/signature-covers-log-id-sth.inc.stub $(srcdir)/signature-covers-log-id-sth.inc.tbs)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/wrong-spki-sth.inc.stub
wrong-spki-sth.inc: $(MDDEPDIR)/wrong-spki-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/wrong-spki-sth.inc.pp
$(MDDEPDIR)/wrong-spki-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(srcdir)/wrong-spki-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main wrong-spki-sth.inc $(MDDEPDIR)/wrong-spki-sth.inc.pp $(MDDEPDIR)/wrong-spki-sth.inc.stub $(srcdir)/wrong-spki-sth.inc.tbs)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/wrong-signing-key-sth.inc.stub
wrong-signing-key-sth.inc: $(MDDEPDIR)/wrong-signing-key-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/wrong-signing-key-sth.inc.pp
$(MDDEPDIR)/wrong-signing-key-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(srcdir)/wrong-signing-key-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main wrong-signing-key-sth.inc $(MDDEPDIR)/wrong-signing-key-sth.inc.pp $(MDDEPDIR)/wrong-signing-key-sth.inc.stub $(srcdir)/wrong-signing-key-sth.inc.tbs)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/missing-log-id-sth.inc.stub
missing-log-id-sth.inc: $(MDDEPDIR)/missing-log-id-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/missing-log-id-sth.inc.pp
$(MDDEPDIR)/missing-log-id-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(srcdir)/missing-log-id-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main missing-log-id-sth.inc $(MDDEPDIR)/missing-log-id-sth.inc.pp $(MDDEPDIR)/missing-log-id-sth.inc.stub $(srcdir)/missing-log-id-sth.inc.tbs)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/missing-timestamp-sth.inc.stub
missing-timestamp-sth.inc: $(MDDEPDIR)/missing-timestamp-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/missing-timestamp-sth.inc.pp
$(MDDEPDIR)/missing-timestamp-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(srcdir)/missing-timestamp-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main missing-timestamp-sth.inc $(MDDEPDIR)/missing-timestamp-sth.inc.pp $(MDDEPDIR)/missing-timestamp-sth.inc.stub $(srcdir)/missing-timestamp-sth.inc.tbs)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/missing-tree-size-sth.inc.stub
missing-tree-size-sth.inc: $(MDDEPDIR)/missing-tree-size-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/missing-tree-size-sth.inc.pp
$(MDDEPDIR)/missing-tree-size-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(srcdir)/missing-tree-size-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main missing-tree-size-sth.inc $(MDDEPDIR)/missing-tree-size-sth.inc.pp $(MDDEPDIR)/missing-tree-size-sth.inc.stub $(srcdir)/missing-tree-size-sth.inc.tbs)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/missing-root-hash-sth.inc.stub
missing-root-hash-sth.inc: $(MDDEPDIR)/missing-root-hash-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/missing-root-hash-sth.inc.pp
$(MDDEPDIR)/missing-root-hash-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(srcdir)/missing-root-hash-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main missing-root-hash-sth.inc $(MDDEPDIR)/missing-root-hash-sth.inc.pp $(MDDEPDIR)/missing-root-hash-sth.inc.stub $(srcdir)/missing-root-hash-sth.inc.tbs)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/missing-extensions-sth.inc.stub
missing-extensions-sth.inc: $(MDDEPDIR)/missing-extensions-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/missing-extensions-sth.inc.pp
$(MDDEPDIR)/missing-extensions-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(srcdir)/missing-extensions-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main missing-extensions-sth.inc $(MDDEPDIR)/missing-extensions-sth.inc.pp $(MDDEPDIR)/missing-extensions-sth.inc.stub $(srcdir)/missing-extensions-sth.inc.tbs)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/truncated-log-id-sth.inc.stub
truncated-log-id-sth.inc: $(MDDEPDIR)/truncated-log-id-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/truncated-log-id-sth.inc.pp
$(MDDEPDIR)/truncated-log-id-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(srcdir)/truncated-log-id-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main truncated-log-id-sth.inc $(MDDEPDIR)/truncated-log-id-sth.inc.pp $(MDDEPDIR)/truncated-log-id-sth.inc.stub $(srcdir)/truncated-log-id-sth.inc.tbs)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/truncated-timestamp-sth.inc.stub
truncated-timestamp-sth.inc: $(MDDEPDIR)/truncated-timestamp-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/truncated-timestamp-sth.inc.pp
$(MDDEPDIR)/truncated-timestamp-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(srcdir)/truncated-timestamp-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main truncated-timestamp-sth.inc $(MDDEPDIR)/truncated-timestamp-sth.inc.pp $(MDDEPDIR)/truncated-timestamp-sth.inc.stub $(srcdir)/truncated-timestamp-sth.inc.tbs)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/truncated-tree-size-sth.inc.stub
truncated-tree-size-sth.inc: $(MDDEPDIR)/truncated-tree-size-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/truncated-tree-size-sth.inc.pp
$(MDDEPDIR)/truncated-tree-size-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(srcdir)/truncated-tree-size-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main truncated-tree-size-sth.inc $(MDDEPDIR)/truncated-tree-size-sth.inc.pp $(MDDEPDIR)/truncated-tree-size-sth.inc.stub $(srcdir)/truncated-tree-size-sth.inc.tbs)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/truncated-root-hash-sth.inc.stub
truncated-root-hash-sth.inc: $(MDDEPDIR)/truncated-root-hash-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/truncated-root-hash-sth.inc.pp
$(MDDEPDIR)/truncated-root-hash-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(srcdir)/truncated-root-hash-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main truncated-root-hash-sth.inc $(MDDEPDIR)/truncated-root-hash-sth.inc.pp $(MDDEPDIR)/truncated-root-hash-sth.inc.stub $(srcdir)/truncated-root-hash-sth.inc.tbs)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/truncated-extension-sth.inc.stub
truncated-extension-sth.inc: $(MDDEPDIR)/truncated-extension-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/truncated-extension-sth.inc.pp
$(MDDEPDIR)/truncated-extension-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(srcdir)/truncated-extension-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main truncated-extension-sth.inc $(MDDEPDIR)/truncated-extension-sth.inc.pp $(MDDEPDIR)/truncated-extension-sth.inc.stub $(srcdir)/truncated-extension-sth.inc.tbs)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/rsa-signer-rsa-spki-sth.inc.stub
rsa-signer-rsa-spki-sth.inc: $(MDDEPDIR)/rsa-signer-rsa-spki-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/rsa-signer-rsa-spki-sth.inc.pp
$(MDDEPDIR)/rsa-signer-rsa-spki-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(srcdir)/rsa-signer-rsa-spki-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main rsa-signer-rsa-spki-sth.inc $(MDDEPDIR)/rsa-signer-rsa-spki-sth.inc.pp $(MDDEPDIR)/rsa-signer-rsa-spki-sth.inc.stub $(srcdir)/rsa-signer-rsa-spki-sth.inc.tbs)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/rsa-signer-ec-spki-sth.inc.stub
rsa-signer-ec-spki-sth.inc: $(MDDEPDIR)/rsa-signer-ec-spki-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/rsa-signer-ec-spki-sth.inc.pp
$(MDDEPDIR)/rsa-signer-ec-spki-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(srcdir)/rsa-signer-ec-spki-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main rsa-signer-ec-spki-sth.inc $(MDDEPDIR)/rsa-signer-ec-spki-sth.inc.pp $(MDDEPDIR)/rsa-signer-ec-spki-sth.inc.stub $(srcdir)/rsa-signer-ec-spki-sth.inc.tbs)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/ec-signer-rsa-spki-sth.inc.stub
ec-signer-rsa-spki-sth.inc: $(MDDEPDIR)/ec-signer-rsa-spki-sth.inc.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/ec-signer-rsa-spki-sth.inc.pp
$(MDDEPDIR)/ec-signer-rsa-spki-sth.inc.stub: /worker/build/security/ct/tests/gtest/createSTHTestData.py $(srcdir)/ec-signer-rsa-spki-sth.inc.tbs
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/ct/tests/gtest/createSTHTestData.py main ec-signer-rsa-spki-sth.inc $(MDDEPDIR)/ec-signer-rsa-spki-sth.inc.pp $(MDDEPDIR)/ec-signer-rsa-spki-sth.inc.stub $(srcdir)/ec-signer-rsa-spki-sth.inc.tbs)
	@$(TOUCH) $@

LOCAL_INCLUDES += -I$(topsrcdir)/security/ct

# We build files in 'unified' mode by including several files
# together into a single source file.  This cuts down on
# compilation times and debug information size.
UNIFIED_CPPSRCS := Unified_cpp_ct_tests_gtest0.cpp
CPPSRCS += $(UNIFIED_CPPSRCS)
COMPUTED_LDFLAGS += -lpthread -Wl,-z,noexecstack -Wl,-z,text -Wl,-z,relro -Wl,-z,nocopyreloc -Wl,-Bsymbolic-functions -Wl,--build-id=sha1 -fstack-protector-strong -fstack-clash-protection -Wl,-rpath-link,/worker/build/obj-x86_64-pc-linux-gnu/dist/bin -Wl,-rpath-link,/usr/local/lib
LIBRARY_NAME := security_ct_tests_gtest
FORCE_STATIC_LIB := 1
REAL_LIBRARY := libsecurity_ct_tests_gtest.a
DEFINES += -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API
COMPUTED_CFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG -DTRIMMED=1 -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API -I/worker/build/security/ct/tests/gtest -I/worker/build/obj-x86_64-pc-linux-gnu/security/ct/tests/gtest -I/worker/build/security/ct -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -DMOZILLA_CLIENT -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi -fexperimental-new-pass-manager
COMPUTED_CXXFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/stl_wrappers -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG -DTRIMMED=1 -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API -I/worker/build/security/ct/tests/gtest -I/worker/build/obj-x86_64-pc-linux-gnu/security/ct/tests/gtest -I/worker/build/security/ct -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -DMOZILLA_CLIENT -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -fexperimental-new-pass-manager
COMPUTED_CXX_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables
COMPUTED_C_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi
