//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIApplicationCache.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIApplicationCacheNamespace",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void init (in unsigned long itemType, in ACString namespaceSpec, in ACString data); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "itemType", ty: "u32" }, Param { name: "namespaceSpec", ty: "*const ::nsstring::nsACString" }, Param { name: "data", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long itemType; */
                    Method {
                        name: "GetItemType",
                        params: &[Param { name: "aItemType", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString namespaceSpec; */
                    Method {
                        name: "GetNamespaceSpec",
                        params: &[Param { name: "aNamespaceSpec", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString data; */
                    Method {
                        name: "GetData",
                        params: &[Param { name: "aData", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIApplicationCache",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void initAsHandle (in ACString groupId, in ACString clientId); */
                    Method {
                        name: "InitAsHandle",
                        params: &[Param { name: "groupId", ty: "*const ::nsstring::nsACString" }, Param { name: "clientId", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIURI manifestURI; */
                    Method {
                        name: "GetManifestURI",
                        params: &[Param { name: "aManifestURI", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString groupID; */
                    Method {
                        name: "GetGroupID",
                        params: &[Param { name: "aGroupID", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString clientID; */
                    Method {
                        name: "GetClientID",
                        params: &[Param { name: "aClientID", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean active; */
                    Method {
                        name: "GetActive",
                        params: &[Param { name: "aActive", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long usage; */
                    Method {
                        name: "GetUsage",
                        params: &[Param { name: "aUsage", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void activate (); */
                    Method {
                        name: "Activate",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void discard (); */
                    Method {
                        name: "Discard",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void markEntry (in ACString key, in unsigned long typeBits); */
                    Method {
                        name: "MarkEntry",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsACString" }, Param { name: "typeBits", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void unmarkEntry (in ACString key, in unsigned long typeBits); */
                    Method {
                        name: "UnmarkEntry",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsACString" }, Param { name: "typeBits", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* unsigned long getTypes (in ACString key); */
                    Method {
                        name: "GetTypes",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<ACString> gatherEntries (in uint32_t typeBits); */
                    Method {
                        name: "GatherEntries",
                        params: &[Param { name: "typeBits", ty: "uint32_t" }, Param { name: "_retval", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addNamespaces (in nsIArray namespaces); */
                    Method {
                        name: "AddNamespaces",
                        params: &[Param { name: "namespaces", ty: "*const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIApplicationCacheNamespace getMatchingNamespace (in ACString key); */
                    Method {
                        name: "GetMatchingNamespace",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut *const nsIApplicationCacheNamespace" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIFile profileDirectory; */
                    Method {
                        name: "GetProfileDirectory",
                        params: &[Param { name: "aProfileDirectory", ty: "*mut*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

