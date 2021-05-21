//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache/nsICacheService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICacheService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsICacheSession createSession (in string clientID, in nsCacheStoragePolicy storagePolicy, in boolean streamBased); */
                    Method {
                        name: "CreateSession",
                        params: &[Param { name: "clientID", ty: "*const libc::c_char" }, Param { name: "storagePolicy", ty: "nsCacheStoragePolicy" }, Param { name: "streamBased", ty: "bool" }, Param { name: "_retval", ty: "*mut*const nsICacheSession" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void visitEntries (in nsICacheVisitor visitor); */
                    Method {
                        name: "VisitEntries",
                        params: &[Param { name: "visitor", ty: "*const nsICacheVisitor" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void evictEntries (in nsCacheStoragePolicy storagePolicy); */
                    Method {
                        name: "EvictEntries",
                        params: &[Param { name: "storagePolicy", ty: "nsCacheStoragePolicy" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIEventTarget cacheIOTarget; */
                    Method {
                        name: "GetCacheIOTarget",
                        params: &[Param { name: "aCacheIOTarget", ty: "*mut*const nsIEventTarget" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsICacheServiceInternal",
            base: Some("nsICacheService"),
            methods: Ok(&[
                    /* readonly attribute double lockHeldTime; */
                    Method {
                        name: "GetLockHeldTime",
                        params: &[Param { name: "aLockHeldTime", ty: "*mut libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

