//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIStorageService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void openAsyncDatabase (in nsIVariant aDatabaseStore, [optional] in nsIPropertyBag2 aOptions, in mozIStorageCompletionCallback aCallback); */
                    Method {
                        name: "OpenAsyncDatabase",
                        params: &[Param { name: "aDatabaseStore", ty: "*const nsIVariant" }, Param { name: "aOptions", ty: "*const nsIPropertyBag2" }, Param { name: "aCallback", ty: "*const mozIStorageCompletionCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* mozIStorageConnection openSpecialDatabase (in ACString aStorageKey, [optional] in ACString aName); */
                    Method {
                        name: "OpenSpecialDatabase",
                        params: &[Param { name: "aStorageKey", ty: "*const ::nsstring::nsACString" }, Param { name: "aName", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const mozIStorageConnection" }],
                        ret: "::nserror::nsresult",
                    },

                    /* mozIStorageConnection openDatabase (in nsIFile aDatabaseFile); */
                    Method {
                        name: "OpenDatabase",
                        params: &[Param { name: "aDatabaseFile", ty: "*const nsIFile" }, Param { name: "_retval", ty: "*mut*const mozIStorageConnection" }],
                        ret: "::nserror::nsresult",
                    },

                    /* mozIStorageConnection openUnsharedDatabase (in nsIFile aDatabaseFile); */
                    Method {
                        name: "OpenUnsharedDatabase",
                        params: &[Param { name: "aDatabaseFile", ty: "*const nsIFile" }, Param { name: "_retval", ty: "*mut*const mozIStorageConnection" }],
                        ret: "::nserror::nsresult",
                    },

                    /* mozIStorageConnection openDatabaseWithFileURL (in nsIFileURL aFileURL, [optional] in ACString aTelemetryFilename); */
                    Method {
                        name: "OpenDatabaseWithFileURL",
                        params: &[Param { name: "aFileURL", ty: "*const nsIFileURL" }, Param { name: "aTelemetryFilename", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const mozIStorageConnection" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIFile backupDatabaseFile (in nsIFile aDBFile, in AString aBackupFileName, [optional] in nsIFile aBackupParentDirectory); */
                    Method {
                        name: "BackupDatabaseFile",
                        params: &[Param { name: "aDBFile", ty: "*const nsIFile" }, Param { name: "aBackupFileName", ty: "*const ::nsstring::nsAString" }, Param { name: "aBackupParentDirectory", ty: "*const nsIFile" }, Param { name: "_retval", ty: "*mut*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

