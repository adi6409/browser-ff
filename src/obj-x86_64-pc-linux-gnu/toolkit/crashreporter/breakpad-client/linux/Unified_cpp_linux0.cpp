#define MOZ_UNIFIED_BUILD
#include "/worker/build/toolkit/crashreporter/breakpad-client/linux/crash_generation/crash_generation_client.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/crashreporter/breakpad-client/linux/crash_generation/crash_generation_client.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/crashreporter/breakpad-client/linux/crash_generation/crash_generation_client.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/toolkit/crashreporter/breakpad-client/linux/crash_generation/crash_generation_server.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/crashreporter/breakpad-client/linux/crash_generation/crash_generation_server.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/crashreporter/breakpad-client/linux/crash_generation/crash_generation_server.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/toolkit/crashreporter/breakpad-client/linux/dump_writer_common/thread_info.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/crashreporter/breakpad-client/linux/dump_writer_common/thread_info.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/crashreporter/breakpad-client/linux/dump_writer_common/thread_info.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/toolkit/crashreporter/breakpad-client/linux/dump_writer_common/ucontext_reader.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/crashreporter/breakpad-client/linux/dump_writer_common/ucontext_reader.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/crashreporter/breakpad-client/linux/dump_writer_common/ucontext_reader.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/toolkit/crashreporter/breakpad-client/linux/handler/exception_handler.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/crashreporter/breakpad-client/linux/handler/exception_handler.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/crashreporter/breakpad-client/linux/handler/exception_handler.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/toolkit/crashreporter/breakpad-client/linux/handler/guid_generator.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/crashreporter/breakpad-client/linux/handler/guid_generator.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/crashreporter/breakpad-client/linux/handler/guid_generator.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/toolkit/crashreporter/breakpad-client/linux/handler/minidump_descriptor.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/crashreporter/breakpad-client/linux/handler/minidump_descriptor.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/crashreporter/breakpad-client/linux/handler/minidump_descriptor.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/toolkit/crashreporter/breakpad-client/linux/log/log.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/crashreporter/breakpad-client/linux/log/log.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/crashreporter/breakpad-client/linux/log/log.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/toolkit/crashreporter/breakpad-client/linux/microdump_writer/microdump_writer.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/crashreporter/breakpad-client/linux/microdump_writer/microdump_writer.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/crashreporter/breakpad-client/linux/microdump_writer/microdump_writer.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/toolkit/crashreporter/breakpad-client/linux/minidump_writer/linux_dumper.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/crashreporter/breakpad-client/linux/minidump_writer/linux_dumper.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/crashreporter/breakpad-client/linux/minidump_writer/linux_dumper.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/toolkit/crashreporter/breakpad-client/linux/minidump_writer/linux_ptrace_dumper.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/crashreporter/breakpad-client/linux/minidump_writer/linux_ptrace_dumper.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/crashreporter/breakpad-client/linux/minidump_writer/linux_ptrace_dumper.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/toolkit/crashreporter/breakpad-client/linux/minidump_writer/minidump_writer.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/crashreporter/breakpad-client/linux/minidump_writer/minidump_writer.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/crashreporter/breakpad-client/linux/minidump_writer/minidump_writer.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif