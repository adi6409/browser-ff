//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIAsyncVerifyRedirectCallback.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAsyncVerifyRedirectCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onRedirectVerifyCallback (in nsresult result); */
                    Method {
                        name: "OnRedirectVerifyCallback",
                        params: &[Param { name: "result", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

