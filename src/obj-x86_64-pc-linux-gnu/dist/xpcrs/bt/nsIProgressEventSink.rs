//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIProgressEventSink.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIProgressEventSink",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onProgress (in nsIRequest aRequest, in long long aProgress, in long long aProgressMax); */
                    Method {
                        name: "OnProgress",
                        params: &[Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aProgress", ty: "i64" }, Param { name: "aProgressMax", ty: "i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onStatus (in nsIRequest aRequest, in nsresult aStatus, in wstring aStatusArg); */
                    Method {
                        name: "OnStatus",
                        params: &[Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aStatus", ty: "::nserror::nsresult" }, Param { name: "aStatusArg", ty: "*const i16" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

