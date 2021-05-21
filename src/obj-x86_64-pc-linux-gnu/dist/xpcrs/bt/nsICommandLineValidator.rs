//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/commandlines/nsICommandLineValidator.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICommandLineValidator",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void validate (in nsICommandLine aCommandLine); */
                    Method {
                        name: "Validate",
                        params: &[Param { name: "aCommandLine", ty: "*const nsICommandLine" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

