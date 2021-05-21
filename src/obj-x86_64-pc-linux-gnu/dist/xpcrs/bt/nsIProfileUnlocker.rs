//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/profile/nsIProfileUnlocker.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIProfileUnlocker",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void unlock (in unsigned long aSeverity); */
                    Method {
                        name: "Unlock",
                        params: &[Param { name: "aSeverity", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

