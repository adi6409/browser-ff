//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache/nsICacheSession.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICacheSession",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute boolean doomEntriesIfExpired; */
                    Method {
                        name: "GetDoomEntriesIfExpired",
                        params: &[Param { name: "aDoomEntriesIfExpired", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDoomEntriesIfExpired",
                        params: &[Param { name: "aDoomEntriesIfExpired", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIFile profileDirectory; */
                    Method {
                        name: "GetProfileDirectory",
                        params: &[Param { name: "aProfileDirectory", ty: "*mut*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetProfileDirectory",
                        params: &[Param { name: "aProfileDirectory", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsICacheEntryDescriptor openCacheEntry (in ACString key, in nsCacheAccessMode accessRequested, in boolean blockingMode); */
                    Method {
                        name: "OpenCacheEntry",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsACString" }, Param { name: "accessRequested", ty: "nsCacheAccessMode" }, Param { name: "blockingMode", ty: "bool" }, Param { name: "_retval", ty: "*mut*const nsICacheEntryDescriptor" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void asyncOpenCacheEntry (in ACString key, in nsCacheAccessMode accessRequested, in nsICacheListener listener, [optional] in boolean noWait); */
                    Method {
                        name: "AsyncOpenCacheEntry",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsACString" }, Param { name: "accessRequested", ty: "nsCacheAccessMode" }, Param { name: "listener", ty: "*const nsICacheListener" }, Param { name: "noWait", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void evictEntries (); */
                    Method {
                        name: "EvictEntries",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isStorageEnabled (); */
                    Method {
                        name: "IsStorageEnabled",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void doomEntry (in ACString key, in nsICacheListener listener); */
                    Method {
                        name: "DoomEntry",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsACString" }, Param { name: "listener", ty: "*const nsICacheListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean isPrivate; */
                    Method {
                        name: "GetIsPrivate",
                        params: &[Param { name: "aIsPrivate", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetIsPrivate",
                        params: &[Param { name: "aIsPrivate", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

