//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/js/xpconnect/idl/xpcIJSGetFactory.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "xpcIJSGetFactory",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIFactory get (in nsCIDRef aCID); */
                    Method {
                        name: "Get",
                        params: &[Param { name: "aCID", ty: "*const nsCID" }, Param { name: "_retval", ty: "*mut*const nsIFactory" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

