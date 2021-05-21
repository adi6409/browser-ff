//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsITokenPasswordDialogs.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITokenPasswordDialogs",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] boolean setPassword (in nsIInterfaceRequestor ctx, in nsIPK11Token token); */
                    Method {
                        name: "SetPassword",
                        params: &[Param { name: "ctx", ty: "*const nsIInterfaceRequestor" }, Param { name: "token", ty: "*const nsIPK11Token" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

