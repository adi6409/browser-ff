#define MOZ_UNIFIED_BUILD
#include "/worker/build/toolkit/crashreporter/google-breakpad/src/common/linux/elfutils.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/crashreporter/google-breakpad/src/common/linux/elfutils.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/crashreporter/google-breakpad/src/common/linux/elfutils.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/toolkit/crashreporter/google-breakpad/src/common/linux/http_upload.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/crashreporter/google-breakpad/src/common/linux/http_upload.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/crashreporter/google-breakpad/src/common/linux/http_upload.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/toolkit/crashreporter/google-breakpad/src/common/linux/linux_libc_support.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/crashreporter/google-breakpad/src/common/linux/linux_libc_support.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/crashreporter/google-breakpad/src/common/linux/linux_libc_support.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/toolkit/crashreporter/google-breakpad/src/common/linux/memory_mapped_file.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/crashreporter/google-breakpad/src/common/linux/memory_mapped_file.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/crashreporter/google-breakpad/src/common/linux/memory_mapped_file.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/toolkit/crashreporter/google-breakpad/src/common/linux/safe_readlink.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/crashreporter/google-breakpad/src/common/linux/safe_readlink.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/crashreporter/google-breakpad/src/common/linux/safe_readlink.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif