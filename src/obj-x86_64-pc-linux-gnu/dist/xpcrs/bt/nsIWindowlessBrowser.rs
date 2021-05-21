//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpfe/appshell/nsIWindowlessBrowser.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWindowlessBrowser",
            base: Some("nsIWebNavigation"),
            methods: Ok(&[
                    /* void close (); */
                    Method {
                        name: "Close",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIDocShell docShell; */
                    Method {
                        name: "GetDocShell",
                        params: &[Param { name: "aDocShell", ty: "*mut*const nsIDocShell" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute BrowsingContext browsingContext; */
                    Method {
                        name: "GetBrowsingContext",
                        params: &[Param { name: "aBrowsingContext", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

