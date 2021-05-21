//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache2/nsICacheEntry.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICacheEntry",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute ACString key; */
                    Method {
                        name: "GetKey",
                        params: &[Param { name: "aKey", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint64_t cacheEntryId; */
                    Method {
                        name: "GetCacheEntryId",
                        params: &[Param { name: "aCacheEntryId", ty: "*mut uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean persistent; */
                    Method {
                        name: "GetPersistent",
                        params: &[Param { name: "aPersistent", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long fetchCount; */
                    Method {
                        name: "GetFetchCount",
                        params: &[Param { name: "aFetchCount", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint32_t lastFetched; */
                    Method {
                        name: "GetLastFetched",
                        params: &[Param { name: "aLastFetched", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint32_t lastModified; */
                    Method {
                        name: "GetLastModified",
                        params: &[Param { name: "aLastModified", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint32_t expirationTime; */
                    Method {
                        name: "GetExpirationTime",
                        params: &[Param { name: "aExpirationTime", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setExpirationTime (in uint32_t expirationTime); */
                    Method {
                        name: "SetExpirationTime",
                        params: &[Param { name: "expirationTime", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint64_t onStartTime; */
                    Method {
                        name: "GetOnStartTime",
                        params: &[Param { name: "aOnStartTime", ty: "*mut uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint64_t onStopTime; */
                    Method {
                        name: "GetOnStopTime",
                        params: &[Param { name: "aOnStopTime", ty: "*mut uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setNetworkTimes (in uint64_t onStartTime, in uint64_t onStopTime); */
                    Method {
                        name: "SetNetworkTimes",
                        params: &[Param { name: "onStartTime", ty: "uint64_t" }, Param { name: "onStopTime", ty: "uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setContentType (in uint8_t contentType); */
                    Method {
                        name: "SetContentType",
                        params: &[Param { name: "contentType", ty: "uint8_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void forceValidFor (in unsigned long aSecondsToTheFuture); */
                    Method {
                        name: "ForceValidFor",
                        params: &[Param { name: "aSecondsToTheFuture", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean isForcedValid; */
                    Method {
                        name: "GetIsForcedValid",
                        params: &[Param { name: "aIsForcedValid", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIInputStream openInputStream (in long long offset); */
                    Method {
                        name: "OpenInputStream",
                        params: &[Param { name: "offset", ty: "i64" }, Param { name: "_retval", ty: "*mut*const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIOutputStream openOutputStream (in long long offset, in long long predictedSize); */
                    Method {
                        name: "OpenOutputStream",
                        params: &[Param { name: "offset", ty: "i64" }, Param { name: "predictedSize", ty: "i64" }, Param { name: "_retval", ty: "*mut*const nsIOutputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsISupports securityInfo; */
                    Method {
                        name: "GetSecurityInfo",
                        params: &[Param { name: "aSecurityInfo", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetSecurityInfo",
                        params: &[Param { name: "aSecurityInfo", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long storageDataSize; */
                    Method {
                        name: "GetStorageDataSize",
                        params: &[Param { name: "aStorageDataSize", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void asyncDoom (in nsICacheEntryDoomCallback listener); */
                    Method {
                        name: "AsyncDoom",
                        params: &[Param { name: "listener", ty: "*const nsICacheEntryDoomCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* string getMetaDataElement (in string key); */
                    Method {
                        name: "GetMetaDataElement",
                        params: &[Param { name: "key", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setMetaDataElement (in string key, in string value); */
                    Method {
                        name: "SetMetaDataElement",
                        params: &[Param { name: "key", ty: "*const libc::c_char" }, Param { name: "value", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void visitMetaData (in nsICacheEntryMetaDataVisitor visitor); */
                    Method {
                        name: "VisitMetaData",
                        params: &[Param { name: "visitor", ty: "*const nsICacheEntryMetaDataVisitor" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void metaDataReady (); */
                    Method {
                        name: "MetaDataReady",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void setValid (); */
                    Method {
                        name: "SetValid",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void dismiss (); */
                    Method {
                        name: "Dismiss",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint32_t diskStorageSizeInKB; */
                    Method {
                        name: "GetDiskStorageSizeInKB",
                        params: &[Param { name: "aDiskStorageSizeInKB", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsICacheEntry recreate ([optional] in boolean aMemoryOnly); */
                    Method {
                        name: "Recreate",
                        params: &[Param { name: "aMemoryOnly", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsICacheEntry" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long long dataSize; */
                    Method {
                        name: "GetDataSize",
                        params: &[Param { name: "aDataSize", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long long altDataSize; */
                    Method {
                        name: "GetAltDataSize",
                        params: &[Param { name: "aAltDataSize", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString altDataType; */
                    Method {
                        name: "GetAltDataType",
                        params: &[Param { name: "aAltDataType", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIAsyncOutputStream openAlternativeOutputStream (in ACString type, in long long predictedSize); */
                    Method {
                        name: "OpenAlternativeOutputStream",
                        params: &[Param { name: "type_", ty: "*const ::nsstring::nsACString" }, Param { name: "predictedSize", ty: "i64" }, Param { name: "_retval", ty: "*mut*const nsIAsyncOutputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIInputStream openAlternativeInputStream (in ACString type); */
                    Method {
                        name: "OpenAlternativeInputStream",
                        params: &[Param { name: "type_", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsILoadContextInfo loadContextInfo; */
                    Method {
                        name: "GetLoadContextInfo",
                        params: &[Param { name: "aLoadContextInfo", ty: "*mut*const nsILoadContextInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void close (); */
                    Method {
                        name: "Close",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void markValid (); */
                    Method {
                        name: "MarkValid",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void maybeMarkValid (); */
                    Method {
                        name: "MaybeMarkValid",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean hasWriteAccess (in boolean aWriteAllowed); */
                    Method {
                        name: "HasWriteAccess",
                        params: &[Param { name: "aWriteAllowed", ty: "bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsICacheEntryMetaDataVisitor",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onMetaDataElement (in string key, in string value); */
                    Method {
                        name: "OnMetaDataElement",
                        params: &[Param { name: "key", ty: "*const libc::c_char" }, Param { name: "value", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

