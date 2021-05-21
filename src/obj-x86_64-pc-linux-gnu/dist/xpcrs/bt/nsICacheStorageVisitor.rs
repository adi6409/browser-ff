//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache2/nsICacheStorageVisitor.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICacheStorageVisitor",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onCacheStorageInfo (in uint32_t aEntryCount, in uint64_t aConsumption, in uint64_t aCapacity, in nsIFile aDiskDirectory); */
                    Method {
                        name: "OnCacheStorageInfo",
                        params: &[Param { name: "aEntryCount", ty: "uint32_t" }, Param { name: "aConsumption", ty: "uint64_t" }, Param { name: "aCapacity", ty: "uint64_t" }, Param { name: "aDiskDirectory", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onCacheEntryInfo (in nsIURI aURI, in ACString aIdEnhance, in int64_t aDataSize, in long aFetchCount, in uint32_t aLastModifiedTime, in uint32_t aExpirationTime, in boolean aPinned, in nsILoadContextInfo aInfo); */
                    Method {
                        name: "OnCacheEntryInfo",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aIdEnhance", ty: "*const ::nsstring::nsACString" }, Param { name: "aDataSize", ty: "int64_t" }, Param { name: "aFetchCount", ty: "i32" }, Param { name: "aLastModifiedTime", ty: "uint32_t" }, Param { name: "aExpirationTime", ty: "uint32_t" }, Param { name: "aPinned", ty: "bool" }, Param { name: "aInfo", ty: "*const nsILoadContextInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onCacheEntryVisitCompleted (); */
                    Method {
                        name: "OnCacheEntryVisitCompleted",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

