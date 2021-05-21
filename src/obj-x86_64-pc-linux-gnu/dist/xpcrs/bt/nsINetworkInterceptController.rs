//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsINetworkInterceptController.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIInterceptedBodyCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void bodyComplete (in nsresult aRv); */
                    Method {
                        name: "BodyComplete",
                        params: &[Param { name: "aRv", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIInterceptedChannel",
            base: Some("nsISupports"),
            methods: Err("native type mozilla::dom::ChannelInfo unsupported"),
        },

        Interface {
            name: "nsINetworkInterceptController",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* bool shouldPrepareForIntercept (in nsIURI aURI, in nsIChannel aChannel); */
                    Method {
                        name: "ShouldPrepareForIntercept",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void channelIntercepted (in nsIInterceptedChannel aChannel); */
                    Method {
                        name: "ChannelIntercepted",
                        params: &[Param { name: "aChannel", ty: "*const nsIInterceptedChannel" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

