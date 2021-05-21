//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsICacheInfoChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIInputStreamReceiver",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onInputStreamReady (in nsIInputStream aStream); */
                    Method {
                        name: "OnInputStreamReady",
                        params: &[Param { name: "aStream", ty: "*const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsICacheInfoChannel",
            base: Some("nsISupports"),
            methods: Err("nostdcall is unsupported"),
        },

        ]; D}

