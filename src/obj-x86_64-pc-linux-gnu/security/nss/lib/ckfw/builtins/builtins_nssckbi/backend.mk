# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG -DTRIMMED=1 -DNSS_FIPS_DISABLED -DNSS_NO_INIT_SUPPORT -DNSS_X86_OR_X64 -DNSS_X64 -DNSS_USE_64 -DUSE_UTIL_DIRECTLY -DNO_NSPR_10_SUPPORT -DSSL_DISABLE_DEPRECATED_CIPHER_SUITE_NAMES -DLINUX2_1 -DLINUX -Dlinux -D_DEFAULT_SOURCE -D_BSD_SOURCE -D_POSIX_SOURCE -DSQL_MEASURE_USE_TEMP_DIR -DHAVE_STRERROR -DXP_UNIX -D_REENTRANT -DNSS_DISABLE_DBM -DNSS_ENABLE_DRAFT_HPKE -DNSS_DISABLE_LIBPKIX
include $(topsrcdir)/config/AB_rCD.mk
PRE_COMPILE_TARGETS += $(MDDEPDIR)/certdata.c.stub
certdata.c: $(MDDEPDIR)/certdata.c.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/certdata.c.pp
$(MDDEPDIR)/certdata.c.stub: /worker/build/security/generate_certdata.py $(srcdir)/certdata.py $(srcdir)/certdata.perl $(srcdir)/certdata.txt
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/generate_certdata.py main certdata.c $(MDDEPDIR)/certdata.c.pp $(MDDEPDIR)/certdata.c.stub $(srcdir)/certdata.py $(srcdir)/certdata.perl $(srcdir)/certdata.txt)
	@$(TOUCH) $@

PRE_COMPILE_TARGETS += $(MDDEPDIR)/out.nssckbi.def.stub
out.nssckbi.def: $(MDDEPDIR)/out.nssckbi.def.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/out.nssckbi.def.pp
$(MDDEPDIR)/out.nssckbi.def.stub: /worker/build/security/generate_mapfile.py $(srcdir)/nssckbi.def
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/security/generate_mapfile.py main out.nssckbi.def $(MDDEPDIR)/out.nssckbi.def.pp $(MDDEPDIR)/out.nssckbi.def.stub $(srcdir)/nssckbi.def)
	@$(TOUCH) $@

LOCAL_INCLUDES += -I$(srcdir)
LOCAL_INCLUDES += -I$(topobjdir)/dist/include/nspr
LOCAL_INCLUDES += -I$(topobjdir)/dist/include/private/nss
LOCAL_INCLUDES += -I$(topobjdir)/dist/include/nss
CSRCS += $(topsrcdir)/security/nss/coreconf/empty.c
CSRCS += $(srcdir)/anchor.c
CSRCS += $(srcdir)/bfind.c
CSRCS += $(srcdir)/binst.c
CSRCS += $(srcdir)/bobject.c
CSRCS += $(srcdir)/bsession.c
CSRCS += $(srcdir)/bslot.c
CSRCS += $(srcdir)/btoken.c
CSRCS += $(srcdir)/ckbiver.c
CSRCS += $(srcdir)/constants.c
CSRCS += certdata.c
NO_PROFILE_GUIDED_OPTIMIZE := 1
COMPUTED_LDFLAGS += -lpthread -Wl,-z,noexecstack -Wl,-z,text -Wl,-z,relro -Wl,-z,nocopyreloc -Wl,-Bsymbolic-functions -Wl,--build-id=sha1 -fstack-protector-strong -fstack-clash-protection -Wl,-rpath-link,/worker/build/obj-x86_64-pc-linux-gnu/dist/bin -Wl,-rpath-link,/usr/local/lib
LIBRARY_NAME := nssckbi
FORCE_SHARED_LIB := 1
IMPORT_LIBRARY := libnssckbi.so
SHARED_LIBRARY := libnssckbi.so
DSO_SONAME := libnssckbi.so
EXTRA_DSO_LDOPTS += -Wl,--version-script,out.nssckbi.def
LIB_IS_C_ONLY := 1
libnssckbi.so_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/security/nss/lib/ckfw/builtins/builtins_nssckbi/libnssckbi_so.list
libnssckbi.so: libnssckbi_so.list
libnssckbi.so: empty.o \
    anchor.o \
    bfind.o \
    binst.o \
    bobject.o \
    bsession.o \
    bslot.o \
    btoken.o \
    ckbiver.o \
    constants.o \
    certdata.o \
    ../../../base/base_nssb/arena.o \
    ../../../base/base_nssb/error.o \
    ../../../base/base_nssb/errorval.o \
    ../../../base/base_nssb/hash.o \
    ../../../base/base_nssb/hashops.o \
    ../../../base/base_nssb/item.o \
    ../../../base/base_nssb/libc.o \
    ../../../base/base_nssb/list.o \
    ../../../base/base_nssb/tracker.o \
    ../../../base/base_nssb/utf8.o \
    ../../ckfw_nssckfw/crypto.o \
    ../../ckfw_nssckfw/find.o \
    ../../ckfw_nssckfw/hash.o \
    ../../ckfw_nssckfw/instance.o \
    ../../ckfw_nssckfw/mechanism.o \
    ../../ckfw_nssckfw/mutex.o \
    ../../ckfw_nssckfw/object.o \
    ../../ckfw_nssckfw/session.o \
    ../../ckfw_nssckfw/sessobj.o \
    ../../ckfw_nssckfw/slot.o \
    ../../ckfw_nssckfw/token.o \
    ../../ckfw_nssckfw/wrap.o
SHARED_LIBS += ../../../../../../config/external/nspr/pr/libnspr4.so
SHARED_LIBS += ../../../../../../config/external/nspr/libc/libplc4.so
SHARED_LIBS += ../../../../../../config/external/nspr/ds/libplds4.so
OS_LIBS += -lpthread
OS_LIBS += -ldl
OS_LIBS += -lc
COMPUTED_CFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG -DTRIMMED=1 -DNSS_FIPS_DISABLED -DNSS_NO_INIT_SUPPORT -DNSS_X86_OR_X64 -DNSS_X64 -DNSS_USE_64 -DUSE_UTIL_DIRECTLY -DNO_NSPR_10_SUPPORT -DSSL_DISABLE_DEPRECATED_CIPHER_SUITE_NAMES -DLINUX2_1 -DLINUX -Dlinux -D_DEFAULT_SOURCE -D_BSD_SOURCE -D_POSIX_SOURCE -DSQL_MEASURE_USE_TEMP_DIR -DHAVE_STRERROR -DXP_UNIX -D_REENTRANT -DNSS_DISABLE_DBM -DNSS_ENABLE_DRAFT_HPKE -DNSS_DISABLE_LIBPKIX -I/worker/build/security/nss/lib/ckfw/builtins -I/worker/build/obj-x86_64-pc-linux-gnu/security/nss/lib/ckfw/builtins/builtins_nssckbi -I/worker/build/security/nss/lib/ckfw/builtins -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/private/nss -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -fPIC -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -DMOZILLA_CLIENT -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi -fexperimental-new-pass-manager
COMPUTED_CXXFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/stl_wrappers -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG -DTRIMMED=1 -DNSS_FIPS_DISABLED -DNSS_NO_INIT_SUPPORT -DNSS_X86_OR_X64 -DNSS_X64 -DNSS_USE_64 -DUSE_UTIL_DIRECTLY -DNO_NSPR_10_SUPPORT -DSSL_DISABLE_DEPRECATED_CIPHER_SUITE_NAMES -DLINUX2_1 -DLINUX -Dlinux -D_DEFAULT_SOURCE -D_BSD_SOURCE -D_POSIX_SOURCE -DSQL_MEASURE_USE_TEMP_DIR -DHAVE_STRERROR -DXP_UNIX -D_REENTRANT -DNSS_DISABLE_DBM -DNSS_ENABLE_DRAFT_HPKE -DNSS_DISABLE_LIBPKIX -I/worker/build/security/nss/lib/ckfw/builtins -I/worker/build/obj-x86_64-pc-linux-gnu/security/nss/lib/ckfw/builtins/builtins_nssckbi -I/worker/build/security/nss/lib/ckfw/builtins -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/private/nss -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -fPIC -DMOZILLA_CLIENT -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -fexperimental-new-pass-manager
COMPUTED_CXX_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables
COMPUTED_C_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi
