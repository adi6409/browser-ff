//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIStreamListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStreamListener",
            base: Some("nsIRequestObserver"),
            methods: Ok(&[
                    /* void onDataAvailable (in nsIRequest aRequest, in nsIInputStream aInputStream, in unsigned long long aOffset, in unsigned long aCount); */
                    Method {
                        name: "OnDataAvailable",
                        params: &[Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aInputStream", ty: "*const nsIInputStream" }, Param { name: "aOffset", ty: "u64" }, Param { name: "aCount", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

