//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/url-classifier/nsIUrlClassifierDBService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUrlClassifierCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void handleEvent (in ACString value); */
                    Method {
                        name: "HandleEvent",
                        params: &[Param { name: "value", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIUrlClassifierUpdateObserver",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void updateUrlRequested (in ACString url, in ACString table); */
                    Method {
                        name: "UpdateUrlRequested",
                        params: &[Param { name: "url", ty: "*const ::nsstring::nsACString" }, Param { name: "table", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void streamFinished (in nsresult status, in unsigned long delay); */
                    Method {
                        name: "StreamFinished",
                        params: &[Param { name: "status", ty: "::nserror::nsresult" }, Param { name: "delay", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void updateError (in nsresult error); */
                    Method {
                        name: "UpdateError",
                        params: &[Param { name: "error", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void updateSuccess (in unsigned long requestedTimeout); */
                    Method {
                        name: "UpdateSuccess",
                        params: &[Param { name: "requestedTimeout", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIUrlClassifierDBService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void lookup (in nsIPrincipal principal, in ACString tables, in nsIUrlClassifierCallback c); */
                    Method {
                        name: "Lookup",
                        params: &[Param { name: "principal", ty: "*const nsIPrincipal" }, Param { name: "tables", ty: "*const ::nsstring::nsACString" }, Param { name: "c", ty: "*const nsIUrlClassifierCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getTables (in nsIUrlClassifierCallback c); */
                    Method {
                        name: "GetTables",
                        params: &[Param { name: "c", ty: "*const nsIUrlClassifierCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setHashCompleter (in ACString tableName, in nsIUrlClassifierHashCompleter completer); */
                    Method {
                        name: "SetHashCompleter",
                        params: &[Param { name: "tableName", ty: "*const ::nsstring::nsACString" }, Param { name: "completer", ty: "*const nsIUrlClassifierHashCompleter" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void clearLastResults (); */
                    Method {
                        name: "ClearLastResults",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void beginUpdate (in nsIUrlClassifierUpdateObserver updater, in ACString tables); */
                    Method {
                        name: "BeginUpdate",
                        params: &[Param { name: "updater", ty: "*const nsIUrlClassifierUpdateObserver" }, Param { name: "tables", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void beginStream (in ACString table); */
                    Method {
                        name: "BeginStream",
                        params: &[Param { name: "table", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void updateStream (in ACString updateChunk); */
                    Method {
                        name: "UpdateStream",
                        params: &[Param { name: "updateChunk", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void finishStream (); */
                    Method {
                        name: "FinishStream",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void finishUpdate (); */
                    Method {
                        name: "FinishUpdate",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void cancelUpdate (); */
                    Method {
                        name: "CancelUpdate",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void resetDatabase (); */
                    Method {
                        name: "ResetDatabase",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void reloadDatabase (); */
                    Method {
                        name: "ReloadDatabase",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void clearCache (); */
                    Method {
                        name: "ClearCache",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIUrlClassifierLookupCallback",
            base: Some("nsISupports"),
            methods: Err("Rust only supports [ref] / [ptr] native types"),
        },

        Interface {
            name: "nsIUrlClassifierClassifyCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void handleResult (in ACString aList, in ACString aPrefix); */
                    Method {
                        name: "HandleResult",
                        params: &[Param { name: "aList", ty: "*const ::nsstring::nsACString" }, Param { name: "aPrefix", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

