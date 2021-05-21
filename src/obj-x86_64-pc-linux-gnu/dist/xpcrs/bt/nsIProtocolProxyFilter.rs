//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIProtocolProxyFilter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIProxyProtocolFilterResult",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onProxyFilterResult (in nsIProxyInfo aProxy); */
                    Method {
                        name: "OnProxyFilterResult",
                        params: &[Param { name: "aProxy", ty: "*const nsIProxyInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIProtocolProxyFilter",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void applyFilter (in nsIURI aURI, in nsIProxyInfo aProxy, in nsIProxyProtocolFilterResult aCallback); */
                    Method {
                        name: "ApplyFilter",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aProxy", ty: "*const nsIProxyInfo" }, Param { name: "aCallback", ty: "*const nsIProxyProtocolFilterResult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIProtocolProxyChannelFilter",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void applyFilter (in nsIChannel aChannel, in nsIProxyInfo aProxy, in nsIProxyProtocolFilterResult aCallback); */
                    Method {
                        name: "ApplyFilter",
                        params: &[Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "aProxy", ty: "*const nsIProxyInfo" }, Param { name: "aCallback", ty: "*const nsIProxyProtocolFilterResult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

