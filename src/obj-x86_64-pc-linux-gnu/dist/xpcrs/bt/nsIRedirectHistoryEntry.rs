//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIRedirectHistoryEntry.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRedirectHistoryEntry",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIPrincipal principal; */
                    Method {
                        name: "GetPrincipal",
                        params: &[Param { name: "aPrincipal", ty: "*mut*const nsIPrincipal" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIURI referrerURI; */
                    Method {
                        name: "GetReferrerURI",
                        params: &[Param { name: "aReferrerURI", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString remoteAddress; */
                    Method {
                        name: "GetRemoteAddress",
                        params: &[Param { name: "aRemoteAddress", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

