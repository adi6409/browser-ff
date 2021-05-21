//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/security/nsIContentSecurityManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIContentSecurityManager",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIStreamListener performSecurityCheck (in nsIChannel aChannel, in nsIStreamListener aStreamListener); */
                    Method {
                        name: "PerformSecurityCheck",
                        params: &[Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "aStreamListener", ty: "*const nsIStreamListener" }, Param { name: "_retval", ty: "*mut*const nsIStreamListener" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

