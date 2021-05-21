//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsITokenDialogs.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITokenDialogs",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] void displayProtectedAuth (in nsIInterfaceRequestor ctx, in nsIProtectedAuthThread runnable); */
                    Method {
                        name: "DisplayProtectedAuth",
                        params: &[Param { name: "ctx", ty: "*const nsIInterfaceRequestor" }, Param { name: "runnable", ty: "*const nsIProtectedAuthThread" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

