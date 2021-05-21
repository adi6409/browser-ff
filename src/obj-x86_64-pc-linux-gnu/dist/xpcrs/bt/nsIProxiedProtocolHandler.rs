//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIProxiedProtocolHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIProxiedProtocolHandler",
            base: Some("nsIProtocolHandler"),
            methods: Ok(&[
                    /* nsIChannel newProxiedChannel (in nsIURI uri, in nsIProxyInfo proxyInfo, in unsigned long proxyResolveFlags, in nsIURI proxyURI, in nsILoadInfo aLoadInfo); */
                    Method {
                        name: "NewProxiedChannel",
                        params: &[Param { name: "uri", ty: "*const nsIURI" }, Param { name: "proxyInfo", ty: "*const nsIProxyInfo" }, Param { name: "proxyResolveFlags", ty: "u32" }, Param { name: "proxyURI", ty: "*const nsIURI" }, Param { name: "aLoadInfo", ty: "*const nsILoadInfo" }, Param { name: "_retval", ty: "*mut*const nsIChannel" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

