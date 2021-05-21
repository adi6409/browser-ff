//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIIOService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIIOService",
            base: Some("nsISupports"),
            methods: Err("native type const mozilla::Maybe<mozilla::dom::ClientInfo> unsupported"),
        },

        Interface {
            name: "nsIIOServiceInternal",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void SetConnectivity (in boolean connectivity); */
                    Method {
                        name: "SetConnectivity",
                        params: &[Param { name: "connectivity", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void NotifyWakeup (); */
                    Method {
                        name: "NotifyWakeup",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

