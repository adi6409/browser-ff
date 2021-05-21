//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIProtocolProxyCallback.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIProtocolProxyCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onProxyAvailable (in nsICancelable aRequest, in nsIChannel aChannel, in nsIProxyInfo aProxyInfo, in nsresult aStatus); */
                    Method {
                        name: "OnProxyAvailable",
                        params: &[Param { name: "aRequest", ty: "*const nsICancelable" }, Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "aProxyInfo", ty: "*const nsIProxyInfo" }, Param { name: "aStatus", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

