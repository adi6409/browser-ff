# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1 -DTEST_UPDATER -DNS_NO_XPCOM '-DMAR_CHANNEL_ID=""' '-DMOZ_APP_VERSION="87.0"'
FINAL_TARGET = $(DEPTH)/_tests/xpcshell/toolkit/mozapps/update/tests
LOCAL_INCLUDES += -I$(topsrcdir)/toolkit/mozapps/update/common
LOCAL_INCLUDES += -I$(topsrcdir)/xpcom/base
CPPSRCS += $(topsrcdir)/toolkit/mozapps/update/updater/archivereader.cpp
CPPSRCS += $(topsrcdir)/toolkit/mozapps/update/updater/progressui_gtk.cpp
CPPSRCS += $(topsrcdir)/toolkit/mozapps/update/updater/updater.cpp
COMPUTED_LDFLAGS += -lpthread -Wl,-z,noexecstack -Wl,-z,text -Wl,-z,relro -Wl,-z,nocopyreloc -Wl,-Bsymbolic-functions -Wl,--build-id=sha1 -fstack-protector-strong -fstack-clash-protection -Wl,-rpath-link,/worker/build/obj-x86_64-pc-linux-gnu/dist/bin -Wl,-rpath-link,/usr/local/lib
COMPUTED_CFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DTEST_UPDATER -DNS_NO_XPCOM '-DMAR_CHANNEL_ID=""' '-DMOZ_APP_VERSION="87.0"' -I/worker/build/toolkit/mozapps/update/updater/updater-xpcshell -I/worker/build/obj-x86_64-pc-linux-gnu/toolkit/mozapps/update/updater/updater-xpcshell -I/worker/build/toolkit/mozapps/update/common -I/worker/build/xpcom/base -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -DMOZILLA_CLIENT -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi -fexperimental-new-pass-manager
COMPUTED_CXXFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DTEST_UPDATER -DNS_NO_XPCOM '-DMAR_CHANNEL_ID=""' '-DMOZ_APP_VERSION="87.0"' -I/worker/build/toolkit/mozapps/update/updater/updater-xpcshell -I/worker/build/obj-x86_64-pc-linux-gnu/toolkit/mozapps/update/updater/updater-xpcshell -I/worker/build/toolkit/mozapps/update/common -I/worker/build/xpcom/base -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -DMOZILLA_CLIENT -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -I/worker/build/widget/gtk/compat-gtk3 -I/usr/include/gtk-3.0 -I/usr/include/pango-1.0 -I/usr/include/glib-2.0 -I/usr/lib/glib-2.0/include -I/usr/include/harfbuzz -I/usr/include/freetype2 -I/usr/include/libpng16 -I/usr/include/libmount -I/usr/include/blkid -I/usr/include/fribidi -I/usr/include/cairo -I/usr/include/lzo -I/usr/include/pixman-1 -I/usr/include/gdk-pixbuf-2.0 -I/usr/include/gio-unix-2.0 -I/usr/include/cloudproviders -I/usr/include/atk-1.0 -I/usr/include/at-spi2-atk/2.0 -I/usr/include/dbus-1.0 -I/usr/lib/dbus-1.0/include -I/usr/include/at-spi-2.0 -I/usr/include/gtk-3.0/unix-print -pthread -fexperimental-new-pass-manager
COMPUTED_CXX_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables
COMPUTED_C_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi
PROGRAM = $(DEPTH)/_tests/xpcshell/toolkit/mozapps/update/tests/updater-xpcshell
updater-xpcshell_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/toolkit/mozapps/update/updater/updater-xpcshell/updater-xpcshell.list
$(DEPTH)/_tests/xpcshell/toolkit/mozapps/update/tests/updater-xpcshell: updater-xpcshell.list
$(DEPTH)/_tests/xpcshell/toolkit/mozapps/update/tests/updater-xpcshell: archivereader.o \
    progressui_gtk.o \
    updater.o \
    ../../../../../modules/libmar/verify/Unified_c_libmar_verify0.o \
    ../../../../../modules/libmar/sign/Unified_c_modules_libmar_sign0.o \
    ../bspatch/bspatch.o \
    ../../common/city.o \
    ../../common/commonupdatedir.o \
    ../../common/readstrings.o \
    ../../common/updatecommon.o \
    ../../../../../modules/libmar/src/Unified_c_modules_libmar_src0.o \
    ../../../../../modules/xz-embedded/Unified_c_modules_xz-embedded0.o
SHARED_LIBS += ../../../../../config/external/nspr/pr/libnspr4.so
SHARED_LIBS += ../../../../../config/external/nspr/libc/libplc4.so
SHARED_LIBS += ../../../../../config/external/nspr/ds/libplds4.so
SHARED_LIBS += ../../../../../security/nss/lib/nss/nss_nss3/libnss3.so
SHARED_LIBS += ../../../../../security/nss/lib/util/util_nssutil3/libnssutil3.so
SHARED_LIBS += ../../../../../security/nss/lib/smime/smime_smime3/libsmime3.so
SHARED_LIBS += ../../../../../config/external/sqlite/libmozsqlite3.so
SHARED_LIBS += ../../../../../security/nss/lib/ssl/ssl_ssl3/libssl3.so
OS_LIBS += -Wl,-rpath=\$$ORIGIN
OS_LIBS += -lgtk-3
OS_LIBS += -lgdk-3
OS_LIBS += -lz
OS_LIBS += -lpangocairo-1.0
OS_LIBS += -lpango-1.0
OS_LIBS += -lharfbuzz
OS_LIBS += -latk-1.0
OS_LIBS += -lcairo-gobject
OS_LIBS += -lcairo
OS_LIBS += -lgdk_pixbuf-2.0
OS_LIBS += -lgio-2.0
OS_LIBS += -lgobject-2.0
OS_LIBS += -lglib-2.0
