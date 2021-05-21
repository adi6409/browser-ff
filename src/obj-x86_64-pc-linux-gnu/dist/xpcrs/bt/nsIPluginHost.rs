//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/plugins/base/nsIPluginHost.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIClearSiteDataCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void callback (in nsresult rv); */
                    Method {
                        name: "Callback",
                        params: &[Param { name: "rv", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPluginHost",
            base: Some("nsISupports"),
            methods: Err("specialtype jsval unsupported"),
        },

        ]; D}

