//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIInputStreamLength.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIInputStreamLength",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* long long length (); */
                    Method {
                        name: "Length",
                        params: &[Param { name: "_retval", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIAsyncInputStreamLength",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void asyncLengthWait (in nsIInputStreamLengthCallback aCallback, in nsIEventTarget aEventTarget); */
                    Method {
                        name: "AsyncLengthWait",
                        params: &[Param { name: "aCallback", ty: "*const nsIInputStreamLengthCallback" }, Param { name: "aEventTarget", ty: "*const nsIEventTarget" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIInputStreamLengthCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onInputStreamLengthReady (in nsIAsyncInputStreamLength aStream, in long long aLength); */
                    Method {
                        name: "OnInputStreamLengthReady",
                        params: &[Param { name: "aStream", ty: "*const nsIAsyncInputStreamLength" }, Param { name: "aLength", ty: "i64" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

