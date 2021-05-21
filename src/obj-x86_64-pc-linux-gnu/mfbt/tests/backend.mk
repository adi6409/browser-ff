# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1 -DIMPL_MFBT
DIRS := gtest
CPPSRCS += $(srcdir)/TestAlgorithm.cpp
CPPSRCS += $(srcdir)/TestArray.cpp
CPPSRCS += $(srcdir)/TestArrayUtils.cpp
CPPSRCS += $(srcdir)/TestAtomicBitfields.cpp
CPPSRCS += $(srcdir)/TestAtomics.cpp
CPPSRCS += $(srcdir)/TestBinarySearch.cpp
CPPSRCS += $(srcdir)/TestBitSet.cpp
CPPSRCS += $(srcdir)/TestBloomFilter.cpp
CPPSRCS += $(srcdir)/TestBufferList.cpp
CPPSRCS += $(srcdir)/TestCasting.cpp
CPPSRCS += $(srcdir)/TestCeilingFloor.cpp
CPPSRCS += $(srcdir)/TestCheckedInt.cpp
CPPSRCS += $(srcdir)/TestCompactPair.cpp
CPPSRCS += $(srcdir)/TestCountPopulation.cpp
CPPSRCS += $(srcdir)/TestCountZeroes.cpp
CPPSRCS += $(srcdir)/TestDefineEnum.cpp
CPPSRCS += $(srcdir)/TestDoublyLinkedList.cpp
CPPSRCS += $(srcdir)/TestEndian.cpp
CPPSRCS += $(srcdir)/TestEnumSet.cpp
CPPSRCS += $(srcdir)/TestEnumTypeTraits.cpp
CPPSRCS += $(srcdir)/TestEnumeratedArray.cpp
CPPSRCS += $(srcdir)/TestFastBernoulliTrial.cpp
CPPSRCS += $(srcdir)/TestFloatingPoint.cpp
CPPSRCS += $(srcdir)/TestFunctionRef.cpp
CPPSRCS += $(srcdir)/TestFunctionTypeTraits.cpp
CPPSRCS += $(srcdir)/TestHashTable.cpp
CPPSRCS += $(srcdir)/TestIntegerPrintfMacros.cpp
CPPSRCS += $(srcdir)/TestIntegerRange.cpp
CPPSRCS += $(srcdir)/TestJSONWriter.cpp
CPPSRCS += $(srcdir)/TestLinkedList.cpp
CPPSRCS += $(srcdir)/TestMacroArgs.cpp
CPPSRCS += $(srcdir)/TestMacroForEach.cpp
CPPSRCS += $(srcdir)/TestMathAlgorithms.cpp
CPPSRCS += $(srcdir)/TestMaybe.cpp
CPPSRCS += $(srcdir)/TestNonDereferenceable.cpp
CPPSRCS += $(srcdir)/TestNotNull.cpp
CPPSRCS += $(srcdir)/TestPoisonArea.cpp
CPPSRCS += $(srcdir)/TestRandomNum.cpp
CPPSRCS += $(srcdir)/TestRange.cpp
CPPSRCS += $(srcdir)/TestRefPtr.cpp
CPPSRCS += $(srcdir)/TestResult.cpp
CPPSRCS += $(srcdir)/TestRollingMean.cpp
CPPSRCS += $(srcdir)/TestSHA1.cpp
CPPSRCS += $(srcdir)/TestSPSCQueue.cpp
CPPSRCS += $(srcdir)/TestSaturate.cpp
CPPSRCS += $(srcdir)/TestScopeExit.cpp
CPPSRCS += $(srcdir)/TestSegmentedVector.cpp
CPPSRCS += $(srcdir)/TestSmallPointerArray.cpp
CPPSRCS += $(srcdir)/TestSplayTree.cpp
CPPSRCS += $(srcdir)/TestTemplateLib.cpp
CPPSRCS += $(srcdir)/TestTextUtils.cpp
CPPSRCS += $(srcdir)/TestThreadSafeWeakPtr.cpp
CPPSRCS += $(srcdir)/TestTuple.cpp
CPPSRCS += $(srcdir)/TestTypeTraits.cpp
CPPSRCS += $(srcdir)/TestTypedEnum.cpp
CPPSRCS += $(srcdir)/TestUniquePtr.cpp
CPPSRCS += $(srcdir)/TestUtf8.cpp
CPPSRCS += $(srcdir)/TestVariant.cpp
CPPSRCS += $(srcdir)/TestVector.cpp
CPPSRCS += $(srcdir)/TestWeakPtr.cpp
CPPSRCS += $(srcdir)/TestWrappingOperations.cpp
CPPSRCS += $(srcdir)/TestXorShift128PlusRNG.cpp
COMPUTED_LDFLAGS += -lpthread -Wl,-z,noexecstack -Wl,-z,text -Wl,-z,relro -Wl,-z,nocopyreloc -Wl,-Bsymbolic-functions -Wl,--build-id=sha1 -fstack-protector-strong -fstack-clash-protection -Wl,-rpath-link,/worker/build/obj-x86_64-pc-linux-gnu/dist/bin -Wl,-rpath-link,/usr/local/lib
COMPUTED_CFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DIMPL_MFBT -I/worker/build/mfbt/tests -I/worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/testing -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -DMOZILLA_CLIENT -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi -fexperimental-new-pass-manager
COMPUTED_CXXFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -DIMPL_MFBT -I/worker/build/mfbt/tests -I/worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/testing -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -DMOZILLA_CLIENT -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Wno-error=shadow -fexperimental-new-pass-manager
COMPUTED_CXX_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables
COMPUTED_C_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi
CPP_UNIT_TESTS += TestAlgorithm
TestAlgorithm_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestAlgorithm.list
TestAlgorithm: TestAlgorithm.list
TestAlgorithm: TestAlgorithm.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestArray
TestArray_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestArray.list
TestArray: TestArray.list
TestArray: TestArray.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestArrayUtils
TestArrayUtils_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestArrayUtils.list
TestArrayUtils: TestArrayUtils.list
TestArrayUtils: TestArrayUtils.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestAtomicBitfields
TestAtomicBitfields_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestAtomicBitfields.list
TestAtomicBitfields: TestAtomicBitfields.list
TestAtomicBitfields: TestAtomicBitfields.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestAtomics
TestAtomics_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestAtomics.list
TestAtomics: TestAtomics.list
TestAtomics: TestAtomics.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestBinarySearch
TestBinarySearch_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestBinarySearch.list
TestBinarySearch: TestBinarySearch.list
TestBinarySearch: TestBinarySearch.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestBitSet
TestBitSet_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestBitSet.list
TestBitSet: TestBitSet.list
TestBitSet: TestBitSet.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestBloomFilter
TestBloomFilter_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestBloomFilter.list
TestBloomFilter: TestBloomFilter.list
TestBloomFilter: TestBloomFilter.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestBufferList
TestBufferList_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestBufferList.list
TestBufferList: TestBufferList.list
TestBufferList: TestBufferList.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestCasting
TestCasting_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestCasting.list
TestCasting: TestCasting.list
TestCasting: TestCasting.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestCeilingFloor
TestCeilingFloor_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestCeilingFloor.list
TestCeilingFloor: TestCeilingFloor.list
TestCeilingFloor: TestCeilingFloor.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestCheckedInt
TestCheckedInt_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestCheckedInt.list
TestCheckedInt: TestCheckedInt.list
TestCheckedInt: TestCheckedInt.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestCompactPair
TestCompactPair_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestCompactPair.list
TestCompactPair: TestCompactPair.list
TestCompactPair: TestCompactPair.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestCountPopulation
TestCountPopulation_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestCountPopulation.list
TestCountPopulation: TestCountPopulation.list
TestCountPopulation: TestCountPopulation.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestCountZeroes
TestCountZeroes_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestCountZeroes.list
TestCountZeroes: TestCountZeroes.list
TestCountZeroes: TestCountZeroes.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestDefineEnum
TestDefineEnum_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestDefineEnum.list
TestDefineEnum: TestDefineEnum.list
TestDefineEnum: TestDefineEnum.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestDoublyLinkedList
TestDoublyLinkedList_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestDoublyLinkedList.list
TestDoublyLinkedList: TestDoublyLinkedList.list
TestDoublyLinkedList: TestDoublyLinkedList.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestEndian
TestEndian_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestEndian.list
TestEndian: TestEndian.list
TestEndian: TestEndian.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestEnumeratedArray
TestEnumeratedArray_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestEnumeratedArray.list
TestEnumeratedArray: TestEnumeratedArray.list
TestEnumeratedArray: TestEnumeratedArray.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestEnumSet
TestEnumSet_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestEnumSet.list
TestEnumSet: TestEnumSet.list
TestEnumSet: TestEnumSet.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestEnumTypeTraits
TestEnumTypeTraits_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestEnumTypeTraits.list
TestEnumTypeTraits: TestEnumTypeTraits.list
TestEnumTypeTraits: TestEnumTypeTraits.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestFastBernoulliTrial
TestFastBernoulliTrial_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestFastBernoulliTrial.list
TestFastBernoulliTrial: TestFastBernoulliTrial.list
TestFastBernoulliTrial: TestFastBernoulliTrial.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestFloatingPoint
TestFloatingPoint_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestFloatingPoint.list
TestFloatingPoint: TestFloatingPoint.list
TestFloatingPoint: TestFloatingPoint.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestFunctionRef
TestFunctionRef_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestFunctionRef.list
TestFunctionRef: TestFunctionRef.list
TestFunctionRef: TestFunctionRef.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestFunctionTypeTraits
TestFunctionTypeTraits_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestFunctionTypeTraits.list
TestFunctionTypeTraits: TestFunctionTypeTraits.list
TestFunctionTypeTraits: TestFunctionTypeTraits.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestHashTable
TestHashTable_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestHashTable.list
TestHashTable: TestHashTable.list
TestHashTable: TestHashTable.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestIntegerPrintfMacros
TestIntegerPrintfMacros_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestIntegerPrintfMacros.list
TestIntegerPrintfMacros: TestIntegerPrintfMacros.list
TestIntegerPrintfMacros: TestIntegerPrintfMacros.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestIntegerRange
TestIntegerRange_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestIntegerRange.list
TestIntegerRange: TestIntegerRange.list
TestIntegerRange: TestIntegerRange.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestJSONWriter
TestJSONWriter_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestJSONWriter.list
TestJSONWriter: TestJSONWriter.list
TestJSONWriter: TestJSONWriter.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestLinkedList
TestLinkedList_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestLinkedList.list
TestLinkedList: TestLinkedList.list
TestLinkedList: TestLinkedList.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestMacroArgs
TestMacroArgs_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestMacroArgs.list
TestMacroArgs: TestMacroArgs.list
TestMacroArgs: TestMacroArgs.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestMacroForEach
TestMacroForEach_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestMacroForEach.list
TestMacroForEach: TestMacroForEach.list
TestMacroForEach: TestMacroForEach.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestMathAlgorithms
TestMathAlgorithms_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestMathAlgorithms.list
TestMathAlgorithms: TestMathAlgorithms.list
TestMathAlgorithms: TestMathAlgorithms.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestMaybe
TestMaybe_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestMaybe.list
TestMaybe: TestMaybe.list
TestMaybe: TestMaybe.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestNonDereferenceable
TestNonDereferenceable_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestNonDereferenceable.list
TestNonDereferenceable: TestNonDereferenceable.list
TestNonDereferenceable: TestNonDereferenceable.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestNotNull
TestNotNull_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestNotNull.list
TestNotNull: TestNotNull.list
TestNotNull: TestNotNull.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestRandomNum
TestRandomNum_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestRandomNum.list
TestRandomNum: TestRandomNum.list
TestRandomNum: TestRandomNum.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestRange
TestRange_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestRange.list
TestRange: TestRange.list
TestRange: TestRange.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestRefPtr
TestRefPtr_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestRefPtr.list
TestRefPtr: TestRefPtr.list
TestRefPtr: TestRefPtr.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestResult
TestResult_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestResult.list
TestResult: TestResult.list
TestResult: TestResult.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestRollingMean
TestRollingMean_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestRollingMean.list
TestRollingMean: TestRollingMean.list
TestRollingMean: TestRollingMean.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestSaturate
TestSaturate_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestSaturate.list
TestSaturate: TestSaturate.list
TestSaturate: TestSaturate.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestScopeExit
TestScopeExit_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestScopeExit.list
TestScopeExit: TestScopeExit.list
TestScopeExit: TestScopeExit.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestSegmentedVector
TestSegmentedVector_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestSegmentedVector.list
TestSegmentedVector: TestSegmentedVector.list
TestSegmentedVector: TestSegmentedVector.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestSHA1
TestSHA1_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestSHA1.list
TestSHA1: TestSHA1.list
TestSHA1: TestSHA1.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestSmallPointerArray
TestSmallPointerArray_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestSmallPointerArray.list
TestSmallPointerArray: TestSmallPointerArray.list
TestSmallPointerArray: TestSmallPointerArray.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestSplayTree
TestSplayTree_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestSplayTree.list
TestSplayTree: TestSplayTree.list
TestSplayTree: TestSplayTree.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestSPSCQueue
TestSPSCQueue_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestSPSCQueue.list
TestSPSCQueue: TestSPSCQueue.list
TestSPSCQueue: TestSPSCQueue.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestTemplateLib
TestTemplateLib_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestTemplateLib.list
TestTemplateLib: TestTemplateLib.list
TestTemplateLib: TestTemplateLib.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestTextUtils
TestTextUtils_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestTextUtils.list
TestTextUtils: TestTextUtils.list
TestTextUtils: TestTextUtils.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestThreadSafeWeakPtr
TestThreadSafeWeakPtr_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestThreadSafeWeakPtr.list
TestThreadSafeWeakPtr: TestThreadSafeWeakPtr.list
TestThreadSafeWeakPtr: TestThreadSafeWeakPtr.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestTuple
TestTuple_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestTuple.list
TestTuple: TestTuple.list
TestTuple: TestTuple.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestTypedEnum
TestTypedEnum_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestTypedEnum.list
TestTypedEnum: TestTypedEnum.list
TestTypedEnum: TestTypedEnum.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestTypeTraits
TestTypeTraits_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestTypeTraits.list
TestTypeTraits: TestTypeTraits.list
TestTypeTraits: TestTypeTraits.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestUniquePtr
TestUniquePtr_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestUniquePtr.list
TestUniquePtr: TestUniquePtr.list
TestUniquePtr: TestUniquePtr.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestVariant
TestVariant_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestVariant.list
TestVariant: TestVariant.list
TestVariant: TestVariant.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestVector
TestVector_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestVector.list
TestVector: TestVector.list
TestVector: TestVector.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestWeakPtr
TestWeakPtr_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestWeakPtr.list
TestWeakPtr: TestWeakPtr.list
TestWeakPtr: TestWeakPtr.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestWrappingOperations
TestWrappingOperations_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestWrappingOperations.list
TestWrappingOperations: TestWrappingOperations.list
TestWrappingOperations: TestWrappingOperations.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestXorShift128PlusRNG
TestXorShift128PlusRNG_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestXorShift128PlusRNG.list
TestXorShift128PlusRNG: TestXorShift128PlusRNG.list
TestXorShift128PlusRNG: TestXorShift128PlusRNG.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestUtf8
TestUtf8_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestUtf8.list
TestUtf8: TestUtf8.list
TestUtf8: TestUtf8.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
CPP_UNIT_TESTS += TestPoisonArea
TestPoisonArea_OBJS := /worker/build/obj-x86_64-pc-linux-gnu/mfbt/tests/TestPoisonArea.list
TestPoisonArea: TestPoisonArea.list
TestPoisonArea: TestPoisonArea.o \
    ../lz4.o \
    ../lz4frame.o \
    ../lz4hc.o \
    ../xxhash.o \
    ../Unified_cpp_mfbt0.o \
    ../Unified_cpp_mfbt1.o
