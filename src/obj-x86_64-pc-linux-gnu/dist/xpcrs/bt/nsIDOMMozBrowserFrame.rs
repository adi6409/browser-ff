//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/html/nsIDOMMozBrowserFrame.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMMozBrowserFrame",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [infallible] attribute boolean mozbrowser; */
                    Method {
                        name: "GetMozbrowser",
                        params: &[Param { name: "aMozbrowser", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetMozbrowser",
                        params: &[Param { name: "aMozbrowser", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

