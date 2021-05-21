//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIWebPageDescriptor.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebPageDescriptor",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void loadPageAsViewSource (in nsIDocShell otherDocShell, in AString aURL); */
                    Method {
                        name: "LoadPageAsViewSource",
                        params: &[Param { name: "otherDocShell", ty: "*const nsIDocShell" }, Param { name: "aURL", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsISupports currentDescriptor; */
                    Method {
                        name: "GetCurrentDescriptor",
                        params: &[Param { name: "aCurrentDescriptor", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

