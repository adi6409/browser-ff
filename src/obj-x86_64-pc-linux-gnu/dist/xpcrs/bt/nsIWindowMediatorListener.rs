//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpfe/appshell/nsIWindowMediatorListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWindowMediatorListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onOpenWindow (in nsIAppWindow window); */
                    Method {
                        name: "OnOpenWindow",
                        params: &[Param { name: "window", ty: "*const nsIAppWindow" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onCloseWindow (in nsIAppWindow window); */
                    Method {
                        name: "OnCloseWindow",
                        params: &[Param { name: "window", ty: "*const nsIAppWindow" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

