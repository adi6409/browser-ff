//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/commandlines/nsICommandLineHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICommandLineHandler",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void handle (in nsICommandLine aCommandLine); */
                    Method {
                        name: "Handle",
                        params: &[Param { name: "aCommandLine", ty: "*const nsICommandLine" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String helpInfo; */
                    Method {
                        name: "GetHelpInfo",
                        params: &[Param { name: "aHelpInfo", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

