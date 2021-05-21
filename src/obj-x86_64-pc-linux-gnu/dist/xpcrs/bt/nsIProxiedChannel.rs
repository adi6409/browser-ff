//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIProxiedChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIProxiedChannel",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIProxyInfo proxyInfo; */
                    Method {
                        name: "GetProxyInfo",
                        params: &[Param { name: "aProxyInfo", ty: "*mut*const nsIProxyInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute int32_t httpProxyConnectResponseCode; */
                    Method {
                        name: "GetHttpProxyConnectResponseCode",
                        params: &[Param { name: "aHttpProxyConnectResponseCode", ty: "*mut int32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

