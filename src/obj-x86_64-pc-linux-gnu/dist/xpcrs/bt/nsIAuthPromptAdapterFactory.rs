//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIAuthPromptAdapterFactory.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAuthPromptAdapterFactory",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIAuthPrompt2 createAdapter (in nsIAuthPrompt aPrompt); */
                    Method {
                        name: "CreateAdapter",
                        params: &[Param { name: "aPrompt", ty: "*const nsIAuthPrompt" }, Param { name: "_retval", ty: "*mut*const nsIAuthPrompt2" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

