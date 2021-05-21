# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1 -DOS_POSIX=1 -DOS_LINUX=1 '-DBIN_SUFFIX=""' '-DMOZ_APP_NAME="konke"'
DIRS := jsactor
LOCAL_INCLUDES += -I$(topobjdir)/ipc/ipdl/_ipdlheaders
LOCAL_INCLUDES += -I$(topsrcdir)/ipc/chromium/src
LOCAL_INCLUDES += -I$(topsrcdir)/ipc/glue
LOCAL_INCLUDES += -I$(topsrcdir)/caps
LOCAL_INCLUDES += -I$(topsrcdir)/chrome
LOCAL_INCLUDES += -I$(topsrcdir)/docshell/base
LOCAL_INCLUDES += -I$(topsrcdir)/dom/base
LOCAL_INCLUDES += -I$(topsrcdir)/dom/bindings
LOCAL_INCLUDES += -I$(topsrcdir)/dom/events
LOCAL_INCLUDES += -I$(topsrcdir)/dom/filesystem
LOCAL_INCLUDES += -I$(topsrcdir)/dom/geolocation
LOCAL_INCLUDES += -I$(topsrcdir)/dom/media/webrtc
LOCAL_INCLUDES += -I$(topsrcdir)/dom/media/webspeech/synth/ipc
LOCAL_INCLUDES += -I$(topsrcdir)/dom/security
LOCAL_INCLUDES += -I$(topsrcdir)/dom/storage
LOCAL_INCLUDES += -I$(topsrcdir)/extensions/spellcheck/src
LOCAL_INCLUDES += -I$(topsrcdir)/gfx/2d
LOCAL_INCLUDES += -I$(topsrcdir)/hal/sandbox
LOCAL_INCLUDES += -I$(topsrcdir)/js/xpconnect/loader
LOCAL_INCLUDES += -I$(topsrcdir)/js/xpconnect/src
LOCAL_INCLUDES += -I$(topsrcdir)/layout/base
LOCAL_INCLUDES += -I$(topsrcdir)/media/webrtc
LOCAL_INCLUDES += -I$(topsrcdir)/netwerk/base
LOCAL_INCLUDES += -I$(topsrcdir)/netwerk/protocol/http
LOCAL_INCLUDES += -I$(topsrcdir)/toolkit/components/printingui/ipc
LOCAL_INCLUDES += -I$(topsrcdir)/toolkit/crashreporter
LOCAL_INCLUDES += -I$(topsrcdir)/toolkit/xre
LOCAL_INCLUDES += -I$(topsrcdir)/uriloader/exthandler
LOCAL_INCLUDES += -I$(topsrcdir)/widget
LOCAL_INCLUDES += -I$(topsrcdir)/xpcom/base
LOCAL_INCLUDES += -I$(topsrcdir)/xpcom/threads
LOCAL_INCLUDES += -I$(topsrcdir)/modules/libjar
CPPSRCS += $(srcdir)/ContentChild.cpp
CPPSRCS += $(srcdir)/ProcessHangMonitor.cpp
CPPSRCS += $(srcdir)/VsyncChild.cpp
CPPSRCS += $(srcdir)/VsyncParent.cpp

# We build files in 'unified' mode by including several files
# together into a single source file.  This cuts down on
# compilation times and debug information size.
UNIFIED_CPPSRCS := Unified_cpp_dom_ipc0.cpp Unified_cpp_dom_ipc1.cpp Unified_cpp_dom_ipc2.cpp
CPPSRCS += $(UNIFIED_CPPSRCS)
JAR_MANIFEST := /worker/build/dom/ipc/jar.mn
COMPUTED_LDFLAGS += -lpthread -Wl,-z,noexecstack -Wl,-z,text -Wl,-z,relro -Wl,-z,nocopyreloc -Wl,-Bsymbolic-functions -Wl,--build-id=sha1 -fstack-protector-strong -fstack-clash-protection -Wl,-rpath-link,/worker/build/obj-x86_64-pc-linux-gnu/dist/bin -Wl,-rpath-link,/usr/local/lib
LIBRARY_NAME := dom_ipc
FORCE_STATIC_LIB := 1
REAL_LIBRARY := libdom_ipc.a
SHARED_LIBS += ../../security/sandbox/linux/libmozsandbox.so
DEFINES += -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API
COMPUTED_CFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DOS_POSIX=1 -DOS_LINUX=1 '-DBIN_SUFFIX=""' '-DMOZ_APP_NAME="konke"' -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API -I/worker/build/dom/ipc -I/worker/build/obj-x86_64-pc-linux-gnu/dom/ipc -I/worker/build/obj-x86_64-pc-linux-gnu/ipc/ipdl/_ipdlheaders -I/worker/build/ipc/chromium/src -I/worker/build/ipc/glue -I/worker/build/caps -I/worker/build/chrome -I/worker/build/docshell/base -I/worker/build/dom/base -I/worker/build/dom/bindings -I/worker/build/dom/events -I/worker/build/dom/filesystem -I/worker/build/dom/geolocation -I/worker/build/dom/media/webrtc -I/worker/build/dom/media/webspeech/synth/ipc -I/worker/build/dom/security -I/worker/build/dom/storage -I/worker/build/extensions/spellcheck/src -I/worker/build/gfx/2d -I/worker/build/hal/sandbox -I/worker/build/js/xpconnect/loader -I/worker/build/js/xpconnect/src -I/worker/build/layout/base -I/worker/build/media/webrtc -I/worker/build/netwerk/base -I/worker/build/netwerk/protocol/http -I/worker/build/toolkit/components/printingui/ipc -I/worker/build/toolkit/crashreporter -I/worker/build/toolkit/xre -I/worker/build/uriloader/exthandler -I/worker/build/widget -I/worker/build/xpcom/base -I/worker/build/xpcom/threads -I/worker/build/modules/libjar -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -DMOZILLA_CLIENT -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi -fexperimental-new-pass-manager
COMPUTED_CXXFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/stl_wrappers -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DOS_POSIX=1 -DOS_LINUX=1 '-DBIN_SUFFIX=""' '-DMOZ_APP_NAME="konke"' -DMOZ_HAS_MOZGLUE -DMOZILLA_INTERNAL_API -DIMPL_LIBXUL -DSTATIC_EXPORTABLE_JS_API -I/worker/build/dom/ipc -I/worker/build/obj-x86_64-pc-linux-gnu/dom/ipc -I/worker/build/obj-x86_64-pc-linux-gnu/ipc/ipdl/_ipdlheaders -I/worker/build/ipc/chromium/src -I/worker/build/ipc/glue -I/worker/build/caps -I/worker/build/chrome -I/worker/build/docshell/base -I/worker/build/dom/base -I/worker/build/dom/bindings -I/worker/build/dom/events -I/worker/build/dom/filesystem -I/worker/build/dom/geolocation -I/worker/build/dom/media/webrtc -I/worker/build/dom/media/webspeech/synth/ipc -I/worker/build/dom/security -I/worker/build/dom/storage -I/worker/build/extensions/spellcheck/src -I/worker/build/gfx/2d -I/worker/build/hal/sandbox -I/worker/build/js/xpconnect/loader -I/worker/build/js/xpconnect/src -I/worker/build/layout/base -I/worker/build/media/webrtc -I/worker/build/netwerk/base -I/worker/build/netwerk/protocol/http -I/worker/build/toolkit/components/printingui/ipc -I/worker/build/toolkit/crashreporter -I/worker/build/toolkit/xre -I/worker/build/uriloader/exthandler -I/worker/build/widget -I/worker/build/xpcom/base -I/worker/build/xpcom/threads -I/worker/build/modules/libjar -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -DMOZILLA_CLIENT -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -I/worker/build/widget/gtk/compat-gtk3 -I/usr/include/gtk-3.0 -I/usr/include/pango-1.0 -I/usr/include/glib-2.0 -I/usr/lib/glib-2.0/include -I/usr/include/harfbuzz -I/usr/include/freetype2 -I/usr/include/libpng16 -I/usr/include/libmount -I/usr/include/blkid -I/usr/include/fribidi -I/usr/include/cairo -I/usr/include/lzo -I/usr/include/pixman-1 -I/usr/include/gdk-pixbuf-2.0 -I/usr/include/gio-unix-2.0 -I/usr/include/cloudproviders -I/usr/include/atk-1.0 -I/usr/include/at-spi2-atk/2.0 -I/usr/include/dbus-1.0 -I/usr/lib/dbus-1.0/include -I/usr/include/at-spi-2.0 -I/usr/include/gtk-3.0/unix-print -pthread -Wno-error=shadow -fexperimental-new-pass-manager
COMPUTED_CXX_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables
COMPUTED_C_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi
NONRECURSIVE_TARGETS += export
NONRECURSIVE_TARGETS_export += xpidl
NONRECURSIVE_TARGETS_export_xpidl_DIRECTORY = $(DEPTH)/xpcom/xpidl
NONRECURSIVE_TARGETS_export_xpidl_TARGETS += export
