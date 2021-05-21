//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIAsyncInputStream.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAsyncInputStream",
            base: Some("nsIInputStream"),
            methods: Ok(&[
                    /* void closeWithStatus (in nsresult aStatus); */
                    Method {
                        name: "CloseWithStatus",
                        params: &[Param { name: "aStatus", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void asyncWait (in nsIInputStreamCallback aCallback, in unsigned long aFlags, in unsigned long aRequestedCount, in nsIEventTarget aEventTarget); */
                    Method {
                        name: "AsyncWait",
                        params: &[Param { name: "aCallback", ty: "*const nsIInputStreamCallback" }, Param { name: "aFlags", ty: "u32" }, Param { name: "aRequestedCount", ty: "u32" }, Param { name: "aEventTarget", ty: "*const nsIEventTarget" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIInputStreamCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onInputStreamReady (in nsIAsyncInputStream aStream); */
                    Method {
                        name: "OnInputStreamReady",
                        params: &[Param { name: "aStream", ty: "*const nsIAsyncInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

