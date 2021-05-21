//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/dns/nsIDNSResolverInfo.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDNSResolverInfo",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute ACString URL; */
                    Method {
                        name: "GetURL",
                        params: &[Param { name: "aURL", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

