//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/prefetch/nsIOfflineCacheUpdate.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIOfflineCacheUpdateObserver",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void updateStateChanged (in nsIOfflineCacheUpdate aUpdate, in uint32_t state); */
                    Method {
                        name: "UpdateStateChanged",
                        params: &[Param { name: "aUpdate", ty: "*const nsIOfflineCacheUpdate" }, Param { name: "state", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void applicationCacheAvailable (in nsIApplicationCache applicationCache); */
                    Method {
                        name: "ApplicationCacheAvailable",
                        params: &[Param { name: "applicationCache", ty: "*const nsIApplicationCache" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIOfflineCacheUpdate",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute unsigned short status; */
                    Method {
                        name: "GetStatus",
                        params: &[Param { name: "aStatus", ty: "*mut u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean partial; */
                    Method {
                        name: "GetPartial",
                        params: &[Param { name: "aPartial", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean isUpgrade; */
                    Method {
                        name: "GetIsUpgrade",
                        params: &[Param { name: "aIsUpgrade", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString updateDomain; */
                    Method {
                        name: "GetUpdateDomain",
                        params: &[Param { name: "aUpdateDomain", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIURI manifestURI; */
                    Method {
                        name: "GetManifestURI",
                        params: &[Param { name: "aManifestURI", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIPrincipal loadingPrincipal; */
                    Method {
                        name: "GetLoadingPrincipal",
                        params: &[Param { name: "aLoadingPrincipal", ty: "*mut*const nsIPrincipal" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean succeeded; */
                    Method {
                        name: "GetSucceeded",
                        params: &[Param { name: "aSucceeded", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void init (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in Document aDocument, [optional] in nsIFile aCustomProfileDir); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aManifestURI", ty: "*const nsIURI" }, Param { name: "aDocumentURI", ty: "*const nsIURI" }, Param { name: "aLoadingPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aDocument", ty: "*const libc::c_void" }, Param { name: "aCustomProfileDir", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void initPartial (in nsIURI aManifestURI, in ACString aClientID, in nsIURI aDocumentURI, in nsIPrincipal aPrincipal, in nsICookieJarSettings aCookieJarSettings); */
                    Method {
                        name: "InitPartial",
                        params: &[Param { name: "aManifestURI", ty: "*const nsIURI" }, Param { name: "aClientID", ty: "*const ::nsstring::nsACString" }, Param { name: "aDocumentURI", ty: "*const nsIURI" }, Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aCookieJarSettings", ty: "*const nsICookieJarSettings" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void initForUpdateCheck (in nsIURI aManifestURI, in nsIPrincipal aLoadingPrincipal, in nsIObserver aObserver); */
                    Method {
                        name: "InitForUpdateCheck",
                        params: &[Param { name: "aManifestURI", ty: "*const nsIURI" }, Param { name: "aLoadingPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aObserver", ty: "*const nsIObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addDynamicURI (in nsIURI aURI); */
                    Method {
                        name: "AddDynamicURI",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void schedule (); */
                    Method {
                        name: "Schedule",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void addObserver (in nsIOfflineCacheUpdateObserver aObserver, [optional] in boolean aHoldWeak); */
                    Method {
                        name: "AddObserver",
                        params: &[Param { name: "aObserver", ty: "*const nsIOfflineCacheUpdateObserver" }, Param { name: "aHoldWeak", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeObserver (in nsIOfflineCacheUpdateObserver aObserver); */
                    Method {
                        name: "RemoveObserver",
                        params: &[Param { name: "aObserver", ty: "*const nsIOfflineCacheUpdateObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void cancel (); */
                    Method {
                        name: "Cancel",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint64_t byteProgress; */
                    Method {
                        name: "GetByteProgress",
                        params: &[Param { name: "aByteProgress", ty: "*mut uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIOfflineCacheUpdateService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute unsigned long numUpdates; */
                    Method {
                        name: "GetNumUpdates",
                        params: &[Param { name: "aNumUpdates", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIOfflineCacheUpdate getUpdate (in unsigned long index); */
                    Method {
                        name: "GetUpdate",
                        params: &[Param { name: "index", ty: "u32" }, Param { name: "_retval", ty: "*mut *const nsIOfflineCacheUpdate" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIOfflineCacheUpdate scheduleUpdate (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in mozIDOMWindow aWindow); */
                    Method {
                        name: "ScheduleUpdate",
                        params: &[Param { name: "aManifestURI", ty: "*const nsIURI" }, Param { name: "aDocumentURI", ty: "*const nsIURI" }, Param { name: "aLoadingPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aWindow", ty: "*const mozIDOMWindow" }, Param { name: "_retval", ty: "*mut *const nsIOfflineCacheUpdate" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIOfflineCacheUpdate scheduleAppUpdate (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in nsIFile aProfileDir); */
                    Method {
                        name: "ScheduleAppUpdate",
                        params: &[Param { name: "aManifestURI", ty: "*const nsIURI" }, Param { name: "aDocumentURI", ty: "*const nsIURI" }, Param { name: "aLoadingPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aProfileDir", ty: "*const nsIFile" }, Param { name: "_retval", ty: "*mut *const nsIOfflineCacheUpdate" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void scheduleOnDocumentStop (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in Document aDocument); */
                    Method {
                        name: "ScheduleOnDocumentStop",
                        params: &[Param { name: "aManifestURI", ty: "*const nsIURI" }, Param { name: "aDocumentURI", ty: "*const nsIURI" }, Param { name: "aLoadingPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aDocument", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void checkForUpdate (in nsIURI aManifestURI, in nsIPrincipal aLoadingPrincipal, in nsIObserver aObserver); */
                    Method {
                        name: "CheckForUpdate",
                        params: &[Param { name: "aManifestURI", ty: "*const nsIURI" }, Param { name: "aLoadingPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aObserver", ty: "*const nsIObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean offlineAppAllowed (in nsIPrincipal aPrincipal); */
                    Method {
                        name: "OfflineAppAllowed",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean offlineAppAllowedForURI (in nsIURI aURI); */
                    Method {
                        name: "OfflineAppAllowedForURI",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void allowOfflineApp (in nsIPrincipal aPrincipal); */
                    Method {
                        name: "AllowOfflineApp",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

