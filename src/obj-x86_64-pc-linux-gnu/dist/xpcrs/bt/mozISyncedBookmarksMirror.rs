//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/places/mozISyncedBookmarksMirror.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozISyncedBookmarksMirrorProgressListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onFetchLocalTree (in long long took, in long long itemCount, in long long deletedCount, in nsIPropertyBag problems); */
                    Method {
                        name: "OnFetchLocalTree",
                        params: &[Param { name: "took", ty: "i64" }, Param { name: "itemCount", ty: "i64" }, Param { name: "deletedCount", ty: "i64" }, Param { name: "problems", ty: "*const nsIPropertyBag" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onFetchRemoteTree (in long long took, in long long itemCount, in long long deletedCount, in nsIPropertyBag problems); */
                    Method {
                        name: "OnFetchRemoteTree",
                        params: &[Param { name: "took", ty: "i64" }, Param { name: "itemCount", ty: "i64" }, Param { name: "deletedCount", ty: "i64" }, Param { name: "problems", ty: "*const nsIPropertyBag" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onMerge (in long long took, in nsIPropertyBag counts); */
                    Method {
                        name: "OnMerge",
                        params: &[Param { name: "took", ty: "i64" }, Param { name: "counts", ty: "*const nsIPropertyBag" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onApply (in long long took); */
                    Method {
                        name: "OnApply",
                        params: &[Param { name: "took", ty: "i64" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "mozISyncedBookmarksMirrorCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void handleSuccess (in bool result); */
                    Method {
                        name: "HandleSuccess",
                        params: &[Param { name: "result", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void handleError (in nsresult code, in AString message); */
                    Method {
                        name: "HandleError",
                        params: &[Param { name: "code", ty: "::nserror::nsresult" }, Param { name: "message", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "mozISyncedBookmarksMirrorLogger",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute short maxLevel; */
                    Method {
                        name: "GetMaxLevel",
                        params: &[Param { name: "aMaxLevel", ty: "*mut i16" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetMaxLevel",
                        params: &[Param { name: "aMaxLevel", ty: "i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void error (in AString message); */
                    Method {
                        name: "Error",
                        params: &[Param { name: "message", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void warn (in AString message); */
                    Method {
                        name: "Warn",
                        params: &[Param { name: "message", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void debug (in AString message); */
                    Method {
                        name: "Debug",
                        params: &[Param { name: "message", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void trace (in AString message); */
                    Method {
                        name: "Trace",
                        params: &[Param { name: "message", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "mozISyncedBookmarksMerger",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute mozIStorageConnection db; */
                    Method {
                        name: "GetDb",
                        params: &[Param { name: "aDb", ty: "*mut*const mozIStorageConnection" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDb",
                        params: &[Param { name: "aDb", ty: "*const mozIStorageConnection" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute mozIServicesLogSink logger; */
                    Method {
                        name: "GetLogger",
                        params: &[Param { name: "aLogger", ty: "*mut *const mozIServicesLogSink" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetLogger",
                        params: &[Param { name: "aLogger", ty: "*const mozIServicesLogSink" }],
                        ret: "::nserror::nsresult",
                    },

                    /* mozIPlacesPendingOperation merge (in long long localTimeSeconds, in long long remoteTimeSeconds, in Array<AString> weakUploads, in mozISyncedBookmarksMirrorCallback callback); */
                    Method {
                        name: "Merge",
                        params: &[Param { name: "localTimeSeconds", ty: "i64" }, Param { name: "remoteTimeSeconds", ty: "i64" }, Param { name: "weakUploads", ty: "*const thin_vec::ThinVec<::nsstring::nsString>" }, Param { name: "callback", ty: "*const mozISyncedBookmarksMirrorCallback" }, Param { name: "_retval", ty: "*mut*const mozIPlacesPendingOperation" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void reset (); */
                    Method {
                        name: "Reset",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

