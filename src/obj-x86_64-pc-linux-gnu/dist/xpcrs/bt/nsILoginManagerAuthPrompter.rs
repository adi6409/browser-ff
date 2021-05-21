//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/passwordmgr/nsILoginManagerAuthPrompter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsILoginManagerAuthPrompter",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void init (in nsIDOMWindow aWindow); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aWindow", ty: "*const nsIDOMWindow" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute Element browser; */
                    Method {
                        name: "GetBrowser",
                        params: &[Param { name: "aBrowser", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetBrowser",
                        params: &[Param { name: "aBrowser", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute Element openerBrowser; */
                    Method {
                        name: "GetOpenerBrowser",
                        params: &[Param { name: "aOpenerBrowser", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetOpenerBrowser",
                        params: &[Param { name: "aOpenerBrowser", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

