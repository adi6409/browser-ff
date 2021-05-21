#define MOZ_UNIFIED_BUILD
#include "/worker/build/dom/media/webrtc/transport/third_party/nICEr/src/util/cb_args.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/media/webrtc/transport/third_party/nICEr/src/util/cb_args.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/media/webrtc/transport/third_party/nICEr/src/util/cb_args.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/media/webrtc/transport/third_party/nICEr/src/util/ice_util.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/media/webrtc/transport/third_party/nICEr/src/util/ice_util.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/media/webrtc/transport/third_party/nICEr/src/util/ice_util.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif