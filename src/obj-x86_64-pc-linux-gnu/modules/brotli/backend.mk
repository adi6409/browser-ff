# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

DEFINES += -DNDEBUG=1 -DTRIMMED=1
HOST_DEFINES += -DNDEBUG=1 -DTRIMMED=1
HOST_CSRCS += $(srcdir)/common/constants.c
HOST_CSRCS += $(srcdir)/common/context.c
HOST_CSRCS += $(srcdir)/common/dictionary.c
HOST_CSRCS += $(srcdir)/common/platform.c
HOST_CSRCS += $(srcdir)/common/transform.c
HOST_CSRCS += $(srcdir)/dec/bit_reader.c
HOST_CSRCS += $(srcdir)/dec/decode.c
HOST_CSRCS += $(srcdir)/dec/huffman.c
HOST_CSRCS += $(srcdir)/dec/state.c
HOST_CSRCS += $(srcdir)/enc/backward_references.c
HOST_CSRCS += $(srcdir)/enc/backward_references_hq.c
HOST_CSRCS += $(srcdir)/enc/bit_cost.c
HOST_CSRCS += $(srcdir)/enc/block_splitter.c
HOST_CSRCS += $(srcdir)/enc/brotli_bit_stream.c
HOST_CSRCS += $(srcdir)/enc/cluster.c
HOST_CSRCS += $(srcdir)/enc/command.c
HOST_CSRCS += $(srcdir)/enc/compress_fragment.c
HOST_CSRCS += $(srcdir)/enc/compress_fragment_two_pass.c
HOST_CSRCS += $(srcdir)/enc/dictionary_hash.c
HOST_CSRCS += $(srcdir)/enc/encode.c
HOST_CSRCS += $(srcdir)/enc/encoder_dict.c
HOST_CSRCS += $(srcdir)/enc/entropy_encode.c
HOST_CSRCS += $(srcdir)/enc/fast_log.c
HOST_CSRCS += $(srcdir)/enc/histogram.c
HOST_CSRCS += $(srcdir)/enc/literal_cost.c
HOST_CSRCS += $(srcdir)/enc/memory.c
HOST_CSRCS += $(srcdir)/enc/metablock.c
HOST_CSRCS += $(srcdir)/enc/static_dict.c
HOST_CSRCS += $(srcdir)/enc/utf8_util.c
HOST_CSRCS += $(srcdir)/tools/brotli.c

# We build files in 'unified' mode by including several files
# together into a single source file.  This cuts down on
# compilation times and debug information size.
UNIFIED_CSRCS := Unified_c_modules_brotli0.c
CSRCS += $(UNIFIED_CSRCS)
COMPUTED_LDFLAGS += -lpthread -Wl,-z,noexecstack -Wl,-z,text -Wl,-z,relro -Wl,-z,nocopyreloc -Wl,-Bsymbolic-functions -Wl,--build-id=sha1 -fstack-protector-strong -fstack-clash-protection -Wl,-rpath-link,/worker/build/obj-x86_64-pc-linux-gnu/dist/bin -Wl,-rpath-link,/usr/local/lib
COMPUTED_HOST_CFLAGS += -DXP_UNIX -O2 -DNDEBUG=1 -DTRIMMED=1 -I/worker/build/modules/brotli -I/worker/build/obj-x86_64-pc-linux-gnu/modules/brotli -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include
COMPUTED_HOST_CXXFLAGS += -O2 -DNDEBUG=1 -DTRIMMED=1 -I/worker/build/modules/brotli -I/worker/build/obj-x86_64-pc-linux-gnu/modules/brotli -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include
COMPUTED_HOST_CXX_LDFLAGS += -O2
COMPUTED_HOST_C_LDFLAGS += -DXP_UNIX -O2
LIBRARY_NAME := brotli
FORCE_STATIC_LIB := 1
REAL_LIBRARY := libbrotli.a
COMPUTED_CFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -I/worker/build/modules/brotli -I/worker/build/obj-x86_64-pc-linux-gnu/modules/brotli -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -DMOZILLA_CLIENT -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi -DBROTLI_BUILD_PORTABLE -fexperimental-new-pass-manager
COMPUTED_CXXFLAGS += -I/worker/build/obj-x86_64-pc-linux-gnu/dist/stl_wrappers -I/worker/build/obj-x86_64-pc-linux-gnu/dist/system_wrappers -include /worker/build/config/gcc_hidden.h -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -DNDEBUG=1 -DTRIMMED=1 -I/worker/build/modules/brotli -I/worker/build/obj-x86_64-pc-linux-gnu/modules/brotli -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nspr -I/worker/build/obj-x86_64-pc-linux-gnu/dist/include/nss -fPIC -DMOZILLA_CLIENT -include /worker/build/obj-x86_64-pc-linux-gnu/mozilla-config.h -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -fexperimental-new-pass-manager
COMPUTED_CXX_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Woverloaded-virtual -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wwrite-strings -Wno-invalid-offsetof -Wclass-varargs -Wempty-init-stmt -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wc++2a-compat -Wcomma -Wimplicit-fallthrough -Wunused-function -Wunused-variable -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-inline-new-delete -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Wno-psabi -Wno-unknown-warning-option -fno-sized-deallocation -fno-aligned-new -fno-exceptions -fno-strict-aliasing -fno-rtti -ffunction-sections -fdata-sections -fno-exceptions -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables
COMPUTED_C_LDFLAGS += -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fstack-protector-strong -fstack-clash-protection -Qunused-arguments -fno-strict-aliasing -ffunction-sections -fdata-sections -fno-math-errno -pthread -pipe -g -O2 -fomit-frame-pointer -funwind-tables -Qunused-arguments -Wall -Wbitfield-enum-conversion -Wempty-body -Wignored-qualifiers -Wpointer-arith -Wshadow-field-in-constructor-modified -Wsign-compare -Wtype-limits -Wunreachable-code -Wunreachable-code-return -Wclass-varargs -Wfloat-overflow-conversion -Wfloat-zero-conversion -Wloop-analysis -Wno-range-loop-analysis -Wstring-conversion -Wtautological-overlap-compare -Wtautological-unsigned-enum-zero-compare -Wtautological-unsigned-zero-compare -Wno-error=tautological-type-limit-compare -Wno-error=deprecated-declarations -Wno-error=array-bounds -Wno-error=backend-plugin -Wno-error=return-std-move -Wno-error=atomic-alignment -Wno-error=deprecated-copy -Wformat -Wformat-security -Wno-gnu-zero-variadic-macro-arguments -Werror=implicit-function-declaration -Wno-psabi
HOST_PROGRAM = $(DEPTH)/dist/host/bin/brotli
brotli_OBJS := host_constants.o \
    host_context.o \
    host_dictionary.o \
    host_platform.o \
    host_transform.o \
    host_bit_reader.o \
    host_decode.o \
    host_huffman.o \
    host_state.o \
    host_backward_references.o \
    host_backward_references_hq.o \
    host_bit_cost.o \
    host_block_splitter.o \
    host_brotli_bit_stream.o \
    host_cluster.o \
    host_command.o \
    host_compress_fragment.o \
    host_compress_fragment_two_pass.o \
    host_dictionary_hash.o \
    host_encode.o \
    host_encoder_dict.o \
    host_entropy_encode.o \
    host_fast_log.o \
    host_histogram.o \
    host_literal_cost.o \
    host_memory.o \
    host_metablock.o \
    host_static_dict.o \
    host_utf8_util.o \
    host_brotli.o
brotli: host_constants.o \
    host_context.o \
    host_dictionary.o \
    host_platform.o \
    host_transform.o \
    host_bit_reader.o \
    host_decode.o \
    host_huffman.o \
    host_state.o \
    host_backward_references.o \
    host_backward_references_hq.o \
    host_bit_cost.o \
    host_block_splitter.o \
    host_brotli_bit_stream.o \
    host_cluster.o \
    host_command.o \
    host_compress_fragment.o \
    host_compress_fragment_two_pass.o \
    host_dictionary_hash.o \
    host_encode.o \
    host_encoder_dict.o \
    host_entropy_encode.o \
    host_fast_log.o \
    host_histogram.o \
    host_literal_cost.o \
    host_memory.o \
    host_metablock.o \
    host_static_dict.o \
    host_utf8_util.o \
    host_brotli.o
HOST_EXTRA_LIBS += -lm
