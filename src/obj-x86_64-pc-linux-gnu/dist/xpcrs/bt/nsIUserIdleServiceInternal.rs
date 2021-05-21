//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIUserIdleServiceInternal.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUserIdleServiceInternal",
            base: Some("nsIUserIdleService"),
            methods: Ok(&[
                    /* void resetIdleTimeOut (in unsigned long idleDeltaInMS); */
                    Method {
                        name: "ResetIdleTimeOut",
                        params: &[Param { name: "idleDeltaInMS", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

