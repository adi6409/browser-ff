# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1 -DXPCOM_GLUE -DMOZ_HAS_MOZGLUE -DMOZ_GECKODRIVER '-DFIREFOX_ICO="/worker/build/browser/branding/dot/firefox.ico"' '-DDOCUMENT_ICO="/worker/build/browser/branding/dot/document.ico"' '-DNEWWINDOW_ICO="/worker/build/browser/branding/dot/newwindow.ico"' '-DNEWTAB_ICO="/worker/build/browser/branding/dot/newtab.ico"' '-DPBMODE_ICO="/worker/build/browser/branding/dot/pbmode.ico"'
FINAL_TARGET = $(if $(XPI_NAME),$(DIST)/xpi-stage/$(XPI_NAME),$(DIST)/bin)$(DIST_SUBDIR:%=/%)
DIRS := macbuild/Contents
LOCAL_INCLUDES += -I$(topobjdir)/build
LOCAL_INCLUDES += -I$(topsrcdir)/toolkit/xre
LOCAL_INCLUDES += -I$(topsrcdir)/xpcom/base
LOCAL_INCLUDES += -I$(topsrcdir)/xpcom/build
CPPSRCS += $(srcdir)/nsBrowserApp.cpp
COMPUTED_LDFLAGS += -lpthread -Wl,-z,noexecstack -Wl,-z,text -Wl,-z,relro -Wl,-z,nocopyreloc -Wl,-Bsymbolic-functions -Wl,--build-id=sha1 -fstack-protector-strong -fstack-clash-protection -rdynamic -Wl,-rpath-link,/worker/build/obj-x86_64-pc-linux-gnu/dist/bin -Wl,-rpath-link,/usr/local/lib
COMPUTED_CFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DXPCOM_GLUE -DMOZ_HAS_MOZGLUE -DMOZ_GECKODRIVER '-DFIREFOX_ICO="/worker/build/browser/branding/dot/firefox.ico"' '-DDOCUMENT_ICO="/worker/build/browser/branding/dot/document.ico"' '-DNEWWINDOW_ICO="/worker/build/browser/branding/dot/newwindow.ico"' '-DNEWTAB_ICO="/worker/build/browser/branding/dot/newtab.ico"' '-DPBMODE_ICO="/worker/build/browser/branding/dot/pbmode.ico"' -I/worker/build/browser/app -I/worker/build/obj-x86_64-pc-linux-gnu/browser/app -I/worker/build/obj-x86_64-pc-linux-gnu/build -I/worker/build/toolkit/xre -I/worker/build/xpcom/base -I/worker/build/xpcom/build -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -DMOZILLA_CLIENT -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi -fexperimental-new-pass-manager
COMPUTED_CXXFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DXPCOM_GLUE -DMOZ_HAS_MOZGLUE -DMOZ_GECKODRIVER '-DFIREFOX_ICO="/worker/build/browser/branding/dot/firefox.ico"' '-DDOCUMENT_ICO="/worker/build/browser/branding/dot/document.ico"' '-DNEWWINDOW_ICO="/worker/build/browser/branding/dot/newwindow.ico"' '-DNEWTAB_ICO="/worker/build/browser/branding/dot/newtab.ico"' '-DPBMODE_ICO="/worker/build/browser/branding/dot/pbmode.ico"' -I/worker/build/browser/app -I/worker/build/obj-x86_64-pc-linux-gnu/browser/app -I/worker/build/obj-x86_64-pc-linux-gnu/build -I/worker/build/toolkit/xre -I/worker/build/xpcom/base -I/worker/build/xpcom/build -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -DMOZILLA_CLIENT -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -fexperimental-new-pass-manager
COMPUTED_CXX_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables
COMPUTED_C_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi
PROGRAM = $(DEPTH)/dist/bin/konke
konke_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/browser/app/konke.list
$(DEPTH)/dist/bin/konke: konke.list
$(DEPTH)/dist/bin/konke: nsBrowserApp.o \
    ../../xpcom/glue/standalone/FileUtils.o \
    ../../xpcom/glue/standalone/MemUtils.o \
    ../../xpcom/glue/standalone/nsXPCOMGlue.o \
    ../../mozglue/build/SSE.o \
    ../../mozglue/build/dummy.o \
    ../../memory/build/Unified_cpp_memory_build0.o \
    ../../memory/mozalloc/cxxalloc.o \
    ../../memory/mozalloc/mozalloc_abort.o \
    ../../memory/mozalloc/Unified_cpp_memory_mozalloc0.o \
    ../../mozglue/baseprofiler/shared-libraries-linux.o \
    ../../mozglue/baseprofiler/Unified_cpp_mozglue_baseprofiler0.o \
    ../../mozglue/baseprofiler/Unified_cpp_mozglue_baseprofiler1.o \
    ../../mozglue/misc/AutoProfilerLabel.o \
    ../../mozglue/misc/ConditionVariable_posix.o \
    ../../mozglue/misc/MmapFaultHandler.o \
    ../../mozglue/misc/Mutex_posix.o \
    ../../mozglue/misc/Printf.o \
    ../../mozglue/misc/StackWalk.o \
    ../../mozglue/misc/TimeStamp.o \
    ../../mozglue/misc/TimeStamp_posix.o \
    ../../mozglue/misc/Uptime.o \
    ../../mozglue/misc/Decimal.o \
    ../../mfbt/lz4.o \
    ../../mfbt/lz4frame.o \
    ../../mfbt/lz4hc.o \
    ../../mfbt/xxhash.o \
    ../../mfbt/Unified_cpp_mfbt0.o \
    ../../mfbt/Unified_cpp_mfbt1.o
