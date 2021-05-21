//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIAuthPromptCallback.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAuthPromptCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onAuthAvailable (in nsISupports aContext, in nsIAuthInformation aAuthInfo); */
                    Method {
                        name: "OnAuthAvailable",
                        params: &[Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "aAuthInfo", ty: "*const nsIAuthInformation" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onAuthCancelled (in nsISupports aContext, in boolean userCancel); */
                    Method {
                        name: "OnAuthCancelled",
                        params: &[Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "userCancel", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

