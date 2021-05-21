//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/extensions/storage/mozIExtensionStorageArea.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIExtensionStorageArea",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void set (in AUTF8String extensionId, in AUTF8String json, in mozIExtensionStorageCallback callback); */
                    Method {
                        name: "Set",
                        params: &[Param { name: "extensionId", ty: "*const ::nsstring::nsACString" }, Param { name: "json", ty: "*const ::nsstring::nsACString" }, Param { name: "callback", ty: "*const mozIExtensionStorageCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void get (in AUTF8String extensionId, in AUTF8String key, in mozIExtensionStorageCallback callback); */
                    Method {
                        name: "Get",
                        params: &[Param { name: "extensionId", ty: "*const ::nsstring::nsACString" }, Param { name: "key", ty: "*const ::nsstring::nsACString" }, Param { name: "callback", ty: "*const mozIExtensionStorageCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void remove (in AUTF8String extensionId, in AUTF8String key, in mozIExtensionStorageCallback callback); */
                    Method {
                        name: "Remove",
                        params: &[Param { name: "extensionId", ty: "*const ::nsstring::nsACString" }, Param { name: "key", ty: "*const ::nsstring::nsACString" }, Param { name: "callback", ty: "*const mozIExtensionStorageCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void clear (in AUTF8String extensionId, in mozIExtensionStorageCallback callback); */
                    Method {
                        name: "Clear",
                        params: &[Param { name: "extensionId", ty: "*const ::nsstring::nsACString" }, Param { name: "callback", ty: "*const mozIExtensionStorageCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getBytesInUse (in AUTF8String extensionId, in AUTF8String keys, in mozIExtensionStorageCallback callback); */
                    Method {
                        name: "GetBytesInUse",
                        params: &[Param { name: "extensionId", ty: "*const ::nsstring::nsACString" }, Param { name: "keys", ty: "*const ::nsstring::nsACString" }, Param { name: "callback", ty: "*const mozIExtensionStorageCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void takeMigrationInfo (in mozIExtensionStorageCallback callback); */
                    Method {
                        name: "TakeMigrationInfo",
                        params: &[Param { name: "callback", ty: "*const mozIExtensionStorageCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "mozIConfigurableExtensionStorageArea",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void configure (in nsIFile databaseFile, in nsIFile kintoFile); */
                    Method {
                        name: "Configure",
                        params: &[Param { name: "databaseFile", ty: "*const nsIFile" }, Param { name: "kintoFile", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void teardown (in mozIExtensionStorageCallback callback); */
                    Method {
                        name: "Teardown",
                        params: &[Param { name: "callback", ty: "*const mozIExtensionStorageCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "mozISyncedExtensionStorageArea",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void fetchPendingSyncChanges (in mozIExtensionStorageCallback callback); */
                    Method {
                        name: "FetchPendingSyncChanges",
                        params: &[Param { name: "callback", ty: "*const mozIExtensionStorageCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "mozIExtensionStorageListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onChanged (in AUTF8String extensionId, in AUTF8String json); */
                    Method {
                        name: "OnChanged",
                        params: &[Param { name: "extensionId", ty: "*const ::nsstring::nsACString" }, Param { name: "json", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "mozIExtensionStorageCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void handleSuccess (in nsIVariant result); */
                    Method {
                        name: "HandleSuccess",
                        params: &[Param { name: "result", ty: "*const nsIVariant" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void handleError (in nsresult code, in AUTF8String message); */
                    Method {
                        name: "HandleError",
                        params: &[Param { name: "code", ty: "::nserror::nsresult" }, Param { name: "message", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

