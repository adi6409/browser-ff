# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1
DIRS := build gtest
include $(topsrcdir)/config/AB_rCD.mk
PRE_COMPILE_TARGETS += $(MDDEPDIR)/buildid.cpp.stub
buildid.cpp: $(MDDEPDIR)/buildid.cpp.stub ;
EXTRA_MDDEPEND_FILES += $(MDDEPDIR)/buildid.cpp.pp
$(MDDEPDIR)/buildid.cpp.stub: /worker/build/toolkit/library/gen_buildid.py build/libxul_so.list
	$(REPORT_BUILD)
	$(call py_action,file_generate,/worker/build/toolkit/library/gen_buildid.py main buildid.cpp $(MDDEPDIR)/buildid.cpp.pp $(MDDEPDIR)/buildid.cpp.stub build/libxul_so.list)
	@$(TOUCH) $@

LOCAL_INCLUDES += -I$(topsrcdir)/config
LOCAL_INCLUDES += -I$(topsrcdir)/widget/windows
CPPSRCS += buildid.cpp
COMPUTED_LDFLAGS += -lpthread -Wl,-z,noexecstack -Wl,-z,text -Wl,-z,relro -Wl,-z,nocopyreloc -Wl,-Bsymbolic-functions -Wl,--build-id=sha1 -fstack-protector-strong -fstack-clash-protection -Wl,-rpath-link,/worker/build/obj-x86_64-pc-linux-gnu/dist/bin -Wl,-rpath-link,/usr/local/lib
LIBRARY_NAME := xul
FORCE_STATIC_LIB := 1
REAL_LIBRARY := libxul_s.a
SHARED_LIBS += ../../security/sandbox/linux/libmozsandbox.so
SHARED_LIBS += ../../config/external/nspr/pr/libnspr4.so
SHARED_LIBS += ../../config/external/nspr/libc/libplc4.so
SHARED_LIBS += ../../config/external/nspr/ds/libplds4.so
SHARED_LIBS += ../../config/external/lgpllibs/liblgpllibs.so
SHARED_LIBS += ../../security/nss/lib/nss/nss_nss3/libnss3.so
SHARED_LIBS += ../../security/nss/lib/util/util_nssutil3/libnssutil3.so
SHARED_LIBS += ../../security/nss/lib/smime/smime_smime3/libsmime3.so
SHARED_LIBS += ../../config/external/sqlite/libmozsqlite3.so
SHARED_LIBS += ../../security/nss/lib/ssl/ssl_ssl3/libssl3.so
SHARED_LIBS += ../../widget/gtk/mozgtk/stub/libmozgtk_stub.so
SHARED_LIBS += ../../widget/gtk/mozwayland/libmozwayland.so
STATIC_LIBS += ../../security/nss/lib/crmf/crmf_crmf/libcrmf.a
STATIC_LIBS += ../../js/src/build/libjs_static.a
OS_LIBS += -lrt
OS_LIBS += -lXrender
OS_LIBS += -lXext
OS_LIBS += -lXdamage
OS_LIBS += -lXfixes
OS_LIBS += -lXcomposite
OS_LIBS += -ldbus-glib-1
OS_LIBS += -ldbus-1
OS_LIBS += -lgobject-2.0
OS_LIBS += -lglib-2.0
OS_LIBS += -lz
OS_LIBS += -lpangocairo-1.0
OS_LIBS += -lpango-1.0
OS_LIBS += -lharfbuzz
OS_LIBS += -latk-1.0
OS_LIBS += -lcairo-gobject
OS_LIBS += -lcairo
OS_LIBS += -lgdk_pixbuf-2.0
OS_LIBS += -lgio-2.0
OS_LIBS += -lX11-xcb
OS_LIBS += -lxcb-shm
OS_LIBS += -lxcb
OS_LIBS += -lX11
OS_LIBS += -lpangoft2-1.0
OS_LIBS += -lfontconfig
OS_LIBS += -lfreetype
OS_LIBS += -lXt
OS_LIBS += -lgthread-2.0
DEFINES += -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API
COMPUTED_CFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API -I/worker/build/toolkit/library -I/worker/build/obj-x86_64-pc-linux-gnu/toolkit/library -I/worker/build/config -I/worker/build/widget/windows -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -DMOZILLA_CLIENT -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi -fexperimental-new-pass-manager
COMPUTED_CXXFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/stl_wrappers -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API -I/worker/build/toolkit/library -I/worker/build/obj-x86_64-pc-linux-gnu/toolkit/library -I/worker/build/config -I/worker/build/widget/windows -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -DMOZILLA_CLIENT -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -fexperimental-new-pass-manager
COMPUTED_CXX_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables
COMPUTED_C_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi
