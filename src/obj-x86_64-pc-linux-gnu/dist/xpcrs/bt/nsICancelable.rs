//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsICancelable.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICancelable",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void cancel (in nsresult aReason); */
                    Method {
                        name: "Cancel",
                        params: &[Param { name: "aReason", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

