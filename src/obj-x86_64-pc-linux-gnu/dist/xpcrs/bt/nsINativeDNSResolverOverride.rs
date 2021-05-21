//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/dns/nsINativeDNSResolverOverride.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINativeDNSResolverOverride",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void addIPOverride (in AUTF8String aHost, in ACString aIPLiteral); */
                    Method {
                        name: "AddIPOverride",
                        params: &[Param { name: "aHost", ty: "*const ::nsstring::nsACString" }, Param { name: "aIPLiteral", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setCnameOverride (in AUTF8String aHost, in ACString aCNAME); */
                    Method {
                        name: "SetCnameOverride",
                        params: &[Param { name: "aHost", ty: "*const ::nsstring::nsACString" }, Param { name: "aCNAME", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void clearHostOverride (in AUTF8String aHost); */
                    Method {
                        name: "ClearHostOverride",
                        params: &[Param { name: "aHost", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void clearOverrides (); */
                    Method {
                        name: "ClearOverrides",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

