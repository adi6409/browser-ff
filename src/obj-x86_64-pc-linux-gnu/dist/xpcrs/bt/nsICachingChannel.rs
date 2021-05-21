//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsICachingChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICachingChannel",
            base: Some("nsICacheInfoChannel"),
            methods: Ok(&[
                    /* attribute nsISupports cacheToken; */
                    Method {
                        name: "GetCacheToken",
                        params: &[Param { name: "aCacheToken", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetCacheToken",
                        params: &[Param { name: "aCacheToken", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsISupports offlineCacheToken; */
                    Method {
                        name: "GetOfflineCacheToken",
                        params: &[Param { name: "aOfflineCacheToken", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetOfflineCacheToken",
                        params: &[Param { name: "aOfflineCacheToken", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean cacheOnlyMetadata; */
                    Method {
                        name: "GetCacheOnlyMetadata",
                        params: &[Param { name: "aCacheOnlyMetadata", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetCacheOnlyMetadata",
                        params: &[Param { name: "aCacheOnlyMetadata", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean pin; */
                    Method {
                        name: "GetPin",
                        params: &[Param { name: "aPin", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetPin",
                        params: &[Param { name: "aPin", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void forceCacheEntryValidFor (in unsigned long aSecondsToTheFuture); */
                    Method {
                        name: "ForceCacheEntryValidFor",
                        params: &[Param { name: "aSecondsToTheFuture", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

