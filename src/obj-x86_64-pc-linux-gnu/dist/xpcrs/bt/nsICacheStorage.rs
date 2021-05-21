//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache2/nsICacheStorage.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICacheStorage",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void asyncOpenURI (in nsIURI aURI, in ACString aIdExtension, in uint32_t aFlags, in nsICacheEntryOpenCallback aCallback); */
                    Method {
                        name: "AsyncOpenURI",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aIdExtension", ty: "*const ::nsstring::nsACString" }, Param { name: "aFlags", ty: "uint32_t" }, Param { name: "aCallback", ty: "*const nsICacheEntryOpenCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsICacheEntry openTruncate (in nsIURI aURI, in ACString aIdExtension); */
                    Method {
                        name: "OpenTruncate",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aIdExtension", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsICacheEntry" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean exists (in nsIURI aURI, in ACString aIdExtension); */
                    Method {
                        name: "Exists",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aIdExtension", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getCacheIndexEntryAttrs (in nsIURI aURI, in ACString aIdExtension, out bool aHasAltData, out uint32_t aSizeInKB); */
                    Method {
                        name: "GetCacheIndexEntryAttrs",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aIdExtension", ty: "*const ::nsstring::nsACString" }, Param { name: "aHasAltData", ty: "*mut bool" }, Param { name: "aSizeInKB", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void asyncDoomURI (in nsIURI aURI, in ACString aIdExtension, in nsICacheEntryDoomCallback aCallback); */
                    Method {
                        name: "AsyncDoomURI",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aIdExtension", ty: "*const ::nsstring::nsACString" }, Param { name: "aCallback", ty: "*const nsICacheEntryDoomCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void asyncEvictStorage (in nsICacheEntryDoomCallback aCallback); */
                    Method {
                        name: "AsyncEvictStorage",
                        params: &[Param { name: "aCallback", ty: "*const nsICacheEntryDoomCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void asyncVisitStorage (in nsICacheStorageVisitor aVisitor, in boolean aVisitEntries); */
                    Method {
                        name: "AsyncVisitStorage",
                        params: &[Param { name: "aVisitor", ty: "*const nsICacheStorageVisitor" }, Param { name: "aVisitEntries", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

