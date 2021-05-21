//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache2/nsICacheStorageService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICacheStorageService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsICacheStorage memoryCacheStorage (in nsILoadContextInfo aLoadContextInfo); */
                    Method {
                        name: "MemoryCacheStorage",
                        params: &[Param { name: "aLoadContextInfo", ty: "*const nsILoadContextInfo" }, Param { name: "_retval", ty: "*mut*const nsICacheStorage" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsICacheStorage diskCacheStorage (in nsILoadContextInfo aLoadContextInfo, in bool aLookupAppCache); */
                    Method {
                        name: "DiskCacheStorage",
                        params: &[Param { name: "aLoadContextInfo", ty: "*const nsILoadContextInfo" }, Param { name: "aLookupAppCache", ty: "bool" }, Param { name: "_retval", ty: "*mut*const nsICacheStorage" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsICacheStorage pinningCacheStorage (in nsILoadContextInfo aLoadContextInfo); */
                    Method {
                        name: "PinningCacheStorage",
                        params: &[Param { name: "aLoadContextInfo", ty: "*const nsILoadContextInfo" }, Param { name: "_retval", ty: "*mut*const nsICacheStorage" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsICacheStorage appCacheStorage (in nsILoadContextInfo aLoadContextInfo, in nsIApplicationCache aApplicationCache); */
                    Method {
                        name: "AppCacheStorage",
                        params: &[Param { name: "aLoadContextInfo", ty: "*const nsILoadContextInfo" }, Param { name: "aApplicationCache", ty: "*const nsIApplicationCache" }, Param { name: "_retval", ty: "*mut*const nsICacheStorage" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsICacheStorage synthesizedCacheStorage (in nsILoadContextInfo aLoadContextInfo); */
                    Method {
                        name: "SynthesizedCacheStorage",
                        params: &[Param { name: "aLoadContextInfo", ty: "*const nsILoadContextInfo" }, Param { name: "_retval", ty: "*mut*const nsICacheStorage" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void clearOrigin (in nsIPrincipal aPrincipal); */
                    Method {
                        name: "ClearOrigin",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void clearOriginAttributes (in AString aOriginAttributes); */
                    Method {
                        name: "ClearOriginAttributes",
                        params: &[Param { name: "aOriginAttributes", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void clear (); */
                    Method {
                        name: "Clear",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void purgeFromMemory (in uint32_t aWhat); */
                    Method {
                        name: "PurgeFromMemory",
                        params: &[Param { name: "aWhat", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIEventTarget ioTarget; */
                    Method {
                        name: "GetIoTarget",
                        params: &[Param { name: "aIoTarget", ty: "*mut*const nsIEventTarget" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void asyncGetDiskConsumption (in nsICacheStorageConsumptionObserver aObserver); */
                    Method {
                        name: "AsyncGetDiskConsumption",
                        params: &[Param { name: "aObserver", ty: "*const nsICacheStorageConsumptionObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void asyncVisitAllStorages (in nsICacheStorageVisitor aVisitor, in boolean aVisitEntries); */
                    Method {
                        name: "AsyncVisitAllStorages",
                        params: &[Param { name: "aVisitor", ty: "*const nsICacheStorageVisitor" }, Param { name: "aVisitEntries", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsICacheStorageConsumptionObserver",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onNetworkCacheDiskConsumption (in int64_t aDiskSize); */
                    Method {
                        name: "OnNetworkCacheDiskConsumption",
                        params: &[Param { name: "aDiskSize", ty: "int64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

