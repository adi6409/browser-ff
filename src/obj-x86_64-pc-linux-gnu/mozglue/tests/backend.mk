# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1 -DMOZ_HAS_MOZGLUE
CPPSRCS += $(srcdir)/ShowSSEConfig.cpp
CPPSRCS += $(srcdir)/TestBaseProfiler.cpp
CPPSRCS += $(srcdir)/TestPrintf.cpp
COMPUTED_LDFLAGS += -lpthread -Wl,-z,noexecstack -Wl,-z,text -Wl,-z,relro -Wl,-z,nocopyreloc -Wl,-Bsymbolic-functions -Wl,--build-id=sha1 -fstack-protector-strong -fstack-clash-protection -rdynamic -Wl,-rpath-link,/worker/build/obj-x86_64-pc-linux-gnu/dist/bin -Wl,-rpath-link,/usr/local/lib
COMPUTED_CFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DMOZ_HAS_MOZGLUE -I/worker/build/mozglue/tests -I/worker/build/obj-x86_64-pc-linux-gnu/mozglue/tests -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/testing -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -DMOZILLA_CLIENT -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi -fexperimental-new-pass-manager
COMPUTED_CXXFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DMOZ_HAS_MOZGLUE -I/worker/build/mozglue/tests -I/worker/build/obj-x86_64-pc-linux-gnu/mozglue/tests -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/testing -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -DMOZILLA_CLIENT -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -fexperimental-new-pass-manager
COMPUTED_CXX_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables
COMPUTED_C_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi
CPP_UNIT_TESTS += ShowSSEConfig
ShowSSEConfig_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mozglue/tests/ShowSSEConfig.list
ShowSSEConfig: ShowSSEConfig.list
ShowSSEConfig: ShowSSEConfig.o \
    ../build/SSE.o \
    ../build/dummy.o \
    ../../memory/build/Unified_cpp_memory_build0.o \
    ../../memory/mozalloc/cxxalloc.o \
    ../../memory/mozalloc/mozalloc_abort.o \
    ../../memory/mozalloc/Unified_cpp_memory_mozalloc0.o \
    ../baseprofiler/shared-libraries-linux.o \
    ../baseprofiler/Unified_cpp_mozglue_baseprofiler0.o \
    ../baseprofiler/Unified_cpp_mozglue_baseprofiler1.o \
    ../misc/AutoProfilerLabel.o \
    ../misc/ConditionVariable_posix.o \
    ../misc/MmapFaultHandler.o \
    ../misc/Mutex_posix.o \
    ../misc/Printf.o \
    ../misc/StackWalk.o \
    ../misc/TimeStamp.o \
    ../misc/TimeStamp_posix.o \
    ../misc/Uptime.o \
    ../misc/Decimal.o \
    ../../mfbt/lz4.o \
    ../../mfbt/lz4frame.o \
    ../../mfbt/lz4hc.o \
    ../../mfbt/xxhash.o \
    ../../mfbt/Unified_cpp_mfbt0.o \
    ../../mfbt/Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestBaseProfiler
TestBaseProfiler_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mozglue/tests/TestBaseProfiler.list
TestBaseProfiler: TestBaseProfiler.list
TestBaseProfiler: TestBaseProfiler.o \
    ../build/SSE.o \
    ../build/dummy.o \
    ../../memory/build/Unified_cpp_memory_build0.o \
    ../../memory/mozalloc/cxxalloc.o \
    ../../memory/mozalloc/mozalloc_abort.o \
    ../../memory/mozalloc/Unified_cpp_memory_mozalloc0.o \
    ../baseprofiler/shared-libraries-linux.o \
    ../baseprofiler/Unified_cpp_mozglue_baseprofiler0.o \
    ../baseprofiler/Unified_cpp_mozglue_baseprofiler1.o \
    ../misc/AutoProfilerLabel.o \
    ../misc/ConditionVariable_posix.o \
    ../misc/MmapFaultHandler.o \
    ../misc/Mutex_posix.o \
    ../misc/Printf.o \
    ../misc/StackWalk.o \
    ../misc/TimeStamp.o \
    ../misc/TimeStamp_posix.o \
    ../misc/Uptime.o \
    ../misc/Decimal.o \
    ../../mfbt/lz4.o \
    ../../mfbt/lz4frame.o \
    ../../mfbt/lz4hc.o \
    ../../mfbt/xxhash.o \
    ../../mfbt/Unified_cpp_mfbt0.o \
    ../../mfbt/Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestPrintf
TestPrintf_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mozglue/tests/TestPrintf.list
TestPrintf: TestPrintf.list
TestPrintf: TestPrintf.o \
    ../build/SSE.o \
    ../build/dummy.o \
    ../../memory/build/Unified_cpp_memory_build0.o \
    ../../memory/mozalloc/cxxalloc.o \
    ../../memory/mozalloc/mozalloc_abort.o \
    ../../memory/mozalloc/Unified_cpp_memory_mozalloc0.o \
    ../baseprofiler/shared-libraries-linux.o \
    ../baseprofiler/Unified_cpp_mozglue_baseprofiler0.o \
    ../baseprofiler/Unified_cpp_mozglue_baseprofiler1.o \
    ../misc/AutoProfilerLabel.o \
    ../misc/ConditionVariable_posix.o \
    ../misc/MmapFaultHandler.o \
    ../misc/Mutex_posix.o \
    ../misc/Printf.o \
    ../misc/StackWalk.o \
    ../misc/TimeStamp.o \
    ../misc/TimeStamp_posix.o \
    ../misc/Uptime.o \
    ../misc/Decimal.o \
    ../../mfbt/lz4.o \
    ../../mfbt/lz4frame.o \
    ../../mfbt/lz4hc.o \
    ../../mfbt/xxhash.o \
    ../../mfbt/Unified_cpp_mfbt0.o \
    ../../mfbt/Unified_cpp_mfbt1.o
