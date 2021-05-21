//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/quota/nsIQuotaManagerService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIQuotaManagerService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] nsIQuotaRequest storageName (); */
                    Method {
                        name: "StorageName",
                        params: &[Param { name: "_retval", ty: "*mut*const nsIQuotaRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsIQuotaRequest storageInitialized (); */
                    Method {
                        name: "StorageInitialized",
                        params: &[Param { name: "_retval", ty: "*mut*const nsIQuotaRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsIQuotaRequest temporaryStorageInitialized (); */
                    Method {
                        name: "TemporaryStorageInitialized",
                        params: &[Param { name: "_retval", ty: "*mut*const nsIQuotaRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsIQuotaRequest init (); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "_retval", ty: "*mut*const nsIQuotaRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsIQuotaRequest initTemporaryStorage (); */
                    Method {
                        name: "InitTemporaryStorage",
                        params: &[Param { name: "_retval", ty: "*mut*const nsIQuotaRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsIQuotaRequest initializePersistentOrigin (in nsIPrincipal aPrincipal); */
                    Method {
                        name: "InitializePersistentOrigin",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "_retval", ty: "*mut*const nsIQuotaRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsIQuotaRequest initializeTemporaryOrigin (in ACString aPersistenceType, in nsIPrincipal aPrincipal); */
                    Method {
                        name: "InitializeTemporaryOrigin",
                        params: &[Param { name: "aPersistenceType", ty: "*const ::nsstring::nsACString" }, Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "_retval", ty: "*mut*const nsIQuotaRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsIQuotaUsageRequest getUsage (in nsIQuotaUsageCallback aCallback, [optional] in boolean aGetAll); */
                    Method {
                        name: "GetUsage",
                        params: &[Param { name: "aCallback", ty: "*const nsIQuotaUsageCallback" }, Param { name: "aGetAll", ty: "bool" }, Param { name: "_retval", ty: "*mut*const nsIQuotaUsageRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsIQuotaUsageRequest getUsageForPrincipal (in nsIPrincipal aPrincipal, in nsIQuotaUsageCallback aCallback, [optional] in boolean aFromMemory); */
                    Method {
                        name: "GetUsageForPrincipal",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aCallback", ty: "*const nsIQuotaUsageCallback" }, Param { name: "aFromMemory", ty: "bool" }, Param { name: "_retval", ty: "*mut*const nsIQuotaUsageRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsIQuotaRequest listOrigins (); */
                    Method {
                        name: "ListOrigins",
                        params: &[Param { name: "_retval", ty: "*mut*const nsIQuotaRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsIQuotaRequest clear (); */
                    Method {
                        name: "Clear",
                        params: &[Param { name: "_retval", ty: "*mut*const nsIQuotaRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsIQuotaRequest clearStoragesForOriginAttributesPattern (in AString aPattern); */
                    Method {
                        name: "ClearStoragesForOriginAttributesPattern",
                        params: &[Param { name: "aPattern", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut*const nsIQuotaRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsIQuotaRequest clearStoragesForPrincipal (in nsIPrincipal aPrincipal, [optional] in ACString aPersistenceType, [optional] in AString aClientType, [optional] in boolean aClearAll); */
                    Method {
                        name: "ClearStoragesForPrincipal",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aPersistenceType", ty: "*const ::nsstring::nsACString" }, Param { name: "aClientType", ty: "*const ::nsstring::nsAString" }, Param { name: "aClearAll", ty: "bool" }, Param { name: "_retval", ty: "*mut*const nsIQuotaRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsIQuotaRequest reset (); */
                    Method {
                        name: "Reset",
                        params: &[Param { name: "_retval", ty: "*mut*const nsIQuotaRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsIQuotaRequest resetStoragesForPrincipal (in nsIPrincipal aPrincipal, [optional] in ACString aPersistenceType, [optional] in AString aClientType); */
                    Method {
                        name: "ResetStoragesForPrincipal",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aPersistenceType", ty: "*const ::nsstring::nsACString" }, Param { name: "aClientType", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut*const nsIQuotaRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsIQuotaRequest persisted (in nsIPrincipal aPrincipal); */
                    Method {
                        name: "Persisted",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "_retval", ty: "*mut*const nsIQuotaRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsIQuotaRequest persist (in nsIPrincipal aPrincipal); */
                    Method {
                        name: "Persist",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "_retval", ty: "*mut*const nsIQuotaRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsIQuotaRequest estimate (in nsIPrincipal aPrincipal); */
                    Method {
                        name: "Estimate",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "_retval", ty: "*mut*const nsIQuotaRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

