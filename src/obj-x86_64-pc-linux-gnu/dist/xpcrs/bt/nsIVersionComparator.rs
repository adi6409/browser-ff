//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIVersionComparator.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIVersionComparator",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* long compare (in ACString A, in ACString B); */
                    Method {
                        name: "Compare",
                        params: &[Param { name: "A", ty: "*const ::nsstring::nsACString" }, Param { name: "B", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

