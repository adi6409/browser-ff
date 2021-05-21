//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIApplicationCacheService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIApplicationCacheService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* ACString buildGroupIDForInfo (in nsIURI aManifestURL, in nsILoadContextInfo aLoadContextInfo); */
                    Method {
                        name: "BuildGroupIDForInfo",
                        params: &[Param { name: "aManifestURL", ty: "*const nsIURI" }, Param { name: "aLoadContextInfo", ty: "*const nsILoadContextInfo" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString buildGroupIDForSuffix (in nsIURI aManifestURL, in ACString aOriginSuffix); */
                    Method {
                        name: "BuildGroupIDForSuffix",
                        params: &[Param { name: "aManifestURL", ty: "*const nsIURI" }, Param { name: "aOriginSuffix", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIApplicationCache createApplicationCache (in ACString group); */
                    Method {
                        name: "CreateApplicationCache",
                        params: &[Param { name: "group", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIApplicationCache" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIApplicationCache createCustomApplicationCache (in ACString group, in nsIFile profileDir, in int32_t quota); */
                    Method {
                        name: "CreateCustomApplicationCache",
                        params: &[Param { name: "group", ty: "*const ::nsstring::nsACString" }, Param { name: "profileDir", ty: "*const nsIFile" }, Param { name: "quota", ty: "int32_t" }, Param { name: "_retval", ty: "*mut*const nsIApplicationCache" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIApplicationCache getApplicationCache (in ACString clientID); */
                    Method {
                        name: "GetApplicationCache",
                        params: &[Param { name: "clientID", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIApplicationCache" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIApplicationCache getActiveCache (in ACString group); */
                    Method {
                        name: "GetActiveCache",
                        params: &[Param { name: "group", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIApplicationCache" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void deactivateGroup (in ACString group); */
                    Method {
                        name: "DeactivateGroup",
                        params: &[Param { name: "group", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void evict (in nsILoadContextInfo aLoadContextInfo); */
                    Method {
                        name: "Evict",
                        params: &[Param { name: "aLoadContextInfo", ty: "*const nsILoadContextInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void evictMatchingOriginAttributes (in AString aPattern); */
                    Method {
                        name: "EvictMatchingOriginAttributes",
                        params: &[Param { name: "aPattern", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIApplicationCache chooseApplicationCache (in ACString key, [optional] in nsILoadContextInfo aLoadContextInfo); */
                    Method {
                        name: "ChooseApplicationCache",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsACString" }, Param { name: "aLoadContextInfo", ty: "*const nsILoadContextInfo" }, Param { name: "_retval", ty: "*mut*const nsIApplicationCache" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void cacheOpportunistically (in nsIApplicationCache cache, in ACString key); */
                    Method {
                        name: "CacheOpportunistically",
                        params: &[Param { name: "cache", ty: "*const nsIApplicationCache" }, Param { name: "key", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<ACString> getGroups (); */
                    Method {
                        name: "GetGroups",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<ACString> getGroupsTimeOrdered (); */
                    Method {
                        name: "GetGroupsTimeOrdered",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

