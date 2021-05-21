//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/glean/xpcom/nsIGleanMetrics.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIGleanBoolean",
            base: Some("nsISupports"),
            methods: Err("specialtype jsval unsupported"),
        },

        Interface {
            name: "nsIGleanDatetime",
            base: Some("nsISupports"),
            methods: Err("optional_argc is unsupported"),
        },

        Interface {
            name: "nsIGleanCounter",
            base: Some("nsISupports"),
            methods: Err("specialtype jsval unsupported"),
        },

        Interface {
            name: "nsIGleanTimingDistribution",
            base: Some("nsISupports"),
            methods: Err("jscontext is unsupported"),
        },

        Interface {
            name: "nsIGleanMemoryDistribution",
            base: Some("nsISupports"),
            methods: Err("jscontext is unsupported"),
        },

        Interface {
            name: "nsIGleanPing",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void submit ([optional] in ACString aReason); */
                    Method {
                        name: "Submit",
                        params: &[Param { name: "aReason", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIGleanString",
            base: Some("nsISupports"),
            methods: Err("jscontext is unsupported"),
        },

        Interface {
            name: "nsIGleanStringList",
            base: Some("nsISupports"),
            methods: Err("jscontext is unsupported"),
        },

        Interface {
            name: "nsIGleanTimespan",
            base: Some("nsISupports"),
            methods: Err("specialtype jsval unsupported"),
        },

        Interface {
            name: "nsIGleanUuid",
            base: Some("nsISupports"),
            methods: Err("jscontext is unsupported"),
        },

        Interface {
            name: "nsIGleanEvent",
            base: Some("nsISupports"),
            methods: Err("specialtype jsval unsupported"),
        },

        ]; D}

