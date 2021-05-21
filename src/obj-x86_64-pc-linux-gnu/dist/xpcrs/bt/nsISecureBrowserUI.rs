//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISecureBrowserUI.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISecureBrowserUI",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute unsigned long state; */
                    Method {
                        name: "GetState",
                        params: &[Param { name: "aState", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute bool isSecureContext; */
                    Method {
                        name: "GetIsSecureContext",
                        params: &[Param { name: "aIsSecureContext", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsITransportSecurityInfo secInfo; */
                    Method {
                        name: "GetSecInfo",
                        params: &[Param { name: "aSecInfo", ty: "*mut*const nsITransportSecurityInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

