//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsINestedURI.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINestedURI",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIURI innerURI; */
                    Method {
                        name: "GetInnerURI",
                        params: &[Param { name: "aInnerURI", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIURI innermostURI; */
                    Method {
                        name: "GetInnermostURI",
                        params: &[Param { name: "aInnermostURI", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsINestedURIMutator",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use,noscript] void init (in nsIURI innerURI); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "innerURI", ty: "*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsINestedAboutURIMutator",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use,noscript] void initWithBase (in nsIURI innerURI, in nsIURI baseURI); */
                    Method {
                        name: "InitWithBase",
                        params: &[Param { name: "innerURI", ty: "*const nsIURI" }, Param { name: "baseURI", ty: "*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIJSURIMutator",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use,noscript] void setBase (in nsIURI aBaseURI); */
                    Method {
                        name: "SetBase",
                        params: &[Param { name: "aBaseURI", ty: "*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

