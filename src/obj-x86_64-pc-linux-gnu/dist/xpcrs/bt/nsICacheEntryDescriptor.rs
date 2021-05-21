//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache/nsICacheEntryDescriptor.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICacheEntryDescriptor",
            base: Some("nsICacheEntryInfo"),
            methods: Ok(&[
                    /* void setExpirationTime (in uint32_t expirationTime); */
                    Method {
                        name: "SetExpirationTime",
                        params: &[Param { name: "expirationTime", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setDataSize (in unsigned long size); */
                    Method {
                        name: "SetDataSize",
                        params: &[Param { name: "size", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIInputStream openInputStream (in unsigned long offset); */
                    Method {
                        name: "OpenInputStream",
                        params: &[Param { name: "offset", ty: "u32" }, Param { name: "_retval", ty: "*mut*const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIOutputStream openOutputStream (in unsigned long offset); */
                    Method {
                        name: "OpenOutputStream",
                        params: &[Param { name: "offset", ty: "u32" }, Param { name: "_retval", ty: "*mut*const nsIOutputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsISupports cacheElement; */
                    Method {
                        name: "GetCacheElement",
                        params: &[Param { name: "aCacheElement", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetCacheElement",
                        params: &[Param { name: "aCacheElement", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute int64_t predictedDataSize; */
                    Method {
                        name: "GetPredictedDataSize",
                        params: &[Param { name: "aPredictedDataSize", ty: "*mut int64_t" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetPredictedDataSize",
                        params: &[Param { name: "aPredictedDataSize", ty: "int64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsCacheAccessMode accessGranted; */
                    Method {
                        name: "GetAccessGranted",
                        params: &[Param { name: "aAccessGranted", ty: "*mut nsCacheAccessMode" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsCacheStoragePolicy storagePolicy; */
                    Method {
                        name: "GetStoragePolicy",
                        params: &[Param { name: "aStoragePolicy", ty: "*mut nsCacheStoragePolicy" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetStoragePolicy",
                        params: &[Param { name: "aStoragePolicy", ty: "nsCacheStoragePolicy" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIFile file; */
                    Method {
                        name: "GetFile",
                        params: &[Param { name: "aFile", ty: "*mut*const nsIFile" }],
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

                    /* void doom (); */
                    Method {
                        name: "Doom",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void doomAndFailPendingRequests (in nsresult status); */
                    Method {
                        name: "DoomAndFailPendingRequests",
                        params: &[Param { name: "status", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void asyncDoom (in nsICacheListener listener); */
                    Method {
                        name: "AsyncDoom",
                        params: &[Param { name: "listener", ty: "*const nsICacheListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void markValid (); */
                    Method {
                        name: "MarkValid",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void close (); */
                    Method {
                        name: "Close",
                        params: &[],
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

                    /* void visitMetaData (in nsICacheMetaDataVisitor visitor); */
                    Method {
                        name: "VisitMetaData",
                        params: &[Param { name: "visitor", ty: "*const nsICacheMetaDataVisitor" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsICacheMetaDataVisitor",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* boolean visitMetaDataElement (in string key, in string value); */
                    Method {
                        name: "VisitMetaDataElement",
                        params: &[Param { name: "key", ty: "*const libc::c_char" }, Param { name: "value", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

