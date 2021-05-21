//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIConsoleListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIConsoleListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void observe (in nsIConsoleMessage aMessage); */
                    Method {
                        name: "Observe",
                        params: &[Param { name: "aMessage", ty: "*const nsIConsoleMessage" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

