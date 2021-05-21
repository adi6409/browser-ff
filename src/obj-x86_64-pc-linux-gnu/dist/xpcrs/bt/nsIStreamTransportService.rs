//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIStreamTransportService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStreamTransportService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsITransport createInputTransport (in nsIInputStream aStream, in boolean aCloseWhenDone); */
                    Method {
                        name: "CreateInputTransport",
                        params: &[Param { name: "aStream", ty: "*const nsIInputStream" }, Param { name: "aCloseWhenDone", ty: "bool" }, Param { name: "_retval", ty: "*mut*const nsITransport" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void InputAvailable (in nsIInputStream aStream, in nsIInputAvailableCallback aCallback); */
                    Method {
                        name: "InputAvailable",
                        params: &[Param { name: "aStream", ty: "*const nsIInputStream" }, Param { name: "aCallback", ty: "*const nsIInputAvailableCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIInputAvailableCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onInputAvailableComplete (in unsigned long long available, in nsresult available_return_code); */
                    Method {
                        name: "OnInputAvailableComplete",
                        params: &[Param { name: "available", ty: "u64" }, Param { name: "available_return_code", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

