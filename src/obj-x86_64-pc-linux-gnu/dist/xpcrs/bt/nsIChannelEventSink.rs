//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIChannelEventSink.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIChannelEventSink",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void asyncOnChannelRedirect (in nsIChannel oldChannel, in nsIChannel newChannel, in unsigned long flags, in nsIAsyncVerifyRedirectCallback callback); */
                    Method {
                        name: "AsyncOnChannelRedirect",
                        params: &[Param { name: "oldChannel", ty: "*const nsIChannel" }, Param { name: "newChannel", ty: "*const nsIChannel" }, Param { name: "flags", ty: "u32" }, Param { name: "callback", ty: "*const nsIAsyncVerifyRedirectCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

