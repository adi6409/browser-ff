//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/url-classifier/nsIUrlListManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUrlListManager",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* ACString getGethashUrl (in ACString tableName); */
                    Method {
                        name: "GetGethashUrl",
                        params: &[Param { name: "tableName", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString getUpdateUrl (in ACString tableName); */
                    Method {
                        name: "GetUpdateUrl",
                        params: &[Param { name: "tableName", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean registerTable (in ACString tableName, in ACString providerName, in ACString updateUrl, in ACString gethashUrl); */
                    Method {
                        name: "RegisterTable",
                        params: &[Param { name: "tableName", ty: "*const ::nsstring::nsACString" }, Param { name: "providerName", ty: "*const ::nsstring::nsACString" }, Param { name: "updateUrl", ty: "*const ::nsstring::nsACString" }, Param { name: "gethashUrl", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void unregisterTable (in ACString tableName); */
                    Method {
                        name: "UnregisterTable",
                        params: &[Param { name: "tableName", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void enableUpdate (in ACString tableName); */
                    Method {
                        name: "EnableUpdate",
                        params: &[Param { name: "tableName", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void disableAllUpdates (); */
                    Method {
                        name: "DisableAllUpdates",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void disableUpdate (in ACString tableName); */
                    Method {
                        name: "DisableUpdate",
                        params: &[Param { name: "tableName", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void maybeToggleUpdateChecking (); */
                    Method {
                        name: "MaybeToggleUpdateChecking",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean checkForUpdates (in ACString updateUrl); */
                    Method {
                        name: "CheckForUpdates",
                        params: &[Param { name: "updateUrl", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean forceUpdates (in ACString tableNames); */
                    Method {
                        name: "ForceUpdates",
                        params: &[Param { name: "tableNames", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* uint64_t getBackOffTime (in ACString provider); */
                    Method {
                        name: "GetBackOffTime",
                        params: &[Param { name: "provider", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isRegistered (); */
                    Method {
                        name: "IsRegistered",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

