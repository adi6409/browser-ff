//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/res/nsISubstitutingProtocolHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISubstitutingProtocolHandler",
            base: Some("nsIProtocolHandler"),
            methods: Ok(&[
                    /* [must_use] void setSubstitution (in ACString root, in nsIURI baseURI); */
                    Method {
                        name: "SetSubstitution",
                        params: &[Param { name: "root", ty: "*const ::nsstring::nsACString" }, Param { name: "baseURI", ty: "*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void setSubstitutionWithFlags (in ACString root, in nsIURI baseURI, in uint32_t flags); */
                    Method {
                        name: "SetSubstitutionWithFlags",
                        params: &[Param { name: "root", ty: "*const ::nsstring::nsACString" }, Param { name: "baseURI", ty: "*const nsIURI" }, Param { name: "flags", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsIURI getSubstitution (in ACString root); */
                    Method {
                        name: "GetSubstitution",
                        params: &[Param { name: "root", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] boolean hasSubstitution (in ACString root); */
                    Method {
                        name: "HasSubstitution",
                        params: &[Param { name: "root", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] AUTF8String resolveURI (in nsIURI resURI); */
                    Method {
                        name: "ResolveURI",
                        params: &[Param { name: "resURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

