//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIProtocolProxyService2.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIProtocolProxyService2",
            base: Some("nsIProtocolProxyService"),
            methods: Ok(&[
                    /* void reloadPAC (); */
                    Method {
                        name: "ReloadPAC",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* nsICancelable asyncResolve2 (in nsIChannel aChannel, in unsigned long aFlags, in nsIProtocolProxyCallback aCallback, [optional] in nsISerialEventTarget aMainThreadTarget); */
                    Method {
                        name: "AsyncResolve2",
                        params: &[Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "aFlags", ty: "u32" }, Param { name: "aCallback", ty: "*const nsIProtocolProxyCallback" }, Param { name: "aMainThreadTarget", ty: "*const nsISerialEventTarget" }, Param { name: "_retval", ty: "*mut*const nsICancelable" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

