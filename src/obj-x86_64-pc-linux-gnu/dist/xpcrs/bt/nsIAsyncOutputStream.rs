//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIAsyncOutputStream.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAsyncOutputStream",
            base: Some("nsIOutputStream"),
            methods: Ok(&[
                    /* void closeWithStatus (in nsresult reason); */
                    Method {
                        name: "CloseWithStatus",
                        params: &[Param { name: "reason", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void asyncWait (in nsIOutputStreamCallback aCallback, in unsigned long aFlags, in unsigned long aRequestedCount, in nsIEventTarget aEventTarget); */
                    Method {
                        name: "AsyncWait",
                        params: &[Param { name: "aCallback", ty: "*const nsIOutputStreamCallback" }, Param { name: "aFlags", ty: "u32" }, Param { name: "aRequestedCount", ty: "u32" }, Param { name: "aEventTarget", ty: "*const nsIEventTarget" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIOutputStreamCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onOutputStreamReady (in nsIAsyncOutputStream aStream); */
                    Method {
                        name: "OnOutputStreamReady",
                        params: &[Param { name: "aStream", ty: "*const nsIAsyncOutputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

