# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1 -DMOZ_HAS_MOZGLUE -DNO_STABS_SUPPORT
LOCAL_INCLUDES += -I$(topsrcdir)/toolkit/components/jsoncpp/include
LOCAL_INCLUDES += -I$(topsrcdir)/toolkit/crashreporter/breakpad-client
LOCAL_INCLUDES += -I$(topsrcdir)/toolkit/crashreporter/google-breakpad/src

# We build files in 'unified' mode by including several files
# together into a single source file.  This cuts down on
# compilation times and debug information size.
UNIFIED_CPPSRCS := Unified_cpp_minidump-analyzer0.cpp
CPPSRCS += $(UNIFIED_CPPSRCS)
COMPUTED_LDFLAGS += -lpthread -Wl,-z,noexecstack -Wl,-z,text -Wl,-z,relro -Wl,-z,nocopyreloc -Wl,-Bsymbolic-functions -Wl,--build-id=sha1 -fstack-protector-strong -fstack-clash-protection -rdynamic -Wl,-rpath-link,/worker/build/obj-x86_64-pc-linux-gnu/dist/bin -Wl,-rpath-link,/usr/local/lib
COMPUTED_CFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DMOZ_HAS_MOZGLUE -DNO_STABS_SUPPORT -I/worker/build/toolkit/crashreporter/minidump-analyzer -I/worker/build/obj-x86_64-pc-linux-gnu/toolkit/crashreporter/minidump-analyzer -I/worker/build/toolkit/components/jsoncpp/include -I/worker/build/toolkit/crashreporter/breakpad-client -I/worker/build/toolkit/crashreporter/google-breakpad/src -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -DMOZILLA_CLIENT -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi -fexperimental-new-pass-manager
COMPUTED_CXXFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DMOZ_HAS_MOZGLUE -DNO_STABS_SUPPORT -I/worker/build/toolkit/crashreporter/minidump-analyzer -I/worker/build/obj-x86_64-pc-linux-gnu/toolkit/crashreporter/minidump-analyzer -I/worker/build/toolkit/components/jsoncpp/include -I/worker/build/toolkit/crashreporter/breakpad-client -I/worker/build/toolkit/crashreporter/google-breakpad/src -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -DMOZILLA_CLIENT -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Wno-unused-local-typedefs -Wno-shadow -Wno-deprecated-declarations -Wno-bool-compare -Wno-unused-but-set-variable -Wno-c++11-narrowing -Wno-implicit-fallthrough -fexperimental-new-pass-manager
COMPUTED_CXX_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables
COMPUTED_C_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi
PROGRAM = $(DEPTH)/dist/bin/minidump-analyzer
minidump-analyzer_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/toolkit/crashreporter/minidump-analyzer/minidump-analyzer.list
$(DEPTH)/dist/bin/minidump-analyzer: minidump-analyzer.list
$(DEPTH)/dist/bin/minidump-analyzer: Unified_cpp_minidump-analyzer0.o \
    ../../../mozglue/build/SSE.o \
    ../../../mozglue/build/dummy.o \
    ../../../memory/build/Unified_cpp_memory_build0.o \
    ../../../memory/mozalloc/cxxalloc.o \
    ../../../memory/mozalloc/mozalloc_abort.o \
    ../../../memory/mozalloc/Unified_cpp_memory_mozalloc0.o \
    ../../../mozglue/baseprofiler/shared-libraries-linux.o \
    ../../../mozglue/baseprofiler/Unified_cpp_mozglue_baseprofiler0.o \
    ../../../mozglue/baseprofiler/Unified_cpp_mozglue_baseprofiler1.o \
    ../../../mozglue/misc/AutoProfilerLabel.o \
    ../../../mozglue/misc/ConditionVariable_posix.o \
    ../../../mozglue/misc/MmapFaultHandler.o \
    ../../../mozglue/misc/Mutex_posix.o \
    ../../../mozglue/misc/Printf.o \
    ../../../mozglue/misc/StackWalk.o \
    ../../../mozglue/misc/TimeStamp.o \
    ../../../mozglue/misc/TimeStamp_posix.o \
    ../../../mozglue/misc/Uptime.o \
    ../../../mozglue/misc/Decimal.o \
    ../../../mfbt/lz4.o \
    ../../../mfbt/lz4frame.o \
    ../../../mfbt/lz4hc.o \
    ../../../mfbt/xxhash.o \
    ../../../mfbt/Unified_cpp_mfbt0.o \
    ../../../mfbt/Unified_cpp_mfbt1.o \
    ../google-breakpad/src/processor/ia32_invariant.o \
    ../google-breakpad/src/processor/Unified_c_src_processor0.o \
    ../google-breakpad/src/processor/disassembler_x86.o \
    ../google-breakpad/src/processor/exploitability_win.o \
    ../google-breakpad/src/processor/Unified_cpp_src_processor0.o \
    ../google-breakpad/src/processor/Unified_cpp_src_processor1.o \
    ../../components/jsoncpp/src/lib_json/Unified_cpp_src_lib_json0.o
