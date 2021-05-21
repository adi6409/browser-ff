//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/cleardata/nsIClearDataService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIClearDataService",
            base: Some("nsISupports"),
            methods: Err("specialtype jsval unsupported"),
        },

        Interface {
            name: "nsIClearDataCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onDataDeleted (in uint32_t aFailedFlags); */
                    Method {
                        name: "OnDataDeleted",
                        params: &[Param { name: "aFailedFlags", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

